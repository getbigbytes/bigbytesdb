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

use bigbytesdb_common_catalog::plan::DataSourcePlan;
use bigbytesdb_common_catalog::plan::Projection;
use bigbytesdb_common_catalog::table_context::TableContext;
use bigbytesdb_common_exception::ErrorCode;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_expression::types::nullable::NullableColumn;
use bigbytesdb_common_expression::types::Bitmap;
use bigbytesdb_common_expression::types::DataType;
use bigbytesdb_common_expression::types::NumberDataType;
use bigbytesdb_common_expression::BlockEntry;
use bigbytesdb_common_expression::Column;
use bigbytesdb_common_expression::DataBlock;
use bigbytesdb_common_expression::Value;
use bigbytesdb_common_pipeline_core::processors::InputPort;
use bigbytesdb_common_pipeline_core::processors::OutputPort;
use bigbytesdb_common_pipeline_core::processors::ProcessorPtr;
use bigbytesdb_common_pipeline_transforms::AsyncAccumulatingTransform;
use bigbytesdb_common_pipeline_transforms::AsyncAccumulatingTransformer;
use bigbytesdb_storages_common_io::ReadSettings;

use super::native_rows_fetcher::NativeRowsFetcher;
use super::parquet_rows_fetcher::ParquetRowsFetcher;
use crate::FuseStorageFormat;
use crate::FuseTable;

type RowFetcher = Box<dyn Fn(Arc<InputPort>, Arc<OutputPort>) -> Result<ProcessorPtr>>;

pub fn row_fetch_processor(
    ctx: Arc<dyn TableContext>,
    row_id_col_offset: usize,
    source: &DataSourcePlan,
    projection: Projection,
    need_wrap_nullable: bool,
) -> Result<RowFetcher> {
    let table = ctx.build_table_from_source_plan(source)?;
    let fuse_table = table
        .as_any()
        .downcast_ref::<FuseTable>()
        .ok_or_else(|| ErrorCode::Internal("Row fetcher is only supported by Fuse engine"))?
        .to_owned();
    let fuse_table = Arc::new(fuse_table);
    let block_reader =
        fuse_table.create_block_reader(ctx.clone(), projection.clone(), false, false, true)?;
    let max_threads = ctx.get_settings().get_max_threads()? as usize;

    match &fuse_table.storage_format {
        FuseStorageFormat::Native => Ok(Box::new(move |input, output| {
            Ok(if block_reader.support_blocking_api() {
                TransformRowsFetcher::create(
                    input,
                    output,
                    row_id_col_offset,
                    NativeRowsFetcher::<true>::create(
                        fuse_table.clone(),
                        projection.clone(),
                        block_reader.clone(),
                        max_threads,
                    ),
                    need_wrap_nullable,
                )
            } else {
                TransformRowsFetcher::create(
                    input,
                    output,
                    row_id_col_offset,
                    NativeRowsFetcher::<false>::create(
                        fuse_table.clone(),
                        projection.clone(),
                        block_reader.clone(),
                        max_threads,
                    ),
                    need_wrap_nullable,
                )
            })
        })),
        FuseStorageFormat::Parquet => {
            let read_settings = ReadSettings::from_ctx(&ctx)?;
            Ok(Box::new(move |input, output| {
                Ok(if block_reader.support_blocking_api() {
                    TransformRowsFetcher::create(
                        input,
                        output,
                        row_id_col_offset,
                        ParquetRowsFetcher::<true>::create(
                            fuse_table.clone(),
                            projection.clone(),
                            block_reader.clone(),
                            read_settings,
                            max_threads,
                        ),
                        need_wrap_nullable,
                    )
                } else {
                    TransformRowsFetcher::create(
                        input,
                        output,
                        row_id_col_offset,
                        ParquetRowsFetcher::<false>::create(
                            fuse_table.clone(),
                            projection.clone(),
                            block_reader.clone(),
                            read_settings,
                            max_threads,
                        ),
                        need_wrap_nullable,
                    )
                })
            }))
        }
    }
}

#[async_trait::async_trait]
pub trait RowsFetcher {
    async fn on_start(&mut self) -> Result<()>;
    async fn fetch(&mut self, row_ids: &[u64]) -> Result<DataBlock>;
}

pub struct TransformRowsFetcher<F: RowsFetcher> {
    row_id_col_offset: usize,
    fetcher: F,
    need_wrap_nullable: bool,
    blocks: Vec<DataBlock>,
}

#[async_trait::async_trait]
impl<F> AsyncAccumulatingTransform for TransformRowsFetcher<F>
where F: RowsFetcher + Send + Sync + 'static
{
    const NAME: &'static str = "TransformRowsFetcher";

    #[async_backtrace::framed]
    async fn on_start(&mut self) -> Result<()> {
        self.fetcher.on_start().await
    }

    async fn transform(&mut self, data: DataBlock) -> Result<Option<DataBlock>> {
        self.blocks.push(data);
        Ok(None)
    }

    #[async_backtrace::framed]
    async fn on_finish(&mut self, _output: bool) -> Result<Option<DataBlock>> {
        if self.blocks.is_empty() {
            return Ok(None);
        }

        let start_time = std::time::Instant::now();
        let num_blocks = self.blocks.len();
        let mut data = DataBlock::concat(&self.blocks)?;
        self.blocks.clear();

        let num_rows = data.num_rows();
        if num_rows == 0 {
            return Ok(None);
        }

        let entry = &data.columns()[self.row_id_col_offset];
        let value = entry
            .value
            .convert_to_full_column(&entry.data_type, num_rows);
        let row_id_column = if matches!(entry.data_type, DataType::Number(NumberDataType::UInt64)) {
            value.into_number().unwrap().into_u_int64().unwrap()
        } else {
            // From merge into matched data, the row id column is nullable but has no null value.
            let value = *value.into_nullable().unwrap();
            debug_assert!(value.validity.null_count() == 0);
            value.column.into_number().unwrap().into_u_int64().unwrap()
        };

        let fetched_block = self.fetcher.fetch(&row_id_column).await?;

        for col in fetched_block.columns().iter() {
            if self.need_wrap_nullable {
                data.add_column(wrap_true_validity(col, num_rows));
            } else {
                data.add_column(col.clone());
            }
        }

        log::info!(
            "TransformRowsFetcher on_finish: num_rows: {}, input blocks: {} in {} milliseconds",
            num_rows,
            num_blocks,
            start_time.elapsed().as_millis()
        );

        Ok(Some(data))
    }
}

impl<F> TransformRowsFetcher<F>
where F: RowsFetcher + Send + Sync + 'static
{
    fn create(
        input: Arc<InputPort>,
        output: Arc<OutputPort>,
        row_id_col_offset: usize,
        fetcher: F,
        need_wrap_nullable: bool,
    ) -> ProcessorPtr {
        ProcessorPtr::create(AsyncAccumulatingTransformer::create(input, output, Self {
            row_id_col_offset,
            fetcher,
            need_wrap_nullable,
            blocks: vec![],
        }))
    }
}

fn wrap_true_validity(column: &BlockEntry, num_rows: usize) -> BlockEntry {
    let (value, data_type) = (&column.value, &column.data_type);
    let col = value.convert_to_full_column(data_type, num_rows);
    if matches!(col, Column::Null { .. }) || col.as_nullable().is_some() {
        column.clone()
    } else {
        let col = NullableColumn::new_column(col, Bitmap::new_trued(num_rows));
        BlockEntry::new(data_type.wrap_nullable(), Value::Column(col))
    }
}
