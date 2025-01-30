// Copyright 2024 Digitrans Inc
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use bigbytes_common_catalog::table::Table;
use bigbytes_common_catalog::table::TableExt;
use bigbytes_common_exception::ErrorCode;
use bigbytes_common_exception::Result;
use bigbytes_common_expression::ComputedExpr;
use bigbytes_common_license::license::Feature::ComputedColumn;
use bigbytes_common_license::license_manager::LicenseManagerSwitch;
use bigbytes_common_meta_app::schema::DatabaseType;
use bigbytes_common_meta_app::schema::TableMeta;
use bigbytes_common_meta_app::schema::UpdateTableMetaReq;
use bigbytes_common_meta_types::MatchSeq;
use bigbytes_common_sql::field_default_value;
use bigbytes_common_sql::plans::AddColumnOption;
use bigbytes_common_sql::plans::AddTableColumnPlan;
use bigbytes_common_sql::plans::Mutation;
use bigbytes_common_sql::plans::Plan;
use bigbytes_common_sql::Planner;
use bigbytes_common_storages_fuse::FuseTable;
use bigbytes_common_storages_stream::stream_table::STREAM_ENGINE;
use bigbytes_common_storages_view::view_table::VIEW_ENGINE;
use bigbytes_storages_common_table_meta::meta::TableSnapshot;
use bigbytes_storages_common_table_meta::meta::Versioned;
use bigbytes_storages_common_table_meta::table::OPT_KEY_SNAPSHOT_LOCATION;
use log::info;

use crate::interpreters::interpreter_table_create::is_valid_column;
use crate::interpreters::interpreter_table_modify_column::build_select_insert_plan;
use crate::interpreters::Interpreter;
use crate::interpreters::MutationInterpreter;
use crate::pipelines::PipelineBuildResult;
use crate::sessions::QueryContext;
use crate::sessions::TableContext;

pub struct AddTableColumnInterpreter {
    ctx: Arc<QueryContext>,
    plan: AddTableColumnPlan,
}

impl AddTableColumnInterpreter {
    pub fn try_create(ctx: Arc<QueryContext>, plan: AddTableColumnPlan) -> Result<Self> {
        Ok(AddTableColumnInterpreter { ctx, plan })
    }
}

#[async_trait::async_trait]
impl Interpreter for AddTableColumnInterpreter {
    fn name(&self) -> &str {
        "AddTableColumnInterpreter"
    }

    fn is_ddl(&self) -> bool {
        self.plan.is_deterministic
    }

