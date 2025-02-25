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

use std::sync::Arc;

use bigbytesdb_common_ast::ast::CreateOption;
use bigbytesdb_common_expression::DataSchema;
use bigbytesdb_common_expression::DataSchemaRef;
use bigbytesdb_common_meta_app::schema::DictionaryMeta;
use bigbytesdb_common_meta_app::tenant::Tenant;

#[derive(Clone, Debug)]
pub struct CreateDictionaryPlan {
    pub create_option: CreateOption,
    pub tenant: Tenant,
    pub catalog: String,
    pub database_id: u64,
    pub dictionary: String,
    pub meta: DictionaryMeta,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DropDictionaryPlan {
    pub if_exists: bool,
    pub tenant: Tenant,
    pub catalog: String,
    pub database_id: u64,
    pub dictionary: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ShowCreateDictionaryPlan {
    pub catalog: String,
    pub database_id: u64,
    pub dictionary: String,
    pub schema: DataSchemaRef,
}

impl ShowCreateDictionaryPlan {
    pub fn schema(&self) -> DataSchemaRef {
        self.schema.clone()
    }
}

/// Rename.
#[derive(Clone, Debug)]
pub struct RenameDictionaryPlan {
    pub tenant: Tenant,
    pub if_exists: bool,
    pub catalog: String,
    pub database_id: u64,
    pub dictionary: String,
    pub new_database_id: u64,
    pub new_dictionary: String,
}

impl RenameDictionaryPlan {
    pub fn schema(&self) -> DataSchemaRef {
        Arc::new(DataSchema::empty())
    }
}
