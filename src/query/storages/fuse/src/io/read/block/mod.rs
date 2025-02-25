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

mod block_reader;
mod block_reader_deserialize;
mod block_reader_merge_io;
mod block_reader_merge_io_async;
mod block_reader_merge_io_sync;
mod block_reader_native;
mod block_reader_native_deserialize;
mod block_reader_parquet_deserialize;
pub mod parquet;

pub use block_reader::BlockReader;
pub use block_reader_merge_io::BlockReadResult;
pub use block_reader_native::NativeReaderExt;
pub use block_reader_native::NativeSourceData;