    #[async_backtrace::framed]
    async fn execute2(&self) -> Result<PipelineBuildResult> {
        let catalog_name = self.plan.catalog.as_str();
        let db_name = self.plan.database.as_str();
        let tbl_name = self.plan.table.as_str();

        let tbl = self.ctx.get_table(catalog_name, db_name, tbl_name).await?;
        // check mutability
        tbl.check_mutable()?;

        let mut table_info = tbl.get_table_info().clone();
        let engine = table_info.engine();
        if matches!(engine, VIEW_ENGINE | STREAM_ENGINE) {
            return Err(ErrorCode::TableEngineNotSupported(format!(
                "{}.{} engine is {} that doesn't support alter",
                &self.plan.database, &self.plan.table, engine
            )));
        }
        if table_info.db_type != DatabaseType::NormalDB {
            return Err(ErrorCode::TableEngineNotSupported(format!(
                "{}.{} doesn't support alter",
                &self.plan.database, &self.plan.table
            )));
        }

        let catalog = self.ctx.get_catalog(catalog_name).await?;
        let field = self.plan.field.clone();
        if field.computed_expr().is_some() {
            LicenseManagerSwitch::instance()
                .check_enterprise_enabled(self.ctx.get_license_key(), ComputedColumn)?;
        }

        if field.default_expr().is_some() {
            let _ = field_default_value(self.ctx.clone(), &field)?;
        }
        is_valid_column(field.name())?;
        let index = match &self.plan.option {
            AddColumnOption::First => 0,
            AddColumnOption::After(name) => table_info.meta.schema.index_of(name)? + 1,
            AddColumnOption::End => table_info.meta.schema.num_fields(),
        };
        table_info
            .meta
            .add_column(&field, &self.plan.comment, index)?;

        // if the new column is a stored computed field,
        // need rebuild the table to generate stored computed column.
        if let Some(ComputedExpr::Stored(_)) = field.computed_expr {
            let fuse_table = FuseTable::try_from_table(tbl.as_ref())?;
            let prev_snapshot_id = fuse_table
                .read_table_snapshot()
                .await
                .map_or(None, |v| v.map(|snapshot| snapshot.snapshot_id));

            // computed columns will generated from other columns.
            let new_schema = table_info.meta.schema.remove_computed_fields();
            let query_fields = new_schema
                .fields()
                .iter()
                .map(|field| format!("`{}`", field.name))
                .collect::<Vec<_>>()
                .join(", ");

            let sql = format!(
                "SELECT {} FROM `{}`.`{}`",
                query_fields, self.plan.database, self.plan.table
            );

            return build_select_insert_plan(
                self.ctx.clone(),
                sql,
                table_info.clone(),
                new_schema.into(),
                prev_snapshot_id,
            )
            .await;
        }

        let mut new_table_meta = table_info.meta.clone();
        let _ = generate_new_snapshot(self.ctx.as_ref(), tbl.as_ref(), &mut new_table_meta).await?;
        let table_id = table_info.ident.table_id;
        let table_version = table_info.ident.seq;

        let req = UpdateTableMetaReq {
            table_id,
            seq: MatchSeq::Exact(table_version),
            new_table_meta,
        };

        let _resp = catalog.update_single_table_meta(req, &table_info).await?;

        // If the column is not deterministic, update to refresh the value with default expr.
        if !self.plan.is_deterministic {
            self.ctx
                .evict_table_from_cache(catalog_name, db_name, tbl_name)?;
            let query = format!(
                "UPDATE `{}`.`{}` SET `{}` = {};",
                db_name,
                tbl_name,
                field.name(),
                field.default_expr().unwrap()
            );
            let mut planner = Planner::new(self.ctx.clone());
            let (plan, _) = planner.plan_sql(&query).await?;
            if let Plan::DataMutation { s_expr, schema, .. } = plan {
                let mutation: Mutation = s_expr.plan().clone().try_into()?;
                let interpreter = MutationInterpreter::try_create(
                    self.ctx.clone(),
                    *s_expr,
                    schema,
                    mutation.metadata.clone(),
                )?;
                let _ = interpreter.execute(self.ctx.clone()).await?;
                return Ok(PipelineBuildResult::create());
            }
        }
        Ok(PipelineBuildResult::create())
    }
}

pub(crate) async fn generate_new_snapshot(
    ctx: &QueryContext,
    table: &dyn Table,
    new_table_meta: &mut TableMeta,
) -> Result<()> {
    if let Ok(fuse_table) = FuseTable::try_from_table(table) {
        if let Some(snapshot) = fuse_table.read_table_snapshot().await? {
            let mut new_snapshot = TableSnapshot::from_previous(
                snapshot.as_ref(),
                Some(fuse_table.get_table_info().ident.seq),
            );

            // replace schema
            new_snapshot.schema = new_table_meta.schema.as_ref().clone();

            // write down new snapshot
            let new_snapshot_location = fuse_table
                .meta_location_generator()
                .snapshot_location_from_uuid(&new_snapshot.snapshot_id, TableSnapshot::VERSION)?;

            let data = new_snapshot.to_bytes()?;
            fuse_table
                .get_operator_ref()
                .write(&new_snapshot_location, data)
                .await?;

            // write down hint
            FuseTable::write_last_snapshot_hint(
                ctx,
                fuse_table.get_operator_ref(),
                fuse_table.meta_location_generator(),
                &new_snapshot_location,
            )
            .await;

            new_table_meta
                .options
                .insert(OPT_KEY_SNAPSHOT_LOCATION.to_owned(), new_snapshot_location);
        } else {
            info!("Snapshot not found, no need to generate new snapshot");
        }
    } else {
        info!("Not a fuse table, no need to generate new snapshot");
    }
    Ok(())
}
