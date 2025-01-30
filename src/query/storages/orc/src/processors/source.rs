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

use std::mem;
use std::sync::Arc;

use bigbytes_common_base::base::Progress;
use bigbytes_common_base::base::ProgressValues;
use bigbytes_common_base::runtime::profile::Profile;
use bigbytes_common_base::runtime::profile::ProfileStatisticsName;
use bigbytes_common_catalog::table_context::TableContext;
use bigbytes_common_exception::ErrorCode;
use bigbytes_common_exception::Result;
use bigbytes_common_expression::DataBlock;
use bigbytes_common_pipeline_core::processors::OutputPort;
use bigbytes_common_pipeline_core::processors::ProcessorPtr;
use bigbytes_common_pipeline_sources::AsyncSource;
use bigbytes_common_pipeline_sources::AsyncSourcer;
use bigbytes_storages_common_stage::SingleFilePartition;
use opendal::Operator;
use orc_rust::async_arrow_reader::StripeFactory;
use orc_rust::ArrowReaderBuilder;

use crate::chunk_reader_impl::OrcChunkReader;
use crate::strip::StripeInMemory;
use crate::utils::map_orc_error;

pub struct ORCSource {
    table_ctx: Arc<dyn TableContext>,
    op: Operator,
    pub(crate) reader: Option<(String, Box<StripeFactory<OrcChunkReader>>, usize)>,
    scan_progress: Arc<Progress>,

    arrow_schema: arrow_schema::SchemaRef,
    schema_from: String,
}

impl ORCSource {
    pub fn try_create(
        output: Arc<OutputPort>,
        table_ctx: Arc<dyn TableContext>,
        op: Operator,
        arrow_schema: arrow_schema::SchemaRef,
        schema_from: String,
    ) -> Result<ProcessorPtr> {
        let scan_progress = table_ctx.get_scan_progress();

        AsyncSourcer::create(table_ctx.clone(), output, ORCSource {
            table_ctx,
            op,
            scan_progress,
            reader: None,
            arrow_schema,
            schema_from,
        })
    }

    fn check_file_schema(&self, arrow_schema: arrow_schema::SchemaRef, path: &str) -> Result<()> {
        if self.arrow_schema.fields != arrow_schema.fields {
            return Err(ErrorCode::TableSchemaMismatch(format!(
                "infer schema from '{}', but get diff schema in file '{}'. Expected schema: {:?}, actual: {:?}",
                self.schema_from, path, self.arrow_schema, arrow_schema
            )));
        }
        Ok(())
    }

    async fn next_part(&mut self) -> Result<bool> {
        let part = match self.table_ctx.get_partition() {
            Some(part) => part,
            None => return Ok(false),
        };
        let file = SingleFilePartition::from_part(&part)?.clone();
        let path = file.path.clone();
        let size = file.size;

        let file = OrcChunkReader {
            operator: self.op.clone(),
            size: file.size as u64,
            path: file.path,
        };
        let builder = ArrowReaderBuilder::try_new_async(file)
            .await
            .map_err(|e| map_orc_error(e, &path))?;
        let reader = builder.build_async();
        let (factory, schema) = reader.into_parts();
        let factory = factory.unwrap();
        self.check_file_schema(schema, &path)?;

        self.reader = Some((path, factory, size));
        Ok(true)
    }
}

#[async_trait::async_trait]
impl AsyncSource for ORCSource {
    const NAME: &'static str = "ORCSource";
    const SKIP_EMPTY_DATA_BLOCK: bool = false;

    #[async_backtrace::framed]
    async fn generate(&mut self) -> Result<Option<DataBlock>> {
        loop {
            if self.reader.is_none() && !self.next_part().await? {
                return Ok(None);
            }
            if let Some((path, factory, size)) = mem::take(&mut self.reader) {
                let (factory, stripe) = factory
                    .read_next_stripe()
                    .await
                    .map_err(|e| ErrorCode::StorageOther(e.to_string()))?;
                match stripe {
                    None => {
                        self.reader = None;
                        let progress_values = ProgressValues {
                            rows: 0,
                            bytes: size,
                        };
                        self.scan_progress.incr(&progress_values);
                        Profile::record_usize_profile(ProfileStatisticsName::ScanBytes, size);
                        Profile::record_usize_profile(ProfileStatisticsName::ScanPartitions, 1);
                        continue;
                    }
                    Some(stripe) => {
                        let progress_values = ProgressValues {
                            rows: stripe.number_of_rows(),
                            bytes: 0,
                        };
                        self.scan_progress.incr(&progress_values);
                        self.reader = Some((path.clone(), Box::new(factory), size));
                        let meta = Box::new(StripeInMemory {
                            path,
                            stripe,
                            schema: None,
                        });
                        return Ok(Some(DataBlock::empty_with_meta(meta)));
                    }
                }
            } else {
                return Err(ErrorCode::Internal(
                    "Bug: ORCSource: should not be called with reader != None.",
                ));
            }
        }
    }
}
