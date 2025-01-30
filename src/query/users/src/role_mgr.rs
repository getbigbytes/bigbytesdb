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

use bigbytesdb_common_exception::ErrorCode;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_management::RoleApi;
use bigbytesdb_common_meta_app::principal::GrantObject;
use bigbytesdb_common_meta_app::principal::OwnershipInfo;
use bigbytesdb_common_meta_app::principal::OwnershipObject;
use bigbytesdb_common_meta_app::principal::RoleInfo;
use bigbytesdb_common_meta_app::principal::UserPrivilegeSet;
use bigbytesdb_common_meta_app::tenant::Tenant;
use bigbytesdb_common_meta_types::MatchSeq;

use crate::role_util::find_all_related_roles;
use crate::UserApiProvider;

pub const BUILTIN_ROLE_ACCOUNT_ADMIN: &str = "account_admin";
pub const BUILTIN_ROLE_PUBLIC: &str = "public";

impl UserApiProvider {
    // Get one role from by tenant.
    #[async_backtrace::framed]
    pub async fn get_role(&self, tenant: &Tenant, role: String) -> Result<RoleInfo> {
        let builtin_roles = self.builtin_roles();
        if let Some(role_info) = builtin_roles.get(&role) {
            return Ok(role_info.clone());
        }

        let client = self.role_api(tenant);
        let role_data = client.get_role(&role, MatchSeq::GE(0)).await?.data;
        Ok(role_data)
    }

    // Get the tenant all roles list.
    #[async_backtrace::framed]
    pub async fn get_roles(&self, tenant: &Tenant) -> Result<Vec<RoleInfo>> {
        let builtin_roles = self.builtin_roles();
        let seq_roles = self
            .role_api(tenant)
            .get_meta_roles()
            .await
            .map_err(|e| e.add_message_back("(while get roles)."))?;
        // overwrite the builtin roles.
        let mut roles = seq_roles
            .into_iter()
            .map(|r| r.data)
            .filter(|r| !builtin_roles.contains_key(&r.name))
            .collect::<Vec<_>>();
        roles.extend(builtin_roles.values().cloned());
        roles.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(roles)
    }

    // Currently we have to built account_admin role in query:
    // 1. ACCOUNT_ADMIN, which has the equivalent privileges of `GRANT ALL ON *.* TO ROLE account_admin`,
    //    it also contains all roles. ACCOUNT_ADMIN can access the data objects which owned by any role.
    // 2. Because the previous deserialization using from_bits caused forward compatibility issues after adding new privilege type
    //    Until we can confirm all product use https://github.com/getbigbytes/bigbytesdb/releases/tag/v1.2.321-nightly or later,
    //    We can add account_admin into meta.
    fn builtin_roles(&self) -> HashMap<String, RoleInfo> {
        let mut account_admin = RoleInfo::new(BUILTIN_ROLE_ACCOUNT_ADMIN);
        account_admin.grants.grant_privileges(
            &GrantObject::Global,
            UserPrivilegeSet::available_privileges_on_global(),
        );

        let mut result = HashMap::new();
        result.insert(BUILTIN_ROLE_ACCOUNT_ADMIN.into(), account_admin);
        result
    }

    #[async_backtrace::framed]
    pub async fn get_ownerships(
        &self,
        tenant: &Tenant,
    ) -> Result<HashMap<OwnershipObject, String>> {
        let seq_owns = self
            .role_api(tenant)
            .get_ownerships()
            .await
            .map_err(|e| e.add_message_back("(while get ownerships)."))?;

        let roles: HashMap<OwnershipObject, String> = seq_owns
            .into_iter()
            .map(|r| (r.data.object, r.data.role))
            .collect();
        Ok(roles)
    }

