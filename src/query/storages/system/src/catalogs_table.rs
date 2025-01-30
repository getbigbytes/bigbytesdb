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

use bigbytes_common_catalog::catalog::CatalogManager;
use bigbytes_common_catalog::plan::PushDownInfo;
use bigbytes_common_catalog::table::Table;
use bigbytes_common_catalog::table_context::TableContext;
use bigbytes_common_exception::Result;
use bigbytes_common_expression::types::StringType;
use bigbytes_common_expression::utils::FromData;
use bigbytes_common_expression::DataBlock;
use bigbytes_common_expression::TableDataType;
use bigbytes_common_expression::TableField;
use bigbytes_common_expression::TableSchemaRefExt;
use bigbytes_common_meta_app::schema::TableIdent;
use bigbytes_common_meta_app::schema::TableInfo;
use bigbytes_common_meta_app::schema::TableMeta;

use super::table::AsyncOneBlockSystemTable;
use super::table::AsyncSystemTable;

pub struct CatalogsTable {
    table_info: TableInfo,
}

#[async_trait::async_trait]
impl AsyncSystemTable for CatalogsTable {
    const NAME: &'static str = "system.catalogs";

    fn get_table_info(&self) -> &TableInfo {
        &self.table_info
    }

    #[async_backtrace::framed]
    async fn get_full_data(
        &self,
        ctx: Arc<dyn TableContext>,
        _push_downs: Option<PushDownInfo>,
    ) -> Result<DataBlock> {
        let mgr = CatalogManager::instance();

        let catalog_names = mgr
            .list_catalogs(&ctx.get_tenant(), ctx.session_state())
            .await?
            .into_iter()
            .map(|v| v.name())
            .collect::<Vec<_>>();

        Ok(DataBlock::new_from_columns(vec![StringType::from_data(
            catalog_names,
        )]))
    }
}

impl CatalogsTable {
    pub fn create(table_id: u64) -> Arc<dyn Table> {
        let schema =
            TableSchemaRefExt::create(vec![TableField::new("name", TableDataType::String)]);

        let table_info = TableInfo {
            desc: "'system'.'catalogs'".to_string(),
            name: "catalogs".to_string(),
            ident: TableIdent::new(table_id, 0),
            meta: TableMeta {
                schema,
                engine: "SystemCatalogs".to_string(),
                ..Default::default()
            },
            ..Default::default()
        };

        AsyncOneBlockSystemTable::create(CatalogsTable { table_info })
    }
}
