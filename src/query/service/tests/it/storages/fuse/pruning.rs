//  Copyright 2024 Digitrans Inc.
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

use std::sync::Arc;

use bigbytesdb_common_ast::ast::Engine;
use bigbytesdb_common_base::base::tokio;
use bigbytesdb_common_catalog::plan::PushDownInfo;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_expression::types::number::Int64Type;
use bigbytesdb_common_expression::types::number::UInt64Type;
use bigbytesdb_common_expression::types::ArgType;
use bigbytesdb_common_expression::types::NumberDataType;
use bigbytesdb_common_expression::DataBlock;
use bigbytesdb_common_expression::FromData;
use bigbytesdb_common_expression::RemoteExpr;
use bigbytesdb_common_expression::TableDataType;
use bigbytesdb_common_expression::TableField;
use bigbytesdb_common_expression::TableSchemaRef;
use bigbytesdb_common_expression::TableSchemaRefExt;
use bigbytesdb_common_meta_app::schema::CreateOption;
use bigbytesdb_common_sql::parse_to_filters;
use bigbytesdb_common_sql::plans::CreateTablePlan;
use bigbytesdb_common_sql::BloomIndexColumns;
use bigbytesdb_common_storages_fuse::pruning::create_segment_location_vector;
use bigbytesdb_common_storages_fuse::pruning::FusePruner;
use bigbytesdb_common_storages_fuse::FuseStorageFormat;
use bigbytesdb_common_storages_fuse::FuseTable;
use bigbytesdb_query::interpreters::CreateTableInterpreter;
use bigbytesdb_query::interpreters::Interpreter;
use bigbytesdb_query::sessions::QueryContext;
use bigbytesdb_query::sessions::TableContext;
use bigbytesdb_query::storages::fuse::io::MetaReaders;
use bigbytesdb_query::storages::fuse::FUSE_OPT_KEY_BLOCK_PER_SEGMENT;
use bigbytesdb_query::storages::fuse::FUSE_OPT_KEY_ROW_PER_BLOCK;
use bigbytesdb_query::test_kits::*;
use bigbytesdb_storages_common_cache::LoadParams;
use bigbytesdb_storages_common_table_meta::meta::BlockMeta;
use bigbytesdb_storages_common_table_meta::meta::TableSnapshot;
use bigbytesdb_storages_common_table_meta::meta::Versioned;
use bigbytesdb_storages_common_table_meta::table::OPT_KEY_DATABASE_ID;
use bigbytesdb_storages_common_table_meta::table::OPT_KEY_SNAPSHOT_LOCATION;
use opendal::Operator;

