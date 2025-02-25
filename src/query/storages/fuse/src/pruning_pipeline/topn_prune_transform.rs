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

use bigbytesdb_common_exception::Result;
use bigbytesdb_common_expression::DataBlock;
use bigbytesdb_common_pipeline_core::processors::InputPort;
use bigbytesdb_common_pipeline_core::processors::OutputPort;
use bigbytesdb_common_pipeline_core::processors::ProcessorPtr;
use bigbytesdb_common_pipeline_transforms::BlockMetaAccumulatingTransform;
use bigbytesdb_common_pipeline_transforms::BlockMetaAccumulatingTransformer;
use bigbytesdb_storages_common_pruner::BlockMetaIndex;
use bigbytesdb_storages_common_pruner::TopNPrunner;
use bigbytesdb_storages_common_table_meta::meta::BlockMeta;

use crate::pruning_pipeline::block_prune_result_meta::BlockPruneResult;

// TopNPruneTransform is a processor that will accumulate the block meta and not push to
// downstream until all data is received and pruned.
pub struct TopNPruneTransform {
    topn_pruner: TopNPrunner,
    metas: Vec<(BlockMetaIndex, Arc<BlockMeta>)>,
}

impl BlockMetaAccumulatingTransform<BlockPruneResult> for TopNPruneTransform {
    const NAME: &'static str = "TopNPruneTransform";

    fn transform(&mut self, data: BlockPruneResult) -> Result<Option<DataBlock>> {
        self.metas.extend(data.block_metas);
        Ok(None)
    }

    fn on_finish(&mut self, _output: bool) -> Result<Option<DataBlock>> {
        self.do_topn_prune()
    }
}

impl TopNPruneTransform {
    pub fn create(
        input: Arc<InputPort>,
        output: Arc<OutputPort>,
        topn_pruner: TopNPrunner,
    ) -> Result<ProcessorPtr> {
        Ok(ProcessorPtr::create(
            BlockMetaAccumulatingTransformer::create(input, output, TopNPruneTransform {
                topn_pruner,
                metas: vec![],
            }),
        ))
    }
    fn do_topn_prune(&self) -> Result<Option<DataBlock>> {
        let pruned = self
            .topn_pruner
            .prune(self.metas.clone())
            .unwrap_or_else(|_| self.metas.clone());
        if pruned.is_empty() {
            Ok(None)
        } else {
            Ok(Some(DataBlock::empty_with_meta(BlockPruneResult::create(
                pruned,
            ))))
        }
    }
}
