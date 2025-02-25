// Copyright 2022 Digitrans Inc.
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

use bigbytesdb_common_base::base::tokio;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_meta_app::schema::database_name_ident::DatabaseNameIdent;
use bigbytesdb_common_meta_app::schema::CreateDatabaseReq;
use bigbytesdb_common_meta_app::schema::CreateOption;
use bigbytesdb_common_meta_app::schema::DropDatabaseReq;
use bigbytesdb_common_meta_app::schema::RenameDatabaseReq;
use bigbytesdb_common_meta_app::tenant::Tenant;
use bigbytesdb_query::catalogs::default::ImmutableCatalog;
use bigbytesdb_query::catalogs::Catalog;

use crate::tests::create_catalog;

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_immutable_catalogs_database() -> Result<()> {
    let tenant_name = "test";
    let tenant = Tenant::new_literal(tenant_name);

    let conf = bigbytesdb_query::test_kits::ConfigBuilder::create().config();
    let catalog = ImmutableCatalog::try_create_with_config(&conf).await?;

    // get system database
    let database = catalog.get_database(&tenant, "system").await?;
    assert_eq!(database.name(), "system");

    // get default database
    let db_2 = catalog.get_database(&tenant, "default").await;
    assert!(db_2.is_err());

    // get non-exist database
    let db_3 = catalog.get_database(&tenant, "test").await;
    assert!(db_3.is_err());

    // create database should failed
    let create_db_req = CreateDatabaseReq {
        create_option: CreateOption::Create,
        name_ident: DatabaseNameIdent::new(&tenant, "system"),
        meta: Default::default(),
    };
    let create_db_req = catalog.create_database(create_db_req).await;
    assert!(create_db_req.is_err());

    let drop_db_req = DropDatabaseReq {
        if_exists: false,
        name_ident: DatabaseNameIdent::new(&tenant, "system"),
    };
    let drop_db_req = catalog.drop_database(drop_db_req).await;
    assert!(drop_db_req.is_err());

    // rename database should failed
    let rename_db_req = RenameDatabaseReq {
        if_exists: false,
        name_ident: DatabaseNameIdent::new(&tenant, "system"),

        new_db_name: "test".to_string(),
    };
    let rename_db_req = catalog.rename_database(rename_db_req).await;
    assert!(rename_db_req.is_err());

    // rename database should failed
    let rename_db_req = RenameDatabaseReq {
        if_exists: false,
        name_ident: DatabaseNameIdent::new(&tenant, "test"),

        new_db_name: "system".to_string(),
    };
    let rename_db_req = catalog.rename_database(rename_db_req).await;
    assert!(rename_db_req.is_err());

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_immutable_catalogs_table() -> Result<()> {
    let tenant_name = "test";
    let tenant = Tenant::new_literal(tenant_name);
    let catalog = create_catalog().await?;

    let db_list_1 = catalog.list_tables(&tenant, "system").await?;
    assert!(!db_list_1.is_empty());

    let table_list = catalog.list_tables(&tenant, "default").await?;
    assert!(table_list.is_empty());

    Ok(())
}
