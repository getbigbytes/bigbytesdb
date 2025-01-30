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

#![feature(try_blocks)]

mod context;
mod datablock;
mod dataframe;
mod schema;
mod utils;

use std::env;
use std::path::Path;

use bigbytes_common_config::Config;
use bigbytes_common_config::InnerConfig;
use bigbytes_common_license::license_manager::LicenseManager;
use bigbytes_common_license::license_manager::OssLicenseManager;
use bigbytes_common_meta_app::storage::StorageFsConfig;
use bigbytes_common_meta_app::storage::StorageParams;
use bigbytes_common_meta_embedded::MetaEmbedded;
use bigbytes_query::clusters::ClusterDiscovery;
use bigbytes_query::GlobalServices;
use pyo3::prelude::*;
use utils::RUNTIME;

/// A Python module implemented in Rust.
#[pymodule]
fn bigbytes(_py: Python, m: &PyModule) -> PyResult<()> {
    let data_path = env::var("BIGBYTES_DATA_PATH").unwrap_or(".bigbytes/".to_string());
    let path = Path::new(&data_path);

    env::set_var("META_EMBEDDED_DIR", path.join("_meta"));

    let mut conf: InnerConfig = Config::load(false).unwrap().try_into().unwrap();
    conf.storage.allow_insecure = true;
    conf.storage.params = StorageParams::Fs(StorageFsConfig {
        root: path.join("_data").to_str().unwrap().to_owned(),
    });

    RUNTIME.block_on(async {
        let meta_dir = path.join("_meta");
        MetaEmbedded::init_global_meta_store(meta_dir.to_string_lossy().to_string())
            .await
            .unwrap();
        GlobalServices::init(&conf).await.unwrap();

        // init oss license manager
        OssLicenseManager::init(conf.query.tenant_id.tenant_name().to_string()).unwrap();
        ClusterDiscovery::instance()
            .register_to_metastore(&conf)
            .await
            .unwrap();
    });

    m.add_class::<context::PySessionContext>()?;
    Ok(())
}
