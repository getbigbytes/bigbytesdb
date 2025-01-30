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

use bigbytes_common_catalog::plan::StreamColumnMeta;
use bigbytes_common_exception::ErrorCode;
use bigbytes_common_exception::Result;
use bigbytes_common_expression::BlockMetaInfoDowncast;
use bigbytes_common_expression::DataBlock;
use bigbytes_common_pipeline_core::processors::InputPort;
use bigbytes_common_pipeline_core::processors::OutputPort;
use bigbytes_common_pipeline_core::processors::ProcessorPtr;
use bigbytes_common_pipeline_transforms::processors::Transform;
use bigbytes_common_pipeline_transforms::processors::Transformer;
use bigbytes_common_sql::StreamContext;

pub struct TransformAddStreamColumns {
    stream_ctx: StreamContext,
}

impl TransformAddStreamColumns
where Self: Transform
{
    pub fn new(stream_ctx: StreamContext) -> Self {
        Self { stream_ctx }
    }
    pub fn try_create(
        input: Arc<InputPort>,
        output: Arc<OutputPort>,
        stream_ctx: StreamContext,
    ) -> Result<ProcessorPtr> {
        Ok(ProcessorPtr::create(Transformer::create(
            input,
            output,
            Self { stream_ctx },
        )))
    }
}

impl Transform for TransformAddStreamColumns {
    const NAME: &'static str = "AddStreamColumnsTransform";

    fn transform(&mut self, mut block: DataBlock) -> Result<DataBlock> {
        if !block.is_empty() {
            if let Some(meta) = block.take_meta() {
                let meta = StreamColumnMeta::downcast_from(meta)
                    .ok_or_else(|| ErrorCode::Internal("It's a bug"))?;

                block = self.stream_ctx.apply(block, &meta)?.add_meta(meta.inner)?;
            }
        }

        Ok(block)
    }
}
