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

use std::collections::HashMap;
use std::sync::Arc;

use bigbytes_common_base::base::GlobalInstance;
use bigbytes_common_config::GlobalConfig;
use bigbytes_common_exception::ErrorCode;
use bigbytes_common_exception::Result;
use bigbytes_common_management::SelectedNode;
use bigbytes_common_management::WarehouseInfo;
use bigbytes_common_meta_types::NodeInfo;
use bigbytes_common_meta_types::NodeType;

#[async_trait::async_trait]
pub trait ResourcesManagement: Sync + Send + 'static {
    fn support_forward_warehouse_request(&self) -> bool;

    async fn init_node(&self, node: &mut NodeInfo) -> Result<()>;

    async fn create_warehouse(
        &self,
        name: String,
        nodes: Vec<SelectedNode>,
    ) -> Result<WarehouseInfo>;

    async fn drop_warehouse(&self, name: String) -> Result<WarehouseInfo>;

    async fn resume_warehouse(&self, name: String) -> Result<()>;

    async fn suspend_warehouse(&self, name: String) -> Result<()>;

    async fn rename_warehouse(&self, name: String, to: String) -> Result<()>;

    async fn inspect_warehouse(&self, name: String) -> Result<Vec<NodeInfo>>;

    async fn list_warehouses(&self) -> Result<Vec<WarehouseInfo>>;

    async fn add_warehouse_cluster(
        &self,
        name: String,
        cluster: String,
        nodes: Vec<SelectedNode>,
    ) -> Result<()>;

    async fn rename_warehouse_cluster(
        &self,
        name: String,
        cluster: String,
        new_cluster: String,
    ) -> Result<()>;

    async fn drop_warehouse_cluster(&self, name: String, cluster: String) -> Result<()>;

    async fn assign_warehouse_nodes(
        &self,
        name: String,
        nodes: HashMap<String, Vec<SelectedNode>>,
    ) -> Result<()>;

    async fn unassign_warehouse_nodes(
        &self,
        name: String,
        nodes: HashMap<String, Vec<SelectedNode>>,
    ) -> Result<()>;

    async fn list_online_nodes(&self) -> Result<Vec<NodeInfo>>;
}

pub struct DummyResourcesManagement;

#[async_trait::async_trait]
impl ResourcesManagement for DummyResourcesManagement {
    fn support_forward_warehouse_request(&self) -> bool {
        false
    }

    async fn init_node(&self, node: &mut NodeInfo) -> Result<()> {
        let config = GlobalConfig::instance();
        node.cluster_id = config.query.cluster_id.clone();
        node.warehouse_id = config.query.cluster_id.clone();
        node.node_type = NodeType::SelfManaged;
        Ok(())
    }

    async fn create_warehouse(&self, _: String, _: Vec<SelectedNode>) -> Result<WarehouseInfo> {
        Err(ErrorCode::Unimplemented("The use of this feature requires a Bigbytes Enterprise Edition license. To unlock enterprise features, please contact Bigbytes to obtain a license. Learn more at https://docs.bigbytes.com/guides/overview/editions/dee/"))
    }

    async fn drop_warehouse(&self, _: String) -> Result<WarehouseInfo> {
        Err(ErrorCode::Unimplemented("The use of this feature requires a Bigbytes Enterprise Edition license. To unlock enterprise features, please contact Bigbytes to obtain a license. Learn more at https://docs.bigbytes.com/guides/overview/editions/dee/"))
    }

    async fn resume_warehouse(&self, _: String) -> Result<()> {
        Err(ErrorCode::Unimplemented("The use of this feature requires a Bigbytes Enterprise Edition license. To unlock enterprise features, please contact Bigbytes to obtain a license. Learn more at https://docs.bigbytes.com/guides/overview/editions/dee/"))
    }

