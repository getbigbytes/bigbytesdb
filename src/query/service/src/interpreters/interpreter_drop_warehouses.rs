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

use bigbytes_common_base::base::GlobalInstance;
use bigbytes_common_catalog::table_context::TableContext;
use bigbytes_common_exception::Result;
use bigbytes_common_license::license::Feature;
use bigbytes_common_license::license_manager::LicenseManagerSwitch;
use bigbytes_common_sql::plans::DropWarehousePlan;
use bigbytes_enterprise_resources_management::ResourcesManagement;

use crate::interpreters::util::AuditElement;
use crate::interpreters::Interpreter;
use crate::pipelines::PipelineBuildResult;
use crate::sessions::QueryContext;

pub struct DropWarehouseInterpreter {
    #[allow(dead_code)]
    ctx: Arc<QueryContext>,
    plan: DropWarehousePlan,
}

impl DropWarehouseInterpreter {
    pub fn try_create(ctx: Arc<QueryContext>, plan: DropWarehousePlan) -> Result<Self> {
        Ok(DropWarehouseInterpreter { ctx, plan })
    }
}

#[async_trait::async_trait]
impl Interpreter for DropWarehouseInterpreter {
    fn name(&self) -> &str {
        "DropWarehouseInterpreter"
    }

    fn is_ddl(&self) -> bool {
        true
    }

    #[async_backtrace::framed]
    async fn execute2(&self) -> Result<PipelineBuildResult> {
        LicenseManagerSwitch::instance()
            .check_enterprise_enabled(self.ctx.get_license_key(), Feature::SystemManagement)?;

        GlobalInstance::get::<Arc<dyn ResourcesManagement>>()
            .drop_warehouse(self.plan.warehouse.clone())
            .await?;

        let user_info = self.ctx.get_current_user()?;
        log::info!(
            target: "bigbytes::log::audit",
            "{}",
            serde_json::to_string(&AuditElement::create(&user_info, "drop_warehouse", &self.plan))?
        );

        Ok(PipelineBuildResult::create())
    }
}
