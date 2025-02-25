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

use std::any::Any;
use std::sync::Arc;

use chrono::DateTime;
use bigbytesdb_common_catalog::catalog::CATALOG_DEFAULT;
use bigbytesdb_common_catalog::plan::DataSourcePlan;
use bigbytesdb_common_catalog::plan::PartStatistics;
use bigbytesdb_common_catalog::plan::Partitions;
use bigbytesdb_common_catalog::plan::PushDownInfo;
use bigbytesdb_common_catalog::table_args::TableArgs;
use bigbytesdb_common_catalog::table_function::TableFunction;
use bigbytesdb_common_config::GlobalConfig;
use bigbytesdb_common_exception::ErrorCode;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_expression::types::StringType;
use bigbytesdb_common_expression::DataBlock;
use bigbytesdb_common_expression::FromData;
use bigbytesdb_common_expression::TableDataType;
use bigbytesdb_common_expression::TableField;
use bigbytesdb_common_expression::TableSchema;
use bigbytesdb_common_meta_app::schema::TableIdent;
use bigbytesdb_common_meta_app::schema::TableInfo;
use bigbytesdb_common_meta_app::schema::TableMeta;
use bigbytesdb_common_openai::OpenAI;
use bigbytesdb_common_pipeline_core::processors::OutputPort;
use bigbytesdb_common_pipeline_core::processors::ProcessorPtr;
use bigbytesdb_common_pipeline_core::Pipeline;
use bigbytesdb_common_pipeline_sources::AsyncSource;
use bigbytesdb_common_pipeline_sources::AsyncSourcer;
use bigbytesdb_common_storages_factory::Table;
use bigbytesdb_common_storages_fuse::table_functions::string_literal;
use bigbytesdb_common_storages_fuse::TableContext;
use bigbytesdb_common_storages_stream::stream_table::STREAM_ENGINE;
use bigbytesdb_common_storages_view::view_table::VIEW_ENGINE;
use log::info;

pub struct GPT2SQLTable {
    prompt: String,
    table_info: TableInfo,
}

impl GPT2SQLTable {
    pub fn create(
        database_name: &str,
        table_func_name: &str,
        table_id: u64,
        table_args: TableArgs,
    ) -> Result<Arc<dyn TableFunction>> {
        // Check args.
        let args = table_args.expect_all_positioned(table_func_name, Some(1))?;
        let prompt = args[0]
            .clone()
            .into_string()
            .map_err(|_| ErrorCode::BadArguments("Expected string argument."))?;

        let schema = TableSchema::new(vec![
            TableField::new("database", TableDataType::String),
            TableField::new("generated_sql", TableDataType::String),
        ]);
        let table_info = TableInfo {
            ident: TableIdent::new(table_id, 0),
            desc: format!("'{}'.'{}'", database_name, table_func_name),
            name: String::from(table_func_name),
            meta: TableMeta {
                schema: Arc::new(schema),
                engine: String::from(table_func_name),
                // Assuming that created_on is unnecessary for function table,
                // we could make created_on fixed to pass test_shuffle_action_try_into.
                created_on: DateTime::from_timestamp(0, 0).unwrap(),
                updated_on: DateTime::from_timestamp(0, 0).unwrap(),
                ..Default::default()
            },
            ..Default::default()
        };

        Ok(Arc::new(GPT2SQLTable { prompt, table_info }))
    }
}

impl TableFunction for GPT2SQLTable {
    fn function_name(&self) -> &str {
        self.name()
    }

    fn as_table<'a>(self: Arc<Self>) -> Arc<dyn Table + 'a>
    where Self: 'a {
        self
    }
}

#[async_trait::async_trait]
impl Table for GPT2SQLTable {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_table_info(&self) -> &TableInfo {
        &self.table_info
    }

    #[async_backtrace::framed]
    async fn read_partitions(
        &self,
        _ctx: Arc<dyn TableContext>,
        _push_downs: Option<PushDownInfo>,
        _dry_run: bool,
    ) -> Result<(PartStatistics, Partitions)> {
        // dummy statistics
        Ok((PartStatistics::default_exact(), Partitions::default()))
    }

