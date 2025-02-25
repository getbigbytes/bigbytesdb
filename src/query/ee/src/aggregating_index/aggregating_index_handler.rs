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
use bigbytesdb_common_catalog::catalog::Catalog;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_meta_app::schema::CreateIndexReply;
use bigbytesdb_common_meta_app::schema::CreateIndexReq;
use bigbytesdb_common_meta_app::schema::DropIndexReq;
use bigbytesdb_common_meta_app::schema::GetIndexReply;
use bigbytesdb_common_meta_app::schema::GetIndexReq;
use bigbytesdb_common_meta_app::schema::UpdateIndexReply;
use bigbytesdb_common_meta_app::schema::UpdateIndexReq;
use bigbytesdb_enterprise_aggregating_index::AggregatingIndexHandler;
use bigbytesdb_enterprise_aggregating_index::AggregatingIndexHandlerWrapper;

pub struct RealAggregatingIndexHandler {}

#[async_trait::async_trait]
impl AggregatingIndexHandler for RealAggregatingIndexHandler {
    #[async_backtrace::framed]
    async fn do_create_index(
        &self,
        catalog: Arc<dyn Catalog>,
        req: CreateIndexReq,
    ) -> Result<CreateIndexReply> {
        catalog.create_index(req).await
    }

    #[async_backtrace::framed]
    async fn do_drop_index(&self, catalog: Arc<dyn Catalog>, req: DropIndexReq) -> Result<()> {
        catalog.drop_index(req).await
    }

    #[async_backtrace::framed]
    async fn do_get_index(
        &self,
        catalog: Arc<dyn Catalog>,
        req: GetIndexReq,
    ) -> Result<GetIndexReply> {
        catalog.get_index(req).await
    }

    #[async_backtrace::framed]
    async fn do_update_index(
        &self,
        catalog: Arc<dyn Catalog>,
        req: UpdateIndexReq,
    ) -> Result<UpdateIndexReply> {
        catalog.update_index(req).await
    }
}

impl RealAggregatingIndexHandler {
    pub fn init() -> Result<()> {
        let rm = RealAggregatingIndexHandler {};
        let wrapper = AggregatingIndexHandlerWrapper::new(Box::new(rm));
        GlobalInstance::set(Arc::new(wrapper));
        Ok(())
    }
}
