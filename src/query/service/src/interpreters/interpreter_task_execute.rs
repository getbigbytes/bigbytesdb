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

use bigbytesdb_common_cloud_control::client_config::make_request;
use bigbytesdb_common_cloud_control::cloud_api::CloudControlApiProvider;
use bigbytesdb_common_cloud_control::pb::ExecuteTaskRequest;
use bigbytesdb_common_config::GlobalConfig;
use bigbytesdb_common_exception::ErrorCode;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_sql::plans::ExecuteTaskPlan;

use crate::interpreters::common::get_task_client_config;
use crate::interpreters::Interpreter;
use crate::pipelines::PipelineBuildResult;
use crate::sessions::QueryContext;

#[derive(Debug)]
pub struct ExecuteTaskInterpreter {
    ctx: Arc<QueryContext>,
    plan: ExecuteTaskPlan,
}

impl ExecuteTaskInterpreter {
    pub fn try_create(ctx: Arc<QueryContext>, plan: ExecuteTaskPlan) -> Result<Self> {
        Ok(ExecuteTaskInterpreter { ctx, plan })
    }
}

#[async_trait::async_trait]
impl Interpreter for ExecuteTaskInterpreter {
    fn name(&self) -> &str {
        "ExecuteTaskInterpreter"
    }

    fn is_ddl(&self) -> bool {
        true
    }

    #[fastrace::trace]
    #[async_backtrace::framed]
    async fn execute2(&self) -> Result<PipelineBuildResult> {
        let config = GlobalConfig::instance();
        if config.query.cloud_control_grpc_server_address.is_none() {
            return Err(ErrorCode::CloudControlNotEnabled(
                "cannot execute task without cloud control enabled, please set cloud_control_grpc_server_address in config",
            ));
        }
        let cloud_api = CloudControlApiProvider::instance();
        let task_client = cloud_api.get_task_client();
        let req = ExecuteTaskRequest {
            task_name: self.plan.task_name.clone(),
            tenant_id: self.plan.tenant.tenant_name().to_string(),
        };
        let config = get_task_client_config(self.ctx.clone(), cloud_api.get_timeout())?;
        let req = make_request(req, config);

        task_client.execute_task(req).await?;
        Ok(PipelineBuildResult::create())
    }
}
