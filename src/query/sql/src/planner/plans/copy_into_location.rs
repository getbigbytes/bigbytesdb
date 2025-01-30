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
use std::fmt::Formatter;

use bigbytesdb_common_ast::ast::CopyIntoLocationOptions;
use bigbytesdb_common_expression::types::DataType;
use bigbytesdb_common_expression::types::NumberDataType;
use bigbytesdb_common_expression::DataField;
use bigbytesdb_common_expression::DataSchemaRef;
use bigbytesdb_common_expression::DataSchemaRefExt;
use bigbytesdb_common_meta_app::principal::StageInfo;

use crate::plans::Plan;

#[derive(Clone)]
pub struct CopyIntoLocationPlan {
    pub stage: Box<StageInfo>,
    pub path: String,
    pub from: Box<Plan>,
    pub options: CopyIntoLocationOptions,
}

impl CopyIntoLocationPlan {
    pub fn schema(&self) -> DataSchemaRef {
        if self.options.detailed_output {
            DataSchemaRefExt::create(vec![
                DataField::new("file_name", DataType::String),
                DataField::new("file_size", DataType::Number(NumberDataType::UInt64)),
                DataField::new("row_count", DataType::Number(NumberDataType::UInt64)),
            ])
        } else {
            DataSchemaRefExt::create(vec![
                DataField::new("rows_unloaded", DataType::Number(NumberDataType::UInt64)),
                DataField::new("input_bytes", DataType::Number(NumberDataType::UInt64)),
                DataField::new("output_bytes", DataType::Number(NumberDataType::UInt64)),
            ])
        }
    }
}

impl Debug for CopyIntoLocationPlan {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "Copy into {:?}/{} from {:?}",
            self.stage, self.path, self.from
        )?;
        Ok(())
    }
}
