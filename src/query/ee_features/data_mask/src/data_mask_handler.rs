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

use bigbytesdb_common_base::base::GlobalInstance;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_meta_app::data_mask::CreateDatamaskReq;
use bigbytesdb_common_meta_app::data_mask::DatamaskMeta;
use bigbytesdb_common_meta_app::data_mask::DropDatamaskReq;
use bigbytesdb_common_meta_app::tenant::Tenant;
use bigbytesdb_common_meta_store::MetaStore;

#[async_trait::async_trait]
pub trait DatamaskHandler: Sync + Send {
    async fn create_data_mask(
        &self,
        meta_api: Arc<MetaStore>,
        req: CreateDatamaskReq,
    ) -> Result<()>;

    async fn drop_data_mask(&self, meta_api: Arc<MetaStore>, req: DropDatamaskReq) -> Result<()>;

    async fn get_data_mask(
        &self,
        meta_api: Arc<MetaStore>,
        tenant: &Tenant,
        name: String,
    ) -> Result<DatamaskMeta>;
}

pub struct DatamaskHandlerWrapper {
    handler: Box<dyn DatamaskHandler>,
}

impl DatamaskHandlerWrapper {
    pub fn new(handler: Box<dyn DatamaskHandler>) -> Self {
        Self { handler }
    }

    pub async fn create_data_mask(
        &self,
        meta_api: Arc<MetaStore>,
        req: CreateDatamaskReq,
    ) -> Result<()> {
        self.handler.create_data_mask(meta_api, req).await
    }

    pub async fn drop_data_mask(
        &self,
        meta_api: Arc<MetaStore>,
        req: DropDatamaskReq,
    ) -> Result<()> {
        self.handler.drop_data_mask(meta_api, req).await
    }

    pub async fn get_data_mask(
        &self,
        meta_api: Arc<MetaStore>,
        tenant: &Tenant,
        name: String,
    ) -> Result<DatamaskMeta> {
        self.handler.get_data_mask(meta_api, tenant, name).await
    }
}

pub fn get_datamask_handler() -> Arc<DatamaskHandlerWrapper> {
    GlobalInstance::get()
}
