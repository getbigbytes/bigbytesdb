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

use bigbytesdb_common_catalog::plan::StageTableInfo;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_expression::DataSchemaRef;
use bigbytesdb_common_expression::DataSchemaRefExt;
use bigbytesdb_common_expression::Scalar;
use bigbytesdb_common_meta_app::schema::TableInfo;
use enum_as_inner::EnumAsInner;

use crate::executor::physical_plan::PhysicalPlan;
use crate::plans::CopyIntoTableMode;
use crate::plans::ValidationMode;
use crate::ColumnBinding;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CopyIntoTable {
    pub plan_id: u32,

    pub required_values_schema: DataSchemaRef,
    pub values_consts: Vec<Scalar>,
    pub required_source_schema: DataSchemaRef,
    pub write_mode: CopyIntoTableMode,
    pub validation_mode: ValidationMode,
    pub stage_table_info: StageTableInfo,
    pub table_info: TableInfo,

    pub project_columns: Option<Vec<ColumnBinding>>,
    pub source: CopyIntoTableSource,
    pub is_transform: bool,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, EnumAsInner)]
pub enum CopyIntoTableSource {
    Query(Box<PhysicalPlan>),
    Stage(Box<PhysicalPlan>),
}

impl CopyIntoTable {
    pub fn output_schema(&self) -> Result<DataSchemaRef> {
        Ok(DataSchemaRefExt::create(vec![]))
    }
}