    async fn suspend_warehouse(&self, _: String) -> Result<()> {
        Err(ErrorCode::Unimplemented("The use of this feature requires a Bigbytes Enterprise Edition license. To unlock enterprise features, please contact Bigbytes to obtain a license. Learn more at https://docs.bigbytes.com/guides/overview/editions/dee/"))
    }

    async fn rename_warehouse(&self, _: String, _: String) -> Result<()> {
        Err(ErrorCode::Unimplemented("The use of this feature requires a Bigbytes Enterprise Edition license. To unlock enterprise features, please contact Bigbytes to obtain a license. Learn more at https://docs.bigbytes.com/guides/overview/editions/dee/"))
    }

    async fn inspect_warehouse(&self, _: String) -> Result<Vec<NodeInfo>> {
        Err(ErrorCode::Unimplemented("The use of this feature requires a Bigbytes Enterprise Edition license. To unlock enterprise features, please contact Bigbytes to obtain a license. Learn more at https://docs.bigbytes.com/guides/overview/editions/dee/"))
    }

    async fn list_warehouses(&self) -> Result<Vec<WarehouseInfo>> {
        Err(ErrorCode::Unimplemented("The use of this feature requires a Bigbytes Enterprise Edition license. To unlock enterprise features, please contact Bigbytes to obtain a license. Learn more at https://docs.bigbytes.com/guides/overview/editions/dee/"))
    }

    async fn add_warehouse_cluster(
        &self,
        _: String,
        _: String,
        _: Vec<SelectedNode>,
    ) -> Result<()> {
        Err(ErrorCode::Unimplemented("The use of this feature requires a Bigbytes Enterprise Edition license. To unlock enterprise features, please contact Bigbytes to obtain a license. Learn more at https://docs.bigbytes.com/guides/overview/editions/dee/"))
    }

    async fn rename_warehouse_cluster(&self, _: String, _: String, _: String) -> Result<()> {
        Err(ErrorCode::Unimplemented("The use of this feature requires a Bigbytes Enterprise Edition license. To unlock enterprise features, please contact Bigbytes to obtain a license. Learn more at https://docs.bigbytes.com/guides/overview/editions/dee/"))
    }

    async fn drop_warehouse_cluster(&self, _: String, _: String) -> Result<()> {
        Err(ErrorCode::Unimplemented("The use of this feature requires a Bigbytes Enterprise Edition license. To unlock enterprise features, please contact Bigbytes to obtain a license. Learn more at https://docs.bigbytes.com/guides/overview/editions/dee/"))
    }

    async fn assign_warehouse_nodes(
        &self,
        _: String,
        _: HashMap<String, Vec<SelectedNode>>,
    ) -> Result<()> {
        Err(ErrorCode::Unimplemented("The use of this feature requires a Bigbytes Enterprise Edition license. To unlock enterprise features, please contact Bigbytes to obtain a license. Learn more at https://docs.bigbytes.com/guides/overview/editions/dee/"))
    }

    async fn unassign_warehouse_nodes(
        &self,
        _: String,
        _: HashMap<String, Vec<SelectedNode>>,
    ) -> Result<()> {
        Err(ErrorCode::Unimplemented("The use of this feature requires a Bigbytes Enterprise Edition license. To unlock enterprise features, please contact Bigbytes to obtain a license. Learn more at https://docs.bigbytes.com/guides/overview/editions/dee/"))
    }

    async fn list_online_nodes(&self) -> Result<Vec<NodeInfo>> {
        Err(ErrorCode::Unimplemented("The use of this feature requires a Bigbytes Enterprise Edition license. To unlock enterprise features, please contact Bigbytes to obtain a license. Learn more at https://docs.bigbytes.com/guides/overview/editions/dee/"))
    }
}

impl DummyResourcesManagement {
    pub fn init() -> Result<()> {
        let instance: Arc<dyn ResourcesManagement> = Arc::new(DummyResourcesManagement);
        GlobalInstance::set(instance);
        Ok(())
    }
}
