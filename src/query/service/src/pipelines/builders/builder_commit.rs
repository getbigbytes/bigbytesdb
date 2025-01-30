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

use bigbytes_common_exception::Result;
use bigbytes_common_pipeline_transforms::processors::TransformPipelineHelper;
use bigbytes_common_sql::executor::physical_plans::CommitSink as PhysicalCommitSink;
use bigbytes_common_sql::executor::physical_plans::MutationKind;
use bigbytes_common_storages_fuse::operations::CommitSink;
use bigbytes_common_storages_fuse::operations::MutationGenerator;
use bigbytes_common_storages_fuse::operations::TableMutationAggregator;
use bigbytes_common_storages_fuse::operations::TransformMergeCommitMeta;
use bigbytes_common_storages_fuse::FuseTable;
use bigbytes_storages_common_table_meta::readers::snapshot_reader::TableSnapshotAccessor;

use crate::pipelines::PipelineBuilder;

impl PipelineBuilder {
    pub(crate) fn build_commit_sink(&mut self, plan: &PhysicalCommitSink) -> Result<()> {
        self.build_pipeline(&plan.input)?;
        let table = self.ctx.build_table_by_table_info(&plan.table_info, None)?;
        let table = FuseTable::try_from_table(table.as_ref())?;
        let cluster_key_id = table.cluster_key_id();

        self.main_pipeline.try_resize(1)?;
        if plan.merge_meta {
            self.main_pipeline
                .add_accumulating_transformer(|| TransformMergeCommitMeta::create(cluster_key_id));
        } else {
            self.main_pipeline.add_async_accumulating_transformer(|| {
                let base_segments = if matches!(
                    plan.mutation_kind,
                    MutationKind::Compact | MutationKind::Insert | MutationKind::Recluster
                ) {
                    vec![]
                } else {
                    plan.snapshot.segments().to_vec()
                };

                // extract re-cluster related mutations from physical plan
                let recluster_info = plan.recluster_info.clone().unwrap_or_default();

                TableMutationAggregator::create(
                    table,
                    self.ctx.clone(),
                    base_segments,
                    recluster_info.merged_blocks,
                    recluster_info.removed_segment_indexes,
                    recluster_info.removed_statistics,
                    plan.mutation_kind,
                )
            });
        }

        let snapshot_gen = MutationGenerator::new(plan.snapshot.clone(), plan.mutation_kind);
        self.main_pipeline.add_sink(|input| {
            CommitSink::try_create(
                table,
                self.ctx.clone(),
                None,
                plan.update_stream_meta.clone(),
                snapshot_gen.clone(),
                input,
                None,
                None,
                plan.deduplicated_label.clone(),
            )
        })
    }
}
