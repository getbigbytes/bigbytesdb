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

use bigbytes_common_exception::Result;
use bigbytes_common_license::license::Feature;
use bigbytes_common_license::license_manager::LicenseManagerSwitch;
use bigbytes_common_sql::plans::CreateStreamPlan;
use bigbytes_common_storages_fuse::TableContext;
use bigbytes_enterprise_stream_handler::get_stream_handler;

use crate::interpreters::Interpreter;
use crate::pipelines::PipelineBuildResult;
use crate::sessions::QueryContext;

pub struct CreateStreamInterpreter {
    ctx: Arc<QueryContext>,
    plan: CreateStreamPlan,
}

impl CreateStreamInterpreter {
    pub fn try_create(ctx: Arc<QueryContext>, plan: CreateStreamPlan) -> Result<Self> {
        Ok(CreateStreamInterpreter { ctx, plan })
    }
}

#[async_trait::async_trait]
impl Interpreter for CreateStreamInterpreter {
    fn name(&self) -> &str {
        "CreateStreamInterpreter"
    }

    fn is_ddl(&self) -> bool {
        true
    }

    #[async_backtrace::framed]
    async fn execute2(&self) -> Result<PipelineBuildResult> {
        LicenseManagerSwitch::instance()
            .check_enterprise_enabled(self.ctx.get_license_key(), Feature::Stream)?;

        let handler = get_stream_handler();
        let _ = handler
            .do_create_stream(self.ctx.clone(), &self.plan)
            .await?;

        Ok(PipelineBuildResult::create())
    }
}
