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

use bigbytes_common_expression::TableSchema;

use crate::plan::StageTableInfo;

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct OrcTableInfo {
    pub stage_table_info: StageTableInfo,
    pub arrow_schema: arrow_schema::SchemaRef,
    pub schema_from: String,
}

impl OrcTableInfo {
    pub fn schema(&self) -> Arc<TableSchema> {
        self.stage_table_info.schema()
    }

    pub fn desc(&self) -> String {
        self.stage_table_info.desc()
    }
}
