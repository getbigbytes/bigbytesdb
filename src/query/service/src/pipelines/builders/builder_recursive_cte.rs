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

use bigbytesdb_common_catalog::table_context::TableContext;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_sql::executor::physical_plans::RecursiveCteScan;
use bigbytesdb_common_sql::executor::physical_plans::UnionAll;

use crate::pipelines::processors::transforms::TransformRecursiveCteScan;
use crate::pipelines::processors::transforms::TransformRecursiveCteSource;
use crate::pipelines::PipelineBuilder;

impl PipelineBuilder {
    pub fn build_recursive_cte_source(&mut self, union_all: &UnionAll) -> Result<()> {
        let max_threads = self.ctx.get_settings().get_max_threads()?;
        self.main_pipeline.add_source(
            |output_port| {
                TransformRecursiveCteSource::try_create(
                    self.ctx.clone(),
                    output_port.clone(),
                    union_all.clone(),
                )
            },
            1,
        )?;
        self.main_pipeline.resize(max_threads as usize, true)
    }

    pub fn build_recursive_cte_scan(&mut self, r_cte_scan: &RecursiveCteScan) -> Result<()> {
        let max_threads = self.ctx.get_settings().get_max_threads()?;
        self.main_pipeline.add_source(
            |output_port| {
                TransformRecursiveCteScan::create(
                    self.ctx.clone(),
                    output_port.clone(),
                    r_cte_scan.table_name.clone(),
                )
            },
            1,
        )?;
        self.main_pipeline.resize(max_threads as usize, true)
    }
}
