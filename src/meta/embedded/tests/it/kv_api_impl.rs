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

use std::collections::BTreeMap;
use std::sync::Once;

use bigbytes_common_base::base::tokio;
use bigbytes_common_meta_kvapi::kvapi;
use bigbytes_common_meta_raft_store::mem_meta::MemMeta;
use bigbytes_common_tracing::init_logging;
use bigbytes_common_tracing::Config;

#[derive(Clone)]
struct Builder;

#[async_trait::async_trait]
impl kvapi::ApiBuilder<MemMeta> for Builder {
    async fn build(&self) -> MemMeta {
        MemMeta::default()
    }

    async fn build_cluster(&self) -> Vec<MemMeta> {
        unreachable!("InMemoryMeta does not support cluster")
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_mem_meta_kv_api() -> anyhow::Result<()> {
    setup_test();
    kvapi::TestSuite {}.test_single_node(&Builder).await
}

fn setup_test() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let mut config = Config::new_testing();
        config.file.prefix_filter = "".to_string();

        let guards = init_logging("meta_unittests", &config, BTreeMap::new());
        Box::leak(Box::new(guards));
    });
}
