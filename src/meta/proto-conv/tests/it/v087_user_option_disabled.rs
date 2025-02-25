// Copyright 2023 Digitrans Inc.
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

use fastrace::func_name;

use crate::common;

// These bytes are built when a new version in introduced,
// and are kept for backward compatibility test.
//
// *************************************************************
// * These messages should never be updated,                   *
// * only be added when a new version is added,                *
// * or be removed when an old version is no longer supported. *
// *************************************************************
//
// The message bytes are built from the output of `test_pb_from_to()`
#[test]
fn test_decode_v87_user_iption() -> anyhow::Result<()> {
    let user_option_v87 = vec![
        8, 1, 18, 5, 114, 111, 108, 101, 49, 26, 8, 109, 121, 112, 111, 108, 105, 99, 121, 40, 1,
        160, 6, 87, 168, 6, 24,
    ];

    let want = || {
        bigbytesdb_common_meta_app::principal::UserOption::default()
            .with_set_flag(bigbytesdb_common_meta_app::principal::UserOptionFlag::TenantSetting)
            .with_default_role(Some("role1".into()))
            .with_network_policy(Some("mypolicy".to_string()))
            .with_disabled(Some(true))
    };

    common::test_pb_from_to(func_name!(), want())?;
    common::test_load_old(func_name!(), user_option_v87.as_slice(), 87, want())
}
