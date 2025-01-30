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

use chrono_tz::Tz;
use bigbytesdb_common_ast::ast::SetType;
use bigbytesdb_common_config::GlobalConfig;
use bigbytesdb_common_exception::ErrorCode;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_expression::cast_scalar;
use bigbytesdb_common_expression::types::DataType;
use bigbytesdb_common_expression::DataBlock;
use bigbytesdb_common_expression::Scalar;
use bigbytesdb_common_functions::BUILTIN_FUNCTIONS;
use bigbytesdb_common_meta_app::tenant::Tenant;
use bigbytesdb_common_sql::plans::Plan;
use bigbytesdb_common_sql::plans::SetPlan;
use bigbytesdb_common_sql::plans::SetScalarsOrQuery;
use bigbytesdb_common_users::UserApiProvider;
use fastrace::func_name;
use futures::TryStreamExt;

use super::SelectInterpreter;
use crate::interpreters::Interpreter;
use crate::pipelines::PipelineBuildResult;
use crate::sessions::QueryAffect;
use crate::sessions::QueryContext;
use crate::sessions::TableContext;

pub struct SetInterpreter {
    ctx: Arc<QueryContext>,
    set: SetPlan,
}

impl SetInterpreter {
    pub fn try_create(ctx: Arc<QueryContext>, set: SetPlan) -> Result<Self> {
        Ok(SetInterpreter { ctx, set })
    }

    async fn set_settings(&self, var: String, value: String, is_global: bool) -> Result<()> {
        let settings = self.ctx.get_session_settings();

        match is_global {
            true => settings.set_global_setting(var, value).await,
            false => settings.set_setting(var, value),
        }
    }

    #[async_backtrace::framed]
    async fn execute_settings(&self, scalars: Vec<Scalar>, is_global: bool) -> Result<()> {
        let scalars: Vec<Scalar> = scalars
            .into_iter()
            .map(|scalar| cast_scalar(None, scalar.clone(), DataType::String, &BUILTIN_FUNCTIONS))
            .collect::<Result<Vec<_>>>()?;

        let mut keys: Vec<String> = vec![];
        let mut values: Vec<String> = vec![];
        let mut is_globals = vec![];

        for (var, scalar) in self.set.idents.iter().zip(scalars.into_iter()) {
            let scalar = scalar.as_string().unwrap();
            let ok = match var.to_lowercase().as_str() {
                // To be compatible with some drivers
                "sql_mode" | "autocommit" => false,
                "timezone" => {
                    // check if the timezone is valid
                    let tz = scalar.trim_matches(|c| c == '\'' || c == '\"');
                    let _ = tz.parse::<Tz>().map_err(|_| {
                        ErrorCode::InvalidTimezone(format!("Invalid Timezone: {}", scalar))
                    })?;
                    self.set_settings(var.to_string(), tz.to_string(), is_global)
                        .await?;
                    true
                }
                "network_policy" => {
                    // check if the network policy exists
                    let tenant = self.ctx.get_tenant();
                    let _ = UserApiProvider::instance()
                        .get_network_policy(&tenant, scalar)
                        .await?;
                    self.set_settings(var.to_string(), scalar.clone(), is_global)
                        .await?;
                    true
                }
                // TODO: if account_admin is built-in meta in future, we need process set sandbox_tenant in there.
                // Like: https://github.com/getbigbytes/bigbytesdb/pull/14451/files#diff-a26c9dfc9c0a37f5efa19e2b16006732b9023f42ee47cbe37fe461fb46b9dfc0R82-R85
                "sandbox_tenant" => {
                    // only used in sqlogictest, it will create a sandbox tenant on every sqlogictest cases
                    // and switch to it by SET sandbox_tenant = xxx;
                    let config = GlobalConfig::instance();
                    let tenant = scalar.clone();
                    if config.query.internal_enable_sandbox_tenant && !tenant.is_empty() {
                        UserApiProvider::try_create_simple(
                            config.meta.to_meta_grpc_client_conf(),
                            &Tenant::new_or_err(tenant, func_name!())?,
                        )
                        .await?;
                    }

                    self.set_settings(var.to_string(), scalar.clone(), is_global)
                        .await?;
                    true
                }
                _ => {
                    self.set_settings(var.to_string(), scalar.clone(), is_global)
                        .await?;
                    true
                }
            };
            if ok {
                keys.push(var.to_string());
                values.push(scalar.clone());
                is_globals.push(is_global);
            }
        }
        self.ctx.set_affect(QueryAffect::ChangeSettings {
            keys,
            values,
            is_globals,
        });
        Ok(())
    }

    #[async_backtrace::framed]
    async fn execute_variables(&self, scalars: Vec<Scalar>) -> Result<()> {
        for (id, scalar) in (self.set.idents.iter()).zip(scalars.into_iter()) {
            self.ctx.set_variable(id.clone(), scalar);
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl Interpreter for SetInterpreter {
    fn name(&self) -> &str {
        "SetInterpreter"
    }

    fn is_ddl(&self) -> bool {
        false
    }

    #[async_backtrace::framed]
    async fn execute2(&self) -> Result<PipelineBuildResult> {
        let scalars = match &self.set.values {
            SetScalarsOrQuery::VarValue(scalars) => scalars.clone(),
            SetScalarsOrQuery::Query(query) => {
                let (s_expr, metadata, bind_context, formatted_ast) = match query.as_ref() {
                    Plan::Query {
                        s_expr,
                        metadata,
                        bind_context,
                        formatted_ast,
                        ..
                    } => (s_expr, metadata, bind_context, formatted_ast),
                    v => unreachable!("Input plan must be Query, but it's {}", v),
                };

                let select_interpreter = SelectInterpreter::try_create(
                    self.ctx.clone(),
                    *(bind_context.clone()),
                    *s_expr.clone(),
                    metadata.clone(),
                    formatted_ast.clone(),
                    false,
                )?;

                let stream = select_interpreter.execute(self.ctx.clone()).await?;
                let datablocks: Vec<DataBlock> = stream.try_collect::<Vec<_>>().await?;
                let datablock = DataBlock::concat(&datablocks)?;

                if datablock.num_rows() != 1 {
                    return Err(ErrorCode::BadArguments(format!(
                        "Expect scalar result in set query result, but got {} rows",
                        datablock.num_rows()
                    )));
                }
                if datablock.num_columns() != self.set.idents.len() {
                    return Err(ErrorCode::BadArguments(format!(
                        "Expect {} column in set query result, but got {} columns",
                        self.set.idents.len(),
                        datablock.num_columns()
                    )));
                }
                datablock
                    .columns()
                    .iter()
                    .map(|c| c.value.index(0).unwrap().to_owned())
                    .collect()
            }
        };

        if scalars.len() != self.set.idents.len() {
            return Err(ErrorCode::BadArguments(format!(
                "Expect {} values in set statement, but got {}",
                self.set.idents.len(),
                scalars.len()
            )));
        }

        match &self.set.set_type {
            SetType::SettingsGlobal => self.execute_settings(scalars, true).await?,
            SetType::SettingsSession => self.execute_settings(scalars, false).await?,
            SetType::Variable => self.execute_variables(scalars).await?,
            SetType::SettingsQuery => {
                return Err(ErrorCode::BadArguments(
                    "Query level setting can not be set",
                ));
            }
        }

        Ok(PipelineBuildResult::create())
    }
}
