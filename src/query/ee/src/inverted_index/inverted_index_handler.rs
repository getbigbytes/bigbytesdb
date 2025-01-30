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

use bigbytes_common_base::base::GlobalInstance;
use bigbytes_common_catalog::catalog::Catalog;
use bigbytes_common_exception::Result;
use bigbytes_common_meta_app::schema::CreateTableIndexReq;
use bigbytes_common_meta_app::schema::DropTableIndexReq;
use bigbytes_enterprise_inverted_index::InvertedIndexHandler;
use bigbytes_enterprise_inverted_index::InvertedIndexHandlerWrapper;

pub struct RealInvertedIndexHandler {}

#[async_trait::async_trait]
impl InvertedIndexHandler for RealInvertedIndexHandler {
    #[async_backtrace::framed]
    async fn do_create_table_index(
        &self,
        catalog: Arc<dyn Catalog>,
        req: CreateTableIndexReq,
    ) -> Result<()> {
        catalog.create_table_index(req).await
    }

    #[async_backtrace::framed]
    async fn do_drop_table_index(
        &self,
        catalog: Arc<dyn Catalog>,
        req: DropTableIndexReq,
    ) -> Result<()> {
        catalog.drop_table_index(req).await
    }
}

impl RealInvertedIndexHandler {
    pub fn init() -> Result<()> {
        let rm = RealInvertedIndexHandler {};
        let wrapper = InvertedIndexHandlerWrapper::new(Box::new(rm));
        GlobalInstance::set(Arc::new(wrapper));
        Ok(())
    }
}
