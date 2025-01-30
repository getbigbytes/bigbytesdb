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

#![allow(clippy::uninlined_format_args)]
#![feature(try_blocks)]

mod entry;

use bigbytes_common_base::mem_allocator::GlobalAllocator;
use bigbytes_common_base::runtime::Runtime;
use bigbytes_common_base::runtime::ThreadTracker;
use bigbytes_common_config::InnerConfig;
use bigbytes_common_exception::Result;
use bigbytes_common_exception::ResultExt;
use bigbytes_common_license::license_manager::LicenseManager;
use bigbytes_common_license::license_manager::OssLicenseManager;
use bigbytes_common_tracing::pipe_file;
use bigbytes_common_tracing::set_crash_hook;
use bigbytes_common_tracing::SignalListener;
use entry::MainError;

use crate::entry::init_services;
use crate::entry::run_cmd;
use crate::entry::start_services;

#[global_allocator]
pub static GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator;

fn main() {
    let binary_version = (*bigbytes_common_config::BIGBYTES_COMMIT_VERSION).clone();

    // Crash tracker
    let (input, output) = pipe_file().unwrap();
    set_crash_hook(output);
    SignalListener::spawn(input, binary_version);

    // Thread tracker
    ThreadTracker::init();

    match Runtime::with_default_worker_threads() {
        Err(cause) => {
            eprintln!("Bigbytes Query start failure, cause: {:?}", cause);
            std::process::exit(cause.code() as i32);
        }
        Ok(rt) => {
            if let Err(cause) = rt.block_on(main_entrypoint()) {
                eprintln!("Bigbytes Query start failure, cause: {:?}", cause);
                std::process::exit(cause.code() as i32);
            }
        }
    }
}

async fn main_entrypoint() -> Result<(), MainError> {
    let make_error = || "an fatal error occurred in query";

    let conf: InnerConfig = InnerConfig::load().await.with_context(make_error)?;
    if run_cmd(&conf).await.with_context(make_error)? {
        return Ok(());
    }

    init_services(&conf, false).await?;
    // init oss license manager
    OssLicenseManager::init(conf.query.tenant_id.tenant_name().to_string())
        .with_context(make_error)?;
    start_services(&conf).await
}
