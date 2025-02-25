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

use bigbytesdb_common_base::base::ProgressValues;
use bigbytesdb_common_base::runtime::profile::Profile;
use bigbytesdb_common_base::runtime::profile::ProfileStatisticsName;
use bigbytesdb_common_catalog::table_context::TableContext;
use bigbytesdb_common_exception::ErrorCode;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_expression::DataBlock;
use bigbytesdb_common_pipeline_sources::PrefetchAsyncSource;
use bigbytesdb_storages_common_stage::SingleFilePartition;
use futures::AsyncRead;
use futures::AsyncReadExt;
use log::debug;
use opendal::Operator;

use crate::read::row_based::batch::BytesBatch;

struct FileState {
    file: SingleFilePartition,
    reader: opendal::FuturesAsyncReader,
    offset: usize,
}

pub struct BytesReader {
    table_ctx: Arc<dyn TableContext>,
    op: Operator,
    read_batch_size: usize,
    io_size: usize,
    file_state: Option<FileState>,
    prefetch_num: usize,
}

impl BytesReader {
    pub fn try_create(
        table_ctx: Arc<dyn TableContext>,
        op: Operator,
        read_batch_size: usize,
        prefetch_num: usize,
    ) -> Result<Self> {
        // TODO: Use 8MiB as default IO size for now, we can extract as a new config.
        let default_io_size: usize = 8 * 1024 * 1024;
        // Calculate the IO size, which:
        //
        // - is the multiple of read_batch_size.
        // - is larger or equal to default_io_size.
        let io_size = default_io_size.div_ceil(read_batch_size) * read_batch_size;

        Ok(Self {
            table_ctx,
            op,
            read_batch_size,
            io_size,
            file_state: None,
            prefetch_num,
        })
    }

    pub async fn read_batch(&mut self) -> Result<DataBlock> {
        if let Some(state) = &mut self.file_state {
            let end = state.file.size.min(self.read_batch_size + state.offset);
            let mut buffer = vec![0u8; end - state.offset];
            let n = read_full(&mut state.reader, &mut buffer[..]).await?;
            if n == 0 {
                return Err(ErrorCode::BadBytes(format!(
                    "Unexpected EOF {} expect {} bytes, read only {} bytes.",
                    state.file.path, state.file.size, state.offset
                )));
            };
            buffer.truncate(n);

            Profile::record_usize_profile(ProfileStatisticsName::ScanBytes, n);
            self.table_ctx
                .get_scan_progress()
                .incr(&ProgressValues { rows: 0, bytes: n });

            debug!("read {} bytes from {}", n, state.file.path);
            let offset = state.offset;
            state.offset += n;
            let is_eof = state.offset == state.file.size;

            let batch = Box::new(BytesBatch {
                data: buffer,
                path: state.file.path.clone(),
                offset,
                is_eof,
            });
            if is_eof {
                self.file_state = None;
            }
            Ok(DataBlock::empty_with_meta(batch))
        } else {
            Err(ErrorCode::Internal(
                "Bug: BytesReader::read_batch() should not be called with file_state = None.",
            ))
        }
    }
}

#[async_trait::async_trait]
impl PrefetchAsyncSource for BytesReader {
    const NAME: &'static str = "BytesReader";

    const SKIP_EMPTY_DATA_BLOCK: bool = false;

    fn is_full(&self, prefetched: &[DataBlock]) -> bool {
        prefetched.len() >= self.prefetch_num
    }

    async fn generate(&mut self) -> Result<Option<DataBlock>> {
        if self.file_state.is_none() {
            let part = match self.table_ctx.get_partition() {
                Some(part) => part,
                None => return Ok(None),
            };
            let file = SingleFilePartition::from_part(&part)?.clone();

            let reader = self
                .op
                .reader_with(&file.path)
                .chunk(self.io_size)
                // TODO: Use 4 concurrent for test, let's extract as a new setting.
                .concurrent(4)
                .await?
                .into_futures_async_read(0..file.size as u64)
                .await?;
            self.file_state = Some(FileState {
                file,
                reader,
                offset: 0,
            })
        }
        match self.read_batch().await {
            Ok(block) => Ok(Some(block)),
            Err(e) => Err(e),
        }
    }
}

#[async_backtrace::framed]
pub async fn read_full<R: AsyncRead + Unpin>(reader: &mut R, buf: &mut [u8]) -> Result<usize> {
    let mut buf = &mut buf[0..];
    let mut n = 0;
    while !buf.is_empty() {
        let read = reader.read(buf).await?;
        if read == 0 {
            break;
        }
        n += read;
        buf = &mut buf[read..]
    }
    Ok(n)
}
