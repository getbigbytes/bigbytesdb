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

use bigbytesdb_common_catalog::catalog::Catalog;
use bigbytesdb_common_catalog::catalog::CatalogManager;
use bigbytesdb_common_catalog::database::Database;
use bigbytesdb_common_catalog::plan::PushDownInfo;
use bigbytesdb_common_catalog::table::Table;
use bigbytesdb_common_catalog::table_context::TableContext;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_expression::types::NumberDataType;
use bigbytesdb_common_expression::types::StringType;
use bigbytesdb_common_expression::types::TimestampType;
use bigbytesdb_common_expression::types::UInt64Type;
use bigbytesdb_common_expression::utils::FromData;
use bigbytesdb_common_expression::DataBlock;
use bigbytesdb_common_expression::TableDataType;
use bigbytesdb_common_expression::TableField;
use bigbytesdb_common_expression::TableSchemaRefExt;
use bigbytesdb_common_meta_app::principal::OwnershipObject;
use bigbytesdb_common_meta_app::schema::database_name_ident::DatabaseNameIdent;
use bigbytesdb_common_meta_app::schema::TableIdent;
use bigbytesdb_common_meta_app::schema::TableInfo;
use bigbytesdb_common_meta_app::schema::TableMeta;
use bigbytesdb_common_meta_app::tenant::Tenant;
use bigbytesdb_common_users::UserApiProvider;
use log::warn;

use crate::table::AsyncOneBlockSystemTable;
use crate::table::AsyncSystemTable;

pub type DatabasesTableWithHistory = DatabasesTable<true>;
pub type DatabasesTableWithoutHistory = DatabasesTable<false>;

pub struct DatabasesTable<const WITH_HISTORY: bool> {
    table_info: TableInfo,
}

#[async_trait::async_trait]
pub trait HistoryAware {
    const TABLE_NAME: &'static str;
    async fn list_databases(
        catalog: &Arc<dyn Catalog>,
        tenant: &Tenant,
        with_history: bool,
    ) -> Result<Vec<Arc<dyn Database>>>;
}

macro_rules! impl_history_aware {
    ($with_history:expr, $table_name:expr) => {
        #[async_trait::async_trait]
        impl HistoryAware for DatabasesTable<$with_history> {
            const TABLE_NAME: &'static str = $table_name;

            #[async_backtrace::framed]
            async fn list_databases(
                catalog: &Arc<dyn Catalog>,
                tenant: &Tenant,
                with_history: bool,
            ) -> Result<Vec<Arc<dyn Database>>> {
                if with_history {
                    catalog.list_databases_history(tenant).await
                } else {
                    catalog.list_databases(tenant).await
                }
            }
        }
    };
}

impl_history_aware!(true, "databases_with_history");
impl_history_aware!(false, "databases");

