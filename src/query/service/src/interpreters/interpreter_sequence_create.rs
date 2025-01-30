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

use chrono::Utc;
use bigbytes_common_exception::Result;
use bigbytes_common_meta_app::schema::CreateSequenceReq;
use bigbytes_common_sql::plans::CreateSequencePlan;
use bigbytes_common_storages_fuse::TableContext;

use crate::interpreters::Interpreter;
use crate::pipelines::PipelineBuildResult;
use crate::sessions::QueryContext;

pub struct CreateSequenceInterpreter {
    ctx: Arc<QueryContext>,
    plan: CreateSequencePlan,
}

impl CreateSequenceInterpreter {
    pub fn try_create(ctx: Arc<QueryContext>, plan: CreateSequencePlan) -> Result<Self> {
        Ok(CreateSequenceInterpreter { ctx, plan })
    }
}

#[async_trait::async_trait]
impl Interpreter for CreateSequenceInterpreter {
    fn name(&self) -> &str {
        "CreateSequenceInterpreter"
    }

    fn is_ddl(&self) -> bool {
        true
    }

    #[async_backtrace::framed]
    async fn execute2(&self) -> Result<PipelineBuildResult> {
        let req = CreateSequenceReq {
            create_option: self.plan.create_option,
            ident: self.plan.ident.clone(),
            comment: self.plan.comment.clone(),
            create_on: Utc::now(),
        };
        let catalog = self.ctx.get_default_catalog()?;
        let _reply = catalog.create_sequence(req).await?;

        Ok(PipelineBuildResult::create())
    }
}