async fn apply_block_pruning(
    table_snapshot: Arc<TableSnapshot>,
    schema: TableSchemaRef,
    push_down: &Option<PushDownInfo>,
    ctx: Arc<QueryContext>,
    op: Operator,
    bloom_index_cols: BloomIndexColumns,
) -> Result<Vec<Arc<BlockMeta>>> {
    let ctx: Arc<dyn TableContext> = ctx;
    let segment_locs = table_snapshot.segments.clone();
    let segment_locs = create_segment_location_vector(segment_locs, None);
    FusePruner::create(
        &ctx,
        op,
        schema,
        push_down,
        bloom_index_cols,
        None,
        FuseStorageFormat::Parquet,
    )?
    .read_pruning(segment_locs)
    .await
    .map(|v| v.into_iter().map(|(_, v)| v).collect())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_block_pruner() -> Result<()> {
    let fixture = TestFixture::setup().await?;
    let ctx = fixture.new_query_ctx().await?;

    fixture.create_default_database().await?;

    let test_tbl_name = "test_index_helper";
    let test_schema = TableSchemaRefExt::create(vec![
        TableField::new("a", TableDataType::Number(NumberDataType::UInt64)),
        TableField::new("b", TableDataType::Number(NumberDataType::UInt64)),
    ]);

    let num_blocks = 10;
    let row_per_block = 10;
    let num_blocks_opt = row_per_block.to_string();

    // create test table
    let create_table_plan = CreateTablePlan {
        catalog: "default".to_owned(),
        create_option: CreateOption::Create,
        tenant: fixture.default_tenant(),
        database: fixture.default_db_name(),
        table: test_tbl_name.to_string(),
        schema: test_schema.clone(),
        engine: Engine::Fuse,
        engine_options: Default::default(),
        storage_params: None,
        options: [
            (FUSE_OPT_KEY_ROW_PER_BLOCK.to_owned(), num_blocks_opt),
            (FUSE_OPT_KEY_BLOCK_PER_SEGMENT.to_owned(), "1".to_owned()),
            (OPT_KEY_DATABASE_ID.to_owned(), "1".to_owned()),
        ]
        .into(),
        field_comments: vec![],
        as_select: None,
        cluster_key: None,
        inverted_indexes: None,
    };

    let interpreter = CreateTableInterpreter::try_create(ctx.clone(), create_table_plan)?;
    let _ = interpreter.execute(ctx.clone()).await?;

    // get table
    let catalog = ctx.get_catalog("default").await?;
    let table = catalog
        .get_table(
            &fixture.default_tenant(),
            fixture.default_db_name().as_str(),
            test_tbl_name,
        )
        .await?;

    let gen_col = |value, rows| {
        UInt64Type::from_data(std::iter::repeat(value).take(rows).collect::<Vec<u64>>())
    };

    // prepare test blocks
    // - there will be `num_blocks` blocks, for each block, it comprises of `row_per_block` rows,
    //    in our case, there will be 10 blocks, and 10 rows for each block
    let blocks = (0..num_blocks)
        .map(|idx| {
            DataBlock::new_from_columns(vec![
                // value of column a always equals  1
                gen_col(1, row_per_block),
                // for column b
                // - for all block `B` in blocks, whose index is `i`
                // - for all row in `B`, value of column b  equals `i`
                gen_col(idx as u64, row_per_block),
            ])
        })
        .collect::<Vec<_>>();

    fixture
        .append_commit_blocks(table.clone(), blocks, false, true)
        .await?;

    // get the latest tbl
    let table = catalog
        .get_table(
            &fixture.default_tenant(),
            fixture.default_db_name().as_str(),
            test_tbl_name,
        )
        .await?;

    let fuse_table = FuseTable::try_from_table(table.as_ref())?;

    let snapshot_loc = table
        .get_table_info()
        .options()
        .get(OPT_KEY_SNAPSHOT_LOCATION)
        .unwrap();

    let reader = MetaReaders::table_snapshot_reader(fuse_table.get_operator());

    let load_params = LoadParams {
        location: snapshot_loc.clone(),
        len_hint: None,
        ver: TableSnapshot::VERSION,
        put_cache: false,
    };

    let snapshot = reader.read(&load_params).await?;

    // nothing is pruned
    let e1 = PushDownInfo {
        filters: Some(parse_to_filters(ctx.clone(), table.clone(), "a > 3")?),
        ..Default::default()
    };

    // some blocks pruned
    let mut e2 = PushDownInfo::default();
    let max_val_of_b = 6u64;

    e2.filters = Some(parse_to_filters(
        ctx.clone(),
        table.clone(),
        "a > 0 and b > 6",
    )?);
    let b2 = num_blocks - max_val_of_b as usize - 1;

    // Sort asc Limit: TopN-pruner.
    let e3 = PushDownInfo {
        order_by: vec![(
            RemoteExpr::ColumnRef {
                span: None,
                id: "b".to_string(),
                data_type: Int64Type::data_type(),
                display_name: "b".to_string(),
            },
            true,
            false,
        )],
        limit: Some(3),
        ..Default::default()
    };

    // Sort desc Limit: TopN-pruner.
    let e4 = PushDownInfo {
        order_by: vec![(
            RemoteExpr::ColumnRef {
                span: None,
                id: "b".to_string(),
                data_type: Int64Type::data_type(),
                display_name: "b".to_string(),
            },
            false,
            false,
        )],
        limit: Some(4),
        ..Default::default()
    };

    // Limit push-down, Limit-pruner.
    let e5 = PushDownInfo {
        order_by: vec![],
        limit: Some(11),
        ..Default::default()
    };

    let extras = vec![
        (None, num_blocks, num_blocks * row_per_block),
        (Some(e1), 0, 0),
        (Some(e2), b2, b2 * row_per_block),
        (Some(e3), 3, 3 * row_per_block),
        (Some(e4), 4, 4 * row_per_block),
        (Some(e5), 2, 2 * row_per_block),
    ];

    for (extra, expected_blocks, expected_rows) in extras {
        let blocks = apply_block_pruning(
            snapshot.clone(),
            table.get_table_info().schema(),
            &extra,
            ctx.clone(),
            fuse_table.get_operator(),
            fuse_table.bloom_index_cols(),
        )
        .await?;

        let rows = blocks.iter().map(|b| b.row_count as usize).sum::<usize>();
        assert_eq!(expected_rows, rows);
        assert_eq!(expected_blocks, blocks.len());
    }

    Ok(())
}