#[async_trait::async_trait]
impl<const WITH_HISTORY: bool> AsyncSystemTable for DatabasesTable<WITH_HISTORY>
where DatabasesTable<WITH_HISTORY>: HistoryAware
{
    const NAME: &'static str = Self::TABLE_NAME;

    fn get_table_info(&self) -> &TableInfo {
        &self.table_info
    }

    #[async_backtrace::framed]
    async fn get_full_data(
        &self,
        ctx: Arc<dyn TableContext>,
        _push_downs: Option<PushDownInfo>,
    ) -> Result<DataBlock> {
        let tenant = ctx.get_tenant();

        let catalogs = CatalogManager::instance();
        let catalogs: Vec<(String, Arc<dyn Catalog>)> = catalogs
            .list_catalogs(&tenant, ctx.session_state())
            .await?
            .iter()
            .map(|e| (e.name(), e.clone()))
            .collect();

        let user_api = UserApiProvider::instance();
        let mut catalog_names = vec![];
        let mut db_names = vec![];
        let mut db_ids = vec![];
        let mut owners: Vec<Option<String>> = vec![];
        let mut dropped_on: Vec<Option<i64>> = vec![];

        let visibility_checker = ctx.get_visibility_checker(false).await?;
        let catalog_dbs = visibility_checker.get_visibility_database();
        // None means has global level privileges
        if let Some(catalog_dbs) = catalog_dbs {
            if WITH_HISTORY {
                for (ctl_name, dbs) in catalog_dbs {
                    let catalog = ctx.get_catalog(ctl_name).await?;
                    let dbs_history = catalog.list_databases_history(&tenant).await?;
                    for db_history in dbs_history {
                        let db_name = db_history
                            .get_db_info()
                            .name_ident
                            .database_name()
                            .to_string();
                        let id = db_history.get_db_info().database_id.db_id;
                        if db_ids.contains(&id) {
                            continue;
                        }
                        if dbs.contains(&(None, Some(&id)))
                            || db_name.to_lowercase() == "information_schema"
                            || db_name.to_lowercase() == "system"
                        {
                            catalog_names.push(ctl_name.clone());
                            db_names.push(db_name);
                            db_ids.push(id);
                            owners.push(
                                user_api
                                    .get_ownership(&tenant, &OwnershipObject::Database {
                                        catalog_name: ctl_name.to_string(),
                                        db_id: id,
                                    })
                                    .await
                                    .ok()
                                    .and_then(|ownership| ownership.map(|o| o.role.clone())),
                            );
                            dropped_on.push(
                                db_history
                                    .get_db_info()
                                    .meta
                                    .drop_on
                                    .map(|v| v.timestamp_micros()),
                            );
                        }
                    }
                }
            } else {
                for (catalog, dbs) in catalog_dbs {
                    let mut catalog_db_ids = vec![];
                    let mut catalog_db_names = vec![];
                    let ctl = ctx.get_catalog(catalog).await?;
                    catalog_db_names.extend(
                        dbs.iter()
                            .filter_map(|(db_name, _)| *db_name)
                            .map(|db_name| db_name.to_string()),
                    );
                    catalog_db_ids.extend(dbs.iter().filter_map(|(_, db_id)| *db_id));

                    if let Ok(databases) = ctl
                        .mget_database_names_by_ids(&tenant, &catalog_db_ids)
                        .await
                    {
                        catalog_db_names.extend(databases.into_iter().flatten());
                    } else {
                        let msg = format!("Failed to get database name by id: {}", ctl.name());
                        warn!("{}", msg);
                    }
                    let db_idents = catalog_db_names
                        .iter()
                        .map(|name| DatabaseNameIdent::new(&tenant, name))
                        .collect::<Vec<DatabaseNameIdent>>();
                    let dbs = ctl.mget_databases(&tenant, &db_idents).await?;

                    for db in dbs {
                        let db_id = db.get_db_info().database_id.db_id;
                        if db_ids.contains(&db_id) {
                            continue;
                        }
                        catalog_names.push(catalog.clone());
                        db_names.push(db.get_db_info().name_ident.database_name().to_string());
                        db_ids.push(db_id);
                        owners.push(
                            user_api
                                .get_ownership(&tenant, &OwnershipObject::Database {
                                    catalog_name: catalog.to_string(),
                                    db_id,
                                })
                                .await
                                .ok()
                                .and_then(|ownership| ownership.map(|o| o.role.clone())),
                        );
                        dropped_on
                            .push(db.get_db_info().meta.drop_on.map(|v| v.timestamp_micros()));
                    }
                }
            }
        } else {
            for (ctl_name, catalog) in catalogs.into_iter() {
                let databases = Self::list_databases(&catalog, &tenant, WITH_HISTORY).await?;
                let final_dbs = databases
                    .into_iter()
                    .filter(|db| {
                        visibility_checker.check_database_visibility(
                            &ctl_name,
                            db.name(),
                            db.get_db_info().database_id.db_id,
                        )
                    })
                    .collect::<Vec<_>>();

                for db in final_dbs {
                    catalog_names.push(ctl_name.clone());
                    let db_name = db.name().to_string();
                    db_names.push(db_name);
                    let id = db.get_db_info().database_id.db_id;
                    db_ids.push(id);
                    owners.push(
                        user_api
                            .get_ownership(&tenant, &OwnershipObject::Database {
                                catalog_name: ctl_name.to_string(),
                                db_id: id,
                            })
                            .await
                            .ok()
                            .and_then(|ownership| ownership.map(|o| o.role.clone())),
                    );
                    dropped_on.push(db.get_db_info().meta.drop_on.map(|v| v.timestamp_micros()));
                }
            }
        }

        Ok(DataBlock::new_from_columns(vec![
            StringType::from_data(catalog_names),
            StringType::from_data(db_names),
            UInt64Type::from_data(db_ids),
            StringType::from_opt_data(owners),
            TimestampType::from_opt_data(dropped_on),
        ]))
    }
}

impl<const WITH_HISTORY: bool> DatabasesTable<WITH_HISTORY>
where DatabasesTable<WITH_HISTORY>: HistoryAware
{
    pub fn create(table_id: u64) -> Arc<dyn Table> {
        let schema = TableSchemaRefExt::create(vec![
            TableField::new("catalog", TableDataType::String),
            TableField::new("name", TableDataType::String),
            TableField::new("database_id", TableDataType::Number(NumberDataType::UInt64)),
            TableField::new(
                "owner",
                TableDataType::Nullable(Box::from(TableDataType::String)),
            ),
            TableField::new(
                "dropped_on",
                TableDataType::Nullable(Box::new(TableDataType::Timestamp)),
            ),
        ]);

        let name = Self::TABLE_NAME;
        let table_info = TableInfo {
            desc: format!("'system'.'{name}'"),
            name: Self::NAME.to_owned(),
            ident: TableIdent::new(table_id, 0),
            meta: TableMeta {
                schema,
                engine: "SystemDatabases".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        AsyncOneBlockSystemTable::create(DatabasesTable { table_info })
    }
}
