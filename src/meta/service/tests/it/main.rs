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

// Because the compiler complains about recursion limit for a trait requirement check...
// error[[E0275](https://doc.rust-lang.org/error-index.html#E0275)]: overflow evaluating the requirement `(...)`
// When compiling `impl KVApiBuilder<MetaGrpcClient> for Builder`.
#![allow(clippy::uninlined_format_args)]
#![recursion_limit = "1024"]
#![feature(extend_one)]
mod api;
mod configs;
mod grpc;
mod meta_node;
mod store;
mod testing;
mod tests;
