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

use bigbytesdb_common_catalog::table::Table;
use bigbytesdb_common_catalog::table_context::TableContext;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_expression::types::StringType;
use bigbytesdb_common_expression::utils::FromData;
use bigbytesdb_common_expression::DataBlock;
use bigbytesdb_common_expression::TableDataType;
use bigbytesdb_common_expression::TableField;
use bigbytesdb_common_expression::TableSchemaRefExt;
use bigbytesdb_common_meta_app::schema::TableIdent;
use bigbytesdb_common_meta_app::schema::TableInfo;
use bigbytesdb_common_meta_app::schema::TableMeta;

use crate::SyncOneBlockSystemTable;
use crate::SyncSystemTable;

pub struct ContributorsTable {
    table_info: TableInfo,
}

impl SyncSystemTable for ContributorsTable {
    const NAME: &'static str = "system.contributors";

    fn get_table_info(&self) -> &TableInfo {
        &self.table_info
    }

    fn get_full_data(&self, _: Arc<dyn TableContext>) -> Result<DataBlock> {
        let contributors: Vec<String> = env!("BIGBYTESDB_COMMIT_AUTHORS")
            .split_terminator(',')
            .map(|x| x.trim().to_string())
            .collect();

        Ok(DataBlock::new_from_columns(vec![StringType::from_data(
            contributors,
        )]))
    }
}

impl ContributorsTable {
    pub fn create(table_id: u64) -> Arc<dyn Table> {
        let schema =
            TableSchemaRefExt::create(vec![TableField::new("name", TableDataType::String)]);

        let table_info = TableInfo {
            desc: "'system'.'contributors'".to_string(),
            name: "contributors".to_string(),
            ident: TableIdent::new(table_id, 0),
            meta: TableMeta {
                schema,
                engine: "SystemContributors".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        SyncOneBlockSystemTable::create(ContributorsTable { table_info })
    }
}
