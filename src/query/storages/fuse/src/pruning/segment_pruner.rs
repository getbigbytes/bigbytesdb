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
use bigbytesdb_common_expression::TableSchemaRef;
use bigbytesdb_common_metrics::storage::*;
use bigbytesdb_storages_common_table_meta::meta::CompactSegmentInfo;

use crate::io::SegmentsIO;
use crate::pruning::PruningContext;
use crate::pruning::SegmentLocation;

pub struct SegmentPruner {
    pub pruning_ctx: Arc<PruningContext>,
    pub table_schema: TableSchemaRef,
}

impl SegmentPruner {
    pub fn create(
        pruning_ctx: Arc<PruningContext>,
        table_schema: TableSchemaRef,
    ) -> Result<Arc<SegmentPruner>> {
        Ok(Arc::new(SegmentPruner {
            pruning_ctx,
            table_schema,
        }))
    }

    #[async_backtrace::framed]
    pub async fn pruning(
        &self,
        segment_locs: Vec<SegmentLocation>,
    ) -> Result<Vec<(SegmentLocation, Arc<CompactSegmentInfo>)>> {
        if segment_locs.is_empty() {
            return Ok(vec![]);
        }

        let mut res = Vec::with_capacity(segment_locs.len());

        let pruning_stats = self.pruning_ctx.pruning_stats.clone();
        let range_pruner = self.pruning_ctx.range_pruner.clone();

        for segment_location in segment_locs {
            let info = SegmentsIO::read_compact_segment(
                self.pruning_ctx.dal.clone(),
                segment_location.location.clone(),
                self.table_schema.clone(),
                true,
            )
            .await?;

            let total_bytes = info.summary.uncompressed_byte_size;
            // Perf.
            {
                metrics_inc_segments_range_pruning_before(1);
                metrics_inc_bytes_segment_range_pruning_before(total_bytes);

                pruning_stats.set_segments_range_pruning_before(1);
            }

            if range_pruner.should_keep(&info.summary.col_stats, None) {
                // Perf.
                {
                    metrics_inc_segments_range_pruning_after(1);
                    metrics_inc_bytes_segment_range_pruning_after(total_bytes);

                    pruning_stats.set_segments_range_pruning_after(1);
                }

                res.push((segment_location, info.clone()));
            }
        }
        Ok(res)
    }
}
