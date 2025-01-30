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

use crate::tenant_key::ident::TIdent;

/// Defines the meta-service key for network policy.
pub type NetworkPolicyIdent = TIdent<Resource>;

pub use kvapi_impl::Resource;

mod kvapi_impl {

    use bigbytesdb_common_exception::ErrorCode;
    use bigbytesdb_common_meta_kvapi::kvapi;

    use crate::principal::NetworkPolicy;
    use crate::principal::NetworkPolicyIdent;
    use crate::tenant_key::errors::ExistError;
    use crate::tenant_key::errors::UnknownError;
    use crate::tenant_key::resource::TenantResource;

    pub struct Resource;
    impl TenantResource for Resource {
        const PREFIX: &'static str = "__fd_network_policies";
        const TYPE: &'static str = "NetworkPolicyIdent";
        const HAS_TENANT: bool = true;
        type ValueType = NetworkPolicy;
    }

    impl kvapi::Value for NetworkPolicy {
        type KeyType = NetworkPolicyIdent;
        fn dependency_keys(&self, _key: &Self::KeyType) -> impl IntoIterator<Item = String> {
            []
        }
    }

    impl kvapi::ValueWithName for NetworkPolicy {
        fn name(&self) -> &str {
            &self.name
        }
    }

    impl From<ExistError<Resource>> for ErrorCode {
        fn from(err: ExistError<Resource>) -> Self {
            ErrorCode::NetworkPolicyAlreadyExists(err.to_string())
        }
    }

    impl From<UnknownError<Resource>> for ErrorCode {
        fn from(err: UnknownError<Resource>) -> Self {
            ErrorCode::UnknownNetworkPolicy(err.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use bigbytesdb_common_meta_kvapi::kvapi::Key;

    use crate::principal::network_policy_ident::NetworkPolicyIdent;
    use crate::tenant::Tenant;

    #[test]
    fn test_network_policy_ident() {
        let tenant = Tenant::new_literal("test");
        let ident = NetworkPolicyIdent::new(tenant, "test1");

        let key = ident.to_string_key();
        assert_eq!(key, "__fd_network_policies/test/test1");

        assert_eq!(ident, NetworkPolicyIdent::from_str_key(&key).unwrap());
    }
}