    fn table_args(&self) -> Option<TableArgs> {
        Some(TableArgs::new_positioned(vec![string_literal(
            self.prompt.as_str(),
        )]))
    }

    fn read_data(
        &self,
        ctx: Arc<dyn TableContext>,
        _plan: &DataSourcePlan,
        pipeline: &mut Pipeline,
        _put_cache: bool,
    ) -> Result<()> {
        pipeline.add_source(
            |output| GPT2SQLSource::create(ctx.clone(), output, self.prompt.clone()),
            1,
        )?;
        Ok(())
    }
}

struct GPT2SQLSource {
    ctx: Arc<dyn TableContext>,
    prompt: String,
    finished: bool,
}

impl GPT2SQLSource {
    pub fn create(
        ctx: Arc<dyn TableContext>,
        output: Arc<OutputPort>,
        prompt: String,
    ) -> Result<ProcessorPtr> {
        AsyncSourcer::create(ctx.clone(), output, GPT2SQLSource {
            prompt,
            ctx,
            finished: false,
        })
    }
}

#[async_trait::async_trait]
impl AsyncSource for GPT2SQLSource {
    const NAME: &'static str = "gpt_to_sql";

    #[async_backtrace::framed]
    async fn generate(&mut self) -> Result<Option<DataBlock>> {
        if self.finished {
            return Ok(None);
        }

        // ### Postgres SQL tables, with their properties:
        // #
        // # Employee(id, name, department_id)
        // # Department(id, name, address)
        // # Salary_Payments(id, employee_id, amount, date)
        // #
        // ### A query to list the names of the departments which employed more than 10 employees in the last 3 months
        // SELECT
        let database = self.ctx.get_current_database();
        let tenant = self.ctx.get_tenant();

        // Disable table info refreshing.
        // Attached tables may not be able to refresh table info successfully.
        let catalog = self
            .ctx
            .get_catalog(CATALOG_DEFAULT)
            .await?
            .disable_table_info_refresh()?;

        let mut template = vec![];
        template.push("### Postgres SQL tables, with their properties:".to_string());
        template.push("#".to_string());

        for table in catalog.list_tables(&tenant, &database).await? {
            let fields = if matches!(table.engine(), VIEW_ENGINE | STREAM_ENGINE) {
                continue;
            } else {
                table.schema().fields().clone()
            };

            let columns_name = fields
                .iter()
                .map(|f| f.name().to_string())
                .collect::<Vec<_>>();
            template.push(format!("{}({})", table.name(), columns_name.join(",")));
        }
        template.push("#".to_string());
        template.push(format!("### {}", self.prompt.clone()));
        template.push("#".to_string());
        template.push("SELECT".to_string());

        let prompt = template.join("");
        info!("openai request prompt: {}", prompt);

        // Response.
        let api_base = GlobalConfig::instance()
            .query
            .openai_api_chat_base_url
            .clone();
        let api_key = GlobalConfig::instance().query.openai_api_key.clone();
        let api_version = GlobalConfig::instance().query.openai_api_version.clone();
        let api_embedding_model = GlobalConfig::instance()
            .query
            .openai_api_embedding_model
            .clone();
        let api_completion_model = GlobalConfig::instance()
            .query
            .openai_api_completion_model
            .clone();
        let openai = OpenAI::create(
            api_base,
            api_key,
            api_version,
            api_embedding_model,
            api_completion_model,
        );

        info!("openai info: {:?}", openai);

        let (sql, _) = openai.completion_sql_request(prompt)?;

        let sql = format!("SELECT {}", sql);
        info!("openai response sql: {}", sql);
        let database = self.ctx.get_current_database();
        let database: Vec<String> = vec![database];
        let sql: Vec<String> = vec![sql];

        // Mark done.
        self.finished = true;

        Ok(Some(DataBlock::new_from_columns(vec![
            StringType::from_data(database),
            StringType::from_data(sql),
        ])))
    }
}
