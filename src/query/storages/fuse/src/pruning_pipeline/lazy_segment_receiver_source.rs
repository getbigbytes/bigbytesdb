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

use async_channel::Receiver;
use bigbytesdb_common_catalog::table_context::TableContext;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_expression::DataBlock;
use bigbytesdb_common_pipeline_core::processors::OutputPort;
use bigbytesdb_common_pipeline_core::processors::ProcessorPtr;
use bigbytesdb_common_pipeline_sources::AsyncSource;
use bigbytesdb_common_pipeline_sources::AsyncSourcer;

use crate::pruning_pipeline::LazySegmentMeta;
use crate::SegmentLocation;

pub struct LazySegmentReceiverSource {
    pub meta_receiver: Receiver<SegmentLocation>,
}

impl LazySegmentReceiverSource {
    pub fn create(
        ctx: Arc<dyn TableContext>,
        receiver: Receiver<SegmentLocation>,
        output_port: Arc<OutputPort>,
    ) -> Result<ProcessorPtr> {
        AsyncSourcer::create(ctx, output_port, Self {
            meta_receiver: receiver,
        })
    }
}

#[async_trait::async_trait]
impl AsyncSource for LazySegmentReceiverSource {
    const NAME: &'static str = "LazySegmentReceiverSource";
    const SKIP_EMPTY_DATA_BLOCK: bool = false;

    #[async_backtrace::framed]
    async fn generate(&mut self) -> Result<Option<DataBlock>> {
        match self.meta_receiver.recv().await {
            Ok(segments) => Ok(Some(DataBlock::empty_with_meta(LazySegmentMeta::create(
                segments,
            )))),
            Err(_) => {
                // The channel is closed, we should return None to stop generating
                Ok(None)
            }
        }
    }
}
