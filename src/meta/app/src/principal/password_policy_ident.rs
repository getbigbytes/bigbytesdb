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

/// Defines the meta-service key for password policy.
pub type PasswordPolicyIdent = TIdent<Resource>;

pub use kvapi_impl::Resource;

mod kvapi_impl {

    use bigbytesdb_common_exception::ErrorCode;
    use bigbytesdb_common_meta_kvapi::kvapi;

    use crate::principal::PasswordPolicy;
    use crate::principal::PasswordPolicyIdent;
    use crate::tenant_key::errors::ExistError;
    use crate::tenant_key::errors::UnknownError;
    use crate::tenant_key::resource::TenantResource;

    pub struct Resource;

    impl TenantResource for Resource {
        const PREFIX: &'static str = "__fd_password_policies";
        const TYPE: &'static str = "PasswordPolicyIdent";
        const HAS_TENANT: bool = true;
        type ValueType = PasswordPolicy;
    }

    impl kvapi::Value for PasswordPolicy {
        type KeyType = PasswordPolicyIdent;
        fn dependency_keys(&self, _key: &Self::KeyType) -> impl IntoIterator<Item = String> {
            []
        }
    }

    impl kvapi::ValueWithName for PasswordPolicy {
        fn name(&self) -> &str {
            &self.name
        }
    }

    impl From<ExistError<Resource>> for ErrorCode {
        fn from(err: ExistError<Resource>) -> Self {
            ErrorCode::PasswordPolicyAlreadyExists(err.to_string())
        }
    }

    impl From<UnknownError<Resource>> for ErrorCode {
        fn from(err: UnknownError<Resource>) -> Self {
            ErrorCode::UnknownPasswordPolicy(err.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use bigbytesdb_common_meta_kvapi::kvapi::Key;

    use crate::principal::password_policy_ident::PasswordPolicyIdent;
    use crate::tenant::Tenant;
    #[test]
    fn test_password_policy_ident() {
        let tenant = Tenant::new_literal("test");
        let ident = PasswordPolicyIdent::new(tenant.clone(), "test2");

        assert_eq!(ident.to_string_key(), "__fd_password_policies/test/test2");
        assert_eq!(
            ident,
            PasswordPolicyIdent::from_str_key("__fd_password_policies/test/test2").unwrap()
        );
    }
}