    #[async_backtrace::framed]
    pub async fn exists_role(&self, tenant: &Tenant, role: String) -> Result<bool> {
        match self.get_role(tenant, role).await {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.code() == ErrorCode::UNKNOWN_ROLE {
                    Ok(false)
                } else {
                    Err(e)
                }
            }
        }
    }

    // Add a new role info.
    #[async_backtrace::framed]
    pub async fn add_role(
        &self,
        tenant: &Tenant,
        role_info: RoleInfo,
        if_not_exists: bool,
    ) -> Result<u64> {
        if if_not_exists && self.exists_role(tenant, role_info.name.clone()).await? {
            return Ok(0);
        }

        let client = self.role_api(tenant);
        let add_role = client.add_role(role_info);
        match add_role.await {
            Ok(res) => Ok(res),
            Err(e) => {
                if if_not_exists && e.code() == ErrorCode::ROLE_ALREADY_EXISTS {
                    Ok(0)
                } else {
                    Err(e.add_message_back("(while add role)"))
                }
            }
        }
    }

    #[async_backtrace::framed]
    pub async fn grant_ownership_to_role(
        &self,
        tenant: &Tenant,
        object: &OwnershipObject,
        new_role: &str,
    ) -> Result<()> {
        // from and to role must exists
        self.get_role(tenant, new_role.to_string()).await?;

        let client = self.role_api(tenant);
        client
            .grant_ownership(object, new_role)
            .await
            .map_err(|e| e.add_message_back("(while set role ownership)"))
    }

    #[async_backtrace::framed]
    pub async fn get_ownership(
        &self,
        tenant: &Tenant,
        object: &OwnershipObject,
    ) -> Result<Option<OwnershipInfo>> {
        let client = self.role_api(tenant);
        let ownership = client
            .get_ownership(object)
            .await
            .map_err(|e| e.add_message_back("(while get ownership)"))?;
        if let Some(owner) = ownership {
            // if object has ownership, but the owner role is not exists, set owner role to ACCOUNT_ADMIN,
            // only account_admin can access this object.
            // Note: get_ownerships no need to do this check.
            // Because this can cause system.table queries to slow down
            // The intention is that the account admin can grant ownership.
            // So system.tables will display dropped role. It's by design.
            if !self.exists_role(tenant, owner.role.clone()).await? {
                Ok(Some(OwnershipInfo {
                    role: BUILTIN_ROLE_ACCOUNT_ADMIN.to_string(),
                    object: object.clone(),
                }))
            } else {
                Ok(Some(owner))
            }
        } else {
            Ok(None)
        }
    }

    #[async_backtrace::framed]
    pub async fn grant_privileges_to_role(
        &self,
        tenant: &Tenant,
        role: &String,
        object: GrantObject,
        privileges: UserPrivilegeSet,
    ) -> Result<Option<u64>> {
        let client = self.role_api(tenant);
        // Enforces warehouse exclusivity: a warehouse’s specific permission can only be granted to a single role.
        // This logic verifies that no other role within the given tenant currently holds the target permission on the specified warehouse.
        if let GrantObject::Warehouse(w) = &object {
            let roles = self.get_roles(tenant).await?;
            let mut illegal_role = String::new();
            let has_same_warehouse_grant = roles.iter().any(|r| {
                let has_grant = r.grants.entries().iter().any(|grant| match grant.object() {
                    GrantObject::Warehouse(rw) => {
                        if w.to_lowercase() == rw.to_lowercase() {
                            illegal_role = r.identity().to_string();
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                });
                has_grant
            });
            if has_same_warehouse_grant {
                return Err(ErrorCode::IllegalRole(format!(
                    "Error granting '{privileges}' access on data warehouse '{w}' to role '{role}'. Role '{illegal_role}' already possesses this access"
                )));
            }
        }
        client
            .update_role_with(role, MatchSeq::GE(1), |ri: &mut RoleInfo| {
                ri.update_role_time();
                ri.grants.grant_privileges(&object, privileges)
            })
            .await
            .map_err(|e| e.add_message_back("(while set role privileges)"))
    }

    #[async_backtrace::framed]
    pub async fn revoke_privileges_from_role(
        &self,
        tenant: &Tenant,
        role: &String,
        object: GrantObject,
        privileges: UserPrivilegeSet,
    ) -> Result<Option<u64>> {
        let client = self.role_api(tenant);
        client
            .update_role_with(role, MatchSeq::GE(1), |ri: &mut RoleInfo| {
                ri.update_role_time();
                ri.grants.revoke_privileges(&object, privileges)
            })
            .await
            .map_err(|e| e.add_message_back("(while revoke role privileges)"))
    }

    // the grant_role can not have cycle with target_role.
    #[async_backtrace::framed]
    pub async fn grant_role_to_role(
        &self,
        tenant: &Tenant,
        target_role: &String,
        grant_role: String,
    ) -> Result<Option<u64>> {
        let related_roles = self
            .find_related_roles(tenant, &[grant_role.clone()])
            .await?;
        let have_cycle = related_roles.iter().any(|r| r.identity() == target_role);
        if have_cycle {
            return Err(ErrorCode::InvalidRole(format!(
                "{} contains {}, can not be grant to {}",
                &grant_role, &target_role, &target_role
            )));
        }

        let client = self.role_api(tenant);
        client
            .update_role_with(target_role, MatchSeq::GE(1), |ri: &mut RoleInfo| {
                ri.update_role_time();
                ri.grants.grant_role(grant_role)
            })
            .await
            .map_err(|e| e.add_message_back("(while grant role to role)"))
    }

    #[async_backtrace::framed]
    pub async fn revoke_role_from_role(
        &self,
        tenant: &Tenant,
        role: &String,
        revoke_role: &String,
    ) -> Result<Option<u64>> {
        let client = self.role_api(tenant);
        client
            .update_role_with(role, MatchSeq::GE(1), |ri: &mut RoleInfo| {
                ri.update_role_time();
                ri.grants.revoke_role(revoke_role)
            })
            .await
            .map_err(|e| e.add_message_back("(while revoke role from role)"))
    }

    // Drop a role by name
    #[async_backtrace::framed]
    pub async fn drop_role(&self, tenant: &Tenant, role: String, if_exists: bool) -> Result<()> {
        let client = self.role_api(tenant);
        // If the dropped role owns objects, transfer objects owner to account_admin role.
        client.transfer_ownership_to_admin(&role).await?;

        let drop_role = client.drop_role(role, MatchSeq::GE(1));
        match drop_role.await {
            Ok(res) => Ok(res),
            Err(e) => {
                if if_exists && e.code() == ErrorCode::UNKNOWN_ROLE {
                    Ok(())
                } else {
                    Err(e.add_message_back("(while set drop role)"))
                }
            }
        }
    }

    // Find all related roles by role names. Every role have a PUBLIC role, and ACCOUNT_ADMIN
    // default contains every role.
    #[async_backtrace::framed]
    async fn find_related_roles(
        &self,
        tenant: &Tenant,
        role_identities: &[String],
    ) -> Result<Vec<RoleInfo>> {
        let tenant_roles_map = self
            .get_roles(tenant)
            .await?
            .into_iter()
            .map(|r| (r.identity().to_string(), r))
            .collect::<HashMap<_, _>>();
        Ok(find_all_related_roles(&tenant_roles_map, role_identities))
    }
}
