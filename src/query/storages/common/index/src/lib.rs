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
#![feature(box_patterns)]

mod bloom_index;
pub mod filters;
mod index;
mod inverted_index;
mod page_index;
mod range_index;

pub use bloom_index::BloomIndex;
pub use bloom_index::BloomIndexMeta;
pub use bloom_index::FilterEvalResult;
pub use index::Index;
pub use inverted_index::extract_component_fields;
pub use inverted_index::extract_fsts;
pub use inverted_index::DocIdsCollector;
pub use inverted_index::InvertedIndexDirectory;
pub use inverted_index::InvertedIndexFile;
pub use inverted_index::InvertedIndexMeta;
pub use inverted_index::TermReader;
pub use page_index::PageIndex;
pub use range_index::statistics_to_domain;
pub use range_index::RangeIndex;
