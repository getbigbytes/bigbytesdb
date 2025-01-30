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

use bigbytesdb_common_meta_app::background::BackgroundJobInfo;
use bigbytesdb_common_meta_app::background::BackgroundTaskIdent;
use bigbytesdb_common_meta_app::background::BackgroundTaskInfo;
use bigbytesdb_common_meta_app::background::CreateBackgroundJobReply;
use bigbytesdb_common_meta_app::background::CreateBackgroundJobReq;
use bigbytesdb_common_meta_app::background::DeleteBackgroundJobReply;
use bigbytesdb_common_meta_app::background::DeleteBackgroundJobReq;
use bigbytesdb_common_meta_app::background::GetBackgroundJobReply;
use bigbytesdb_common_meta_app::background::GetBackgroundJobReq;
use bigbytesdb_common_meta_app::background::GetBackgroundTaskReply;
use bigbytesdb_common_meta_app::background::GetBackgroundTaskReq;
use bigbytesdb_common_meta_app::background::ListBackgroundJobsReq;
use bigbytesdb_common_meta_app::background::ListBackgroundTasksReq;
use bigbytesdb_common_meta_app::background::UpdateBackgroundJobParamsReq;
use bigbytesdb_common_meta_app::background::UpdateBackgroundJobReply;
use bigbytesdb_common_meta_app::background::UpdateBackgroundJobStatusReq;
use bigbytesdb_common_meta_app::background::UpdateBackgroundTaskReply;
use bigbytesdb_common_meta_app::background::UpdateBackgroundTaskReq;
use bigbytesdb_common_meta_types::SeqV;

use crate::kv_app_error::KVAppError;

#[async_trait::async_trait]
pub trait BackgroundApi: Send + Sync {
    async fn create_background_job(
        &self,
        req: CreateBackgroundJobReq,
    ) -> Result<CreateBackgroundJobReply, KVAppError>;

    async fn drop_background_job(
        &self,
        req: DeleteBackgroundJobReq,
    ) -> Result<DeleteBackgroundJobReply, KVAppError>;

    async fn update_background_job_status(
        &self,
        req: UpdateBackgroundJobStatusReq,
    ) -> Result<UpdateBackgroundJobReply, KVAppError>;

    async fn update_background_job_params(
        &self,
        req: UpdateBackgroundJobParamsReq,
    ) -> Result<UpdateBackgroundJobReply, KVAppError>;

    async fn get_background_job(
        &self,
        req: GetBackgroundJobReq,
    ) -> Result<GetBackgroundJobReply, KVAppError>;

    /// Return a list of job name and job info
    async fn list_background_jobs(
        &self,
        req: ListBackgroundJobsReq,
    ) -> Result<Vec<(String, SeqV<BackgroundJobInfo>)>, KVAppError>;

    /// Return a list of background tasks (task_id, BackgroundInfo)
    async fn list_background_tasks(
        &self,
        req: ListBackgroundTasksReq,
    ) -> Result<Vec<(BackgroundTaskIdent, SeqV<BackgroundTaskInfo>)>, KVAppError>;

    async fn update_background_task(
        &self,
        req: UpdateBackgroundTaskReq,
    ) -> Result<UpdateBackgroundTaskReply, KVAppError>;

    async fn get_background_task(
        &self,
        req: GetBackgroundTaskReq,
    ) -> Result<GetBackgroundTaskReply, KVAppError>;
}
