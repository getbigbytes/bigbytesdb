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

use bigbytes_common_expression::TableSchemaRef;

use crate::filters::xor8::xor8_filter::Xor8Filter;

/// Filters of a given DataBlock
/// `filter_schema.fields.len()` should equals `filters.len()`
pub struct BlockFilter {
    // schema of index block, chosen columns only
    pub filter_schema: TableSchemaRef,
    // filters of index block, chosen columns only
    pub filters: Vec<Arc<Xor8Filter>>,
}
