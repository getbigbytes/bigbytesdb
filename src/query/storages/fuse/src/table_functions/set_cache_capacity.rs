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

use bigbytesdb_common_catalog::plan::DataSourcePlan;
use bigbytesdb_common_catalog::table::DistributionLevel;
use bigbytesdb_common_catalog::table_context::TableContext;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_expression::types::StringType;
use bigbytesdb_common_expression::DataBlock;
use bigbytesdb_common_expression::FromData;
use bigbytesdb_common_expression::TableDataType;
use bigbytesdb_common_expression::TableField;
use bigbytesdb_common_expression::TableSchemaRef;
use bigbytesdb_common_expression::TableSchemaRefExt;
use bigbytesdb_storages_common_cache::CacheManager;

use crate::table_functions::string_literal;
use crate::table_functions::string_value;
use crate::table_functions::SimpleTableFunc;
use crate::table_functions::TableArgs;

#[derive(Clone)]
pub struct SetCapacity {
    cache_name: String,
    capacity: u64,
}

impl From<&SetCapacity> for TableArgs {
    fn from(value: &SetCapacity) -> Self {
        TableArgs::new_positioned(vec![
            string_literal(&value.cache_name),
            string_literal(&value.capacity.to_string()),
        ])
    }
}

pub struct SetCacheCapacity {
    operation: SetCapacity,
}
#[async_trait::async_trait]
impl SimpleTableFunc for SetCacheCapacity {
    fn table_args(&self) -> Option<TableArgs> {
        Some((&self.operation).into())
    }

    fn schema(&self) -> TableSchemaRef {
        TableSchemaRefExt::create(vec![
            TableField::new("node", TableDataType::String),
            TableField::new("result", TableDataType::String),
        ])
    }

    fn distribution_level(&self) -> DistributionLevel {
        DistributionLevel::Warehouse
    }

    async fn apply(
        &self,
        ctx: &Arc<dyn TableContext>,
        _plan: &DataSourcePlan,
    ) -> Result<Option<DataBlock>> {
        let cache_mgr = CacheManager::instance();
        let op = &self.operation;
        cache_mgr.set_cache_capacity(&op.cache_name, op.capacity)?;

        let node = vec![ctx.get_cluster().local_id.clone()];
        let res = vec!["Ok".to_owned()];

        Ok(Some(DataBlock::new_from_columns(vec![
            StringType::from_data(node),
            StringType::from_data(res),
        ])))
    }

    fn create(_func_name: &str, table_args: TableArgs) -> Result<Self>
    where Self: Sized {
        let args = table_args.expect_all_positioned("", Some(2))?;
        let cache_name = string_value(&args[0])?;
        let capacity = string_value(&args[1])?.parse::<u64>()?;

        let operation = SetCapacity {
            cache_name,
            capacity,
        };
        Ok(Self { operation })
    }
}
