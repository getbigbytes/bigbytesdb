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

use bigbytesdb_common_catalog::catalog::StorageDescription;
use bigbytesdb_common_catalog::plan::DataSourcePlan;
use bigbytesdb_common_catalog::plan::PartStatistics;
use bigbytesdb_common_catalog::plan::Partitions;
use bigbytesdb_common_catalog::plan::PushDownInfo;
use bigbytesdb_common_catalog::table::DistributionLevel;
use bigbytesdb_common_catalog::table::Table;
use bigbytesdb_common_catalog::table_context::TableContext;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_expression::DataBlock;
use bigbytesdb_common_expression::DataSchemaRef;
use bigbytesdb_common_meta_app::schema::TableInfo;
use bigbytesdb_common_pipeline_core::processors::OutputPort;
use bigbytesdb_common_pipeline_core::processors::ProcessorPtr;
use bigbytesdb_common_pipeline_core::Pipeline;
use bigbytesdb_common_pipeline_sinks::EmptySink;
use bigbytesdb_common_pipeline_sources::SyncSource;
use bigbytesdb_common_pipeline_sources::SyncSourcer;

pub struct NullTable {
    table_info: TableInfo,
}

impl NullTable {
    pub fn try_create(table_info: TableInfo) -> Result<Box<dyn Table>> {
        Ok(Box::new(Self { table_info }))
    }

    pub fn description() -> StorageDescription {
        StorageDescription {
            engine_name: "NULL".to_string(),
            comment: "NULL Storage Engine".to_string(),
            ..Default::default()
        }
    }
}

#[async_trait::async_trait]
impl Table for NullTable {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_table_info(&self) -> &TableInfo {
        &self.table_info
    }

    /// Null do not keep data, it's safe to make it non-local.
    fn distribution_level(&self) -> DistributionLevel {
        DistributionLevel::Cluster
    }

    #[async_backtrace::framed]
    async fn read_partitions(
        &self,
        _ctx: Arc<dyn TableContext>,
        _push_downs: Option<PushDownInfo>,
        _dry_run: bool,
    ) -> Result<(PartStatistics, Partitions)> {
        Ok((PartStatistics::default(), Partitions::default()))
    }

    fn read_data(
        &self,
        ctx: Arc<dyn TableContext>,
        _: &DataSourcePlan,
        pipeline: &mut Pipeline,
        _put_cache: bool,
    ) -> Result<()> {
        let schema: DataSchemaRef = Arc::new(self.table_info.schema().into());
        pipeline.add_source(
            |output| NullSource::create(ctx.clone(), output, schema.clone()),
            1,
        )?;

        Ok(())
    }

    fn append_data(&self, _: Arc<dyn TableContext>, pipeline: &mut Pipeline) -> Result<()> {
        pipeline.add_sink(|input| Ok(ProcessorPtr::create(EmptySink::create(input))))?;
        Ok(())
    }
}

struct NullSource {
    finish: bool,
    schema: DataSchemaRef,
}

impl NullSource {
    pub fn create(
        ctx: Arc<dyn TableContext>,
        output: Arc<OutputPort>,
        schema: DataSchemaRef,
    ) -> Result<ProcessorPtr> {
        SyncSourcer::create(ctx, output, NullSource {
            finish: false,
            schema,
        })
    }
}

impl SyncSource for NullSource {
    const NAME: &'static str = "NullSource";

    fn generate(&mut self) -> Result<Option<DataBlock>> {
        if self.finish {
            return Ok(None);
        }

        self.finish = true;
        Ok(Some(DataBlock::empty_with_schema(self.schema.clone())))
    }
}
