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

use std::fmt::Debug;

use bigbytesdb_common_meta_app::principal::StageInfo;
use bigbytesdb_common_meta_app::schema::CreateOption;
use bigbytesdb_common_meta_app::tenant::Tenant;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CreateStagePlan {
    pub create_option: CreateOption,
    pub tenant: Tenant,
    pub stage_info: StageInfo,
}

/// Drop.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DropStagePlan {
    pub if_exists: bool,
    pub name: String,
}

/// Remove.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RemoveStagePlan {
    pub stage: StageInfo,
    pub path: String,
    pub pattern: String,
}
