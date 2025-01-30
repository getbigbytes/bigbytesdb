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

use bigbytesdb_common_catalog::table::Table;
use bigbytesdb_common_catalog::table_context::TableContext;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_expression::DataBlock;
use bigbytesdb_common_pipeline_core::Pipeline;
use bigbytesdb_common_pipeline_sources::OneBlockSource;
use bigbytesdb_storages_common_table_meta::meta::TableSnapshot;

use crate::operations::common::CommitMeta;
use crate::operations::common::CommitSink;
use crate::operations::common::ConflictResolveContext;
use crate::operations::common::TruncateGenerator;
use crate::operations::common::TruncateMode;
use crate::FuseTable;

impl FuseTable {
    #[inline]
    #[async_backtrace::framed]
    pub async fn do_truncate(
        &self,
        ctx: Arc<dyn TableContext>,
        pipeline: &mut Pipeline,
        mode: TruncateMode,
    ) -> Result<()> {
        if let Some(prev_snapshot) = self.read_table_snapshot().await? {
            self.build_truncate_pipeline(ctx, pipeline, mode, prev_snapshot)?;
        }
        Ok(())
    }

    #[inline]
    #[async_backtrace::framed]
    pub fn build_truncate_pipeline(
        &self,
        ctx: Arc<dyn TableContext>,
        pipeline: &mut Pipeline,
        mode: TruncateMode,
        prev_snapshot: Arc<TableSnapshot>,
    ) -> Result<()> {
        // Delete operation commit can retry multi-times if table version mismatched.
        let prev_snapshot_id = if !matches!(mode, TruncateMode::Delete) {
            Some(prev_snapshot.snapshot_id)
        } else {
            None
        };
        pipeline.add_source(
            |output| {
                let meta = CommitMeta {
                    conflict_resolve_context: ConflictResolveContext::None,
                    new_segment_locs: vec![],
                    table_id: self.get_id(),
                };
                let block = DataBlock::empty_with_meta(Box::new(meta));
                OneBlockSource::create(output, block)
            },
            1,
        )?;

        let snapshot_gen = TruncateGenerator::new(mode);
        pipeline.add_sink(|input| {
            CommitSink::try_create(
                self,
                ctx.clone(),
                None,
                vec![],
                snapshot_gen.clone(),
                input,
                None,
                prev_snapshot_id,
                None,
            )
        })
    }
}
