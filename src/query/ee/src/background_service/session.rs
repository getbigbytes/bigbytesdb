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

use bigbytesdb_common_config::InnerConfig;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_meta_app::principal::GrantObject;
use bigbytesdb_common_meta_app::principal::UserInfo;
use bigbytesdb_common_meta_app::principal::UserPrivilegeType;
use bigbytesdb_common_users::BUILTIN_ROLE_ACCOUNT_ADMIN;
use bigbytesdb_query::sessions::Session;
use bigbytesdb_query::sessions::SessionManager;
use bigbytesdb_query::sessions::SessionType;

pub async fn create_session(conf: &InnerConfig) -> Result<Arc<Session>> {
    let session_manager = SessionManager::instance();
    let session = session_manager
        .create_session(SessionType::FlightSQL)
        .await?;

    let session = session_manager.register_session(session)?;

    let user = get_background_service_user(conf);
    session
        .set_authed_user(user.clone(), Some(BUILTIN_ROLE_ACCOUNT_ADMIN.to_string()))
        .await?;
    Ok(session)
}

pub fn get_background_service_user(conf: &InnerConfig) -> UserInfo {
    let mut user = UserInfo::new_no_auth(
        format!(
            "{}-{}-background-svc",
            conf.query.tenant_id.tenant_name(),
            conf.query.cluster_id.clone()
        )
        .as_str(),
        "0.0.0.0",
    );
    user.grants
        .grant_privileges(&GrantObject::Global, UserPrivilegeType::Select.into());
    user
}
