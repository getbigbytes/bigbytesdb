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

use bigbytesdb_common_ast::ast::Engine;
use bigbytesdb_common_base::base::tokio;
use bigbytesdb_common_meta_app::schema::CreateOption;
use bigbytesdb_common_sql::plans::AlterTableClusterKeyPlan;
use bigbytesdb_common_sql::plans::CreateTablePlan;
use bigbytesdb_common_sql::plans::DropTableClusterKeyPlan;
use bigbytesdb_query::interpreters::AlterTableClusterKeyInterpreter;
use bigbytesdb_query::interpreters::CreateTableInterpreter;
use bigbytesdb_query::interpreters::DropTableClusterKeyInterpreter;
use bigbytesdb_query::interpreters::Interpreter;
use bigbytesdb_query::test_kits::*;
use bigbytesdb_storages_common_table_meta::table::LINEAR_CLUSTER_TYPE;
use bigbytesdb_storages_common_table_meta::table::OPT_KEY_CLUSTER_TYPE;
use bigbytesdb_storages_common_table_meta::table::OPT_KEY_DATABASE_ID;

#[tokio::test(flavor = "multi_thread")]
async fn test_fuse_alter_table_cluster_key() -> bigbytesdb_common_exception::Result<()> {
    let fixture = TestFixture::setup().await?;
    fixture.create_default_database().await?;

    let ctx = fixture.new_query_ctx().await?;

    let create_table_plan = CreateTablePlan {
        create_option: CreateOption::Create,
        tenant: fixture.default_tenant(),
        catalog: fixture.default_catalog_name(),
        database: fixture.default_db_name(),
        table: fixture.default_table_name(),
        schema: TestFixture::default_table_schema(),
        engine: Engine::Fuse,
        engine_options: Default::default(),
        storage_params: None,
        options: [
            // database id is required for FUSE
            (OPT_KEY_DATABASE_ID.to_owned(), "1".to_owned()),
        ]
        .into(),
        field_comments: vec![],
        as_select: None,
        cluster_key: None,
        inverted_indexes: None,
    };

    // create test table
    let interpreter = CreateTableInterpreter::try_create(ctx.clone(), create_table_plan)?;
    let _ = interpreter.execute(ctx.clone()).await?;

    // add cluster key
    let alter_table_cluster_key_plan = AlterTableClusterKeyPlan {
        tenant: fixture.default_tenant(),
        catalog: fixture.default_catalog_name(),
        database: fixture.default_db_name(),
        table: fixture.default_table_name(),
        cluster_keys: vec!["id".to_string()],
        cluster_type: "linear".to_string(),
    };
    let interpreter =
        AlterTableClusterKeyInterpreter::try_create(ctx.clone(), alter_table_cluster_key_plan)?;
    let _ = interpreter.execute(ctx.clone()).await?;

    let table = fixture.latest_default_table().await?;
    let table_info = table.get_table_info();
    assert_eq!(table_info.meta.cluster_key_seq, 1);
    assert_eq!(
        table_info.meta.options.get(OPT_KEY_CLUSTER_TYPE).unwrap(),
        LINEAR_CLUSTER_TYPE
    );

    // drop cluster key
    let drop_table_cluster_key_plan = DropTableClusterKeyPlan {
        tenant: fixture.default_tenant(),
        catalog: fixture.default_catalog_name(),
        database: fixture.default_db_name(),
        table: fixture.default_table_name(),
    };
    let interpreter =
        DropTableClusterKeyInterpreter::try_create(ctx.clone(), drop_table_cluster_key_plan)?;
    let _ = interpreter.execute(ctx.clone()).await?;

    let table = fixture.latest_default_table().await?;
    let table_info = table.get_table_info();
    assert_eq!(table_info.meta.cluster_key, None);
    assert_eq!(table_info.meta.cluster_key_seq, 1);

    Ok(())
}
