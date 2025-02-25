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
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_expression::DataSchemaRef;
use bigbytesdb_common_meta_app::schema::UpdateStreamMetaReq;
use bigbytesdb_common_meta_app::schema::UpsertTableCopiedFileReq;
use bigbytesdb_common_pipeline_core::Pipeline;

use crate::pipelines::PipelineBuilder;
use crate::sessions::QueryContext;

/// This file implements append to table pipeline builder.
impl PipelineBuilder {
    #[allow(clippy::too_many_arguments)]
    pub fn build_append2table_with_commit_pipeline(
        ctx: Arc<QueryContext>,
        main_pipeline: &mut Pipeline,
        table: Arc<dyn Table>,
        source_schema: DataSchemaRef,
        copied_files: Option<UpsertTableCopiedFileReq>,
        update_stream_meta: Vec<UpdateStreamMetaReq>,
        overwrite: bool,
        deduplicated_label: Option<String>,
    ) -> Result<()> {
        Self::fill_and_reorder_columns(ctx.clone(), main_pipeline, table.clone(), source_schema)?;

        table.append_data(ctx.clone(), main_pipeline)?;

        table.commit_insertion(
            ctx,
            main_pipeline,
            copied_files,
            update_stream_meta,
            overwrite,
            None,
            deduplicated_label,
        )?;

        Ok(())
    }

    pub fn build_append2table_without_commit_pipeline(
        ctx: Arc<QueryContext>,
        main_pipeline: &mut Pipeline,
        table: Arc<dyn Table>,
        source_schema: DataSchemaRef,
    ) -> Result<()> {
        Self::fill_and_reorder_columns(ctx.clone(), main_pipeline, table.clone(), source_schema)?;

        table.append_data(ctx, main_pipeline)?;

        Ok(())
    }
}
