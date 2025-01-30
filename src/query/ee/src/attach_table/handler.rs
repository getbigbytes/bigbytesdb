// Copyright 2024 Digitrans Inc
//
// Licensed under the Elastic License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.elastic.co/licensing/elastic-license
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use bigbytes_common_base::base::GlobalInstance;
use bigbytes_common_meta_app::schema::CreateTableReq;
use bigbytes_common_meta_app::schema::TableMeta;
use bigbytes_common_meta_app::schema::TableNameIdent;
use bigbytes_common_meta_app::schema::TableStatistics;
use bigbytes_common_sql::plans::CreateTablePlan;
use bigbytes_common_storage::check_operator;
use bigbytes_common_storage::init_operator;
use bigbytes_common_storages_fuse::io::MetaReaders;
use bigbytes_common_storages_fuse::FUSE_TBL_LAST_SNAPSHOT_HINT;
use bigbytes_enterprise_attach_table::AttachTableHandler;
use bigbytes_enterprise_attach_table::AttachTableHandlerWrapper;
use bigbytes_storages_common_cache::LoadParams;
use bigbytes_storages_common_table_meta::meta::TableSnapshot;
use bigbytes_storages_common_table_meta::meta::Versioned;
use bigbytes_storages_common_table_meta::table::OPT_KEY_SNAPSHOT_LOCATION;

pub struct RealAttachTableHandler {}
#[async_trait::async_trait]
impl AttachTableHandler for RealAttachTableHandler {
    #[async_backtrace::framed]
    async fn build_attach_table_request(
        &self,
        storage_prefix: &str,
        plan: &CreateTablePlan,
    ) -> bigbytes_common_exception::Result<CreateTableReq> {
        // Safe to unwrap here, as attach table must have storage params.
        let sp = plan.storage_params.as_ref().unwrap();
        let operator = init_operator(sp)?;
        check_operator(&operator, sp).await?;
        let reader = MetaReaders::table_snapshot_reader(operator.clone());
        let hint = format!("{}/{}", storage_prefix, FUSE_TBL_LAST_SNAPSHOT_HINT);
        let snapshot_loc = operator.read(&hint).await?.to_vec();
        let snapshot_loc = String::from_utf8(snapshot_loc)?;
        let info = operator.info();
        let root = info.root();
        let snapshot_loc = snapshot_loc[root.len()..].to_string();
        let mut options = plan.options.clone();
        options.insert(OPT_KEY_SNAPSHOT_LOCATION.to_string(), snapshot_loc.clone());

        let params = LoadParams {
            location: snapshot_loc.clone(),
            len_hint: None,
            ver: TableSnapshot::VERSION,
            put_cache: true,
        };

        let snapshot = reader.read(&params).await?;
        let stat = TableStatistics {
            number_of_rows: snapshot.summary.row_count,
            data_bytes: snapshot.summary.uncompressed_byte_size,
            compressed_data_bytes: snapshot.summary.compressed_byte_size,
            index_data_bytes: snapshot.summary.index_size,
            number_of_segments: Some(snapshot.segments.len() as u64),
            number_of_blocks: Some(snapshot.summary.block_count),
        };

        let field_comments = vec!["".to_string(); snapshot.schema.num_fields()];
        let table_meta = TableMeta {
            schema: Arc::new(snapshot.schema.clone()),
            engine: plan.engine.to_string(),
            storage_params: plan.storage_params.clone(),
            options,
            cluster_key: None,
            field_comments,
            drop_on: None,
            statistics: stat,
            ..Default::default()
        };
        let req = CreateTableReq {
            create_option: plan.create_option,
            name_ident: TableNameIdent {
                tenant: plan.tenant.clone(),
                db_name: plan.database.to_string(),
                table_name: plan.table.to_string(),
            },
            table_meta,
            as_dropped: false,
        };

        Ok(req)
    }
}

impl RealAttachTableHandler {
    pub fn init() -> bigbytes_common_exception::Result<()> {
        let rm = RealAttachTableHandler {};
        let wrapper = AttachTableHandlerWrapper::new(Box::new(rm));
        GlobalInstance::set(Arc::new(wrapper));
        Ok(())
    }
}
