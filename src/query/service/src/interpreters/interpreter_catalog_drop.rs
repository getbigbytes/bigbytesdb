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

use async_trait::async_trait;
use bigbytesdb_common_catalog::catalog::CatalogManager;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_sql::plans::DropCatalogPlan;
use bigbytesdb_common_storages_fuse::TableContext;
use log::debug;

use super::Interpreter;
use crate::pipelines::PipelineBuildResult;
use crate::sessions::QueryContext;

pub struct DropCatalogInterpreter {
    ctx: Arc<QueryContext>,
    plan: DropCatalogPlan,
}

impl DropCatalogInterpreter {
    pub fn create(ctx: Arc<QueryContext>, plan: DropCatalogPlan) -> Self {
        Self { ctx, plan }
    }
}

#[async_trait]
impl Interpreter for DropCatalogInterpreter {
    fn name(&self) -> &str {
        "DropCatalogInterpreter"
    }

    fn is_ddl(&self) -> bool {
        true
    }

    #[fastrace::trace]
    #[async_backtrace::framed]
    async fn execute2(&self) -> Result<PipelineBuildResult> {
        debug!("ctx.id" = self.ctx.get_id().as_str(); "drop_catalog_execute");

        let mgr = CatalogManager::instance();
        mgr.drop_catalog(self.plan.clone().into()).await?;

        Ok(PipelineBuildResult::create())
    }
}
