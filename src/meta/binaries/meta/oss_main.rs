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

#![feature(stmt_expr_attributes)]

mod entry;
mod kvapi;

use bigbytesdb_common_base::mem_allocator::GlobalAllocator;
use bigbytesdb_meta::configs::Config;

#[global_allocator]
pub static GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> anyhow::Result<()> {
    let conf = Config::load()?;
    conf.validate()?;
    entry::entry(conf).await
}
