// Copyright 2024 Digitrans Inc
//
// Licensed under the Elastic License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.elastic.co/licensing/elastic-license
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use bigbytesdb_common_base::base::GlobalInstance;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_meta_api::DatamaskApi;
use bigbytesdb_common_meta_app::app_error::AppError;
use bigbytesdb_common_meta_app::data_mask::CreateDatamaskReq;
use bigbytesdb_common_meta_app::data_mask::DataMaskNameIdent;
use bigbytesdb_common_meta_app::data_mask::DatamaskMeta;
use bigbytesdb_common_meta_app::data_mask::DropDatamaskReq;
use bigbytesdb_common_meta_app::tenant::Tenant;
use bigbytesdb_common_meta_store::MetaStore;
use bigbytesdb_enterprise_data_mask_feature::data_mask_handler::DatamaskHandler;
use bigbytesdb_enterprise_data_mask_feature::data_mask_handler::DatamaskHandlerWrapper;

pub struct RealDatamaskHandler {}

#[async_trait::async_trait]
impl DatamaskHandler for RealDatamaskHandler {
    async fn create_data_mask(
        &self,
        meta_api: Arc<MetaStore>,
        req: CreateDatamaskReq,
    ) -> Result<()> {
        let _ = meta_api.create_data_mask(req).await?;

        Ok(())
    }

    async fn drop_data_mask(&self, meta_api: Arc<MetaStore>, req: DropDatamaskReq) -> Result<()> {
        let dropped = meta_api.drop_data_mask(&req.name).await?;
        if dropped.is_none() {
            if req.if_exists {
                // Ok
            } else {
                return Err(AppError::from(req.name.unknown_error("drop data mask")).into());
            }
        }

        Ok(())
    }

    async fn get_data_mask(
        &self,
        meta_api: Arc<MetaStore>,
        tenant: &Tenant,
        name: String,
    ) -> Result<DatamaskMeta> {
        let name_ident = DataMaskNameIdent::new(tenant, name);
        let seq_meta = meta_api
            .get_data_mask(&name_ident)
            .await?
            .ok_or_else(|| AppError::from(name_ident.unknown_error("get data mask")))?;
        Ok(seq_meta.data)
    }
}

impl RealDatamaskHandler {
    pub fn init() -> Result<()> {
        let rm = RealDatamaskHandler {};
        let wrapper = DatamaskHandlerWrapper::new(Box::new(rm));
        GlobalInstance::set(Arc::new(wrapper));
        Ok(())
    }
}
