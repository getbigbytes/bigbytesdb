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
use bigbytesdb_common_cloud_control::pb::DescribeTaskRequest;
use bigbytesdb_common_config::GlobalConfig;
use bigbytesdb_common_exception::ErrorCode;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_sql::plans::DescribeTaskPlan;
use bigbytesdb_common_storages_system::parse_tasks_to_datablock;

use crate::interpreters::common::get_task_client_config;
use crate::interpreters::Interpreter;
use crate::pipelines::PipelineBuildResult;
use crate::sessions::QueryContext;

#[derive(Debug)]
pub struct DescribeTaskInterpreter {
    ctx: Arc<QueryContext>,
    plan: DescribeTaskPlan,
}

impl DescribeTaskInterpreter {
    pub fn try_create(ctx: Arc<QueryContext>, plan: DescribeTaskPlan) -> Result<Self> {
        Ok(DescribeTaskInterpreter { ctx, plan })
    }
}

impl DescribeTaskInterpreter {
    fn build_request(&self) -> DescribeTaskRequest {
        let plan = self.plan.clone();
        DescribeTaskRequest {
            task_name: plan.task_name,
            tenant_id: plan.tenant.tenant_name().to_string(),
            if_exist: false,
        }
    }
}

#[async_trait::async_trait]
impl Interpreter for DescribeTaskInterpreter {
    fn name(&self) -> &str {
        "DescribeTaskInterpreter"
    }

    fn is_ddl(&self) -> bool {
        false
    }

    #[fastrace::trace]
    #[async_backtrace::framed]
    async fn execute2(&self) -> Result<PipelineBuildResult> {
        let config = GlobalConfig::instance();
        if config.query.cloud_control_grpc_server_address.is_none() {
            return Err(ErrorCode::CloudControlNotEnabled(
                "cannot describe task without cloud control enabled, please set cloud_control_grpc_server_address in config",
            ));
        }
        let cloud_api = CloudControlApiProvider::instance();
        let task_client = cloud_api.get_task_client();
        let req = self.build_request();
        let config = get_task_client_config(self.ctx.clone(), cloud_api.get_timeout())?;
        let req = make_request(req, config);
        let resp = task_client.describe_task(req).await?;
        if resp.task.is_none() {
            return Ok(PipelineBuildResult::create());
        }
        let tasks = vec![resp.task.unwrap()];
        let result = parse_tasks_to_datablock(tasks)?;
        PipelineBuildResult::from_blocks(vec![result])
    }
}
