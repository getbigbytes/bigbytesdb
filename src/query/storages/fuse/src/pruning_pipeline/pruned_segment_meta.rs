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

use std::fmt::Debug;
use std::fmt::Formatter;
use std::sync::Arc;

use bigbytesdb_common_expression::local_block_meta_serde;
use bigbytesdb_common_expression::BlockMetaInfo;
use bigbytesdb_common_expression::BlockMetaInfoPtr;
use bigbytesdb_storages_common_table_meta::meta::CompactSegmentInfo;

use crate::SegmentLocation;

pub struct PrunedSegmentMeta {
    pub segments: (SegmentLocation, Arc<CompactSegmentInfo>),
}

impl PrunedSegmentMeta {
    pub fn create(segments: (SegmentLocation, Arc<CompactSegmentInfo>)) -> BlockMetaInfoPtr {
        Box::new(PrunedSegmentMeta { segments })
    }
}

impl Debug for PrunedSegmentMeta {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrunedSegmentMeta").finish()
    }
}

local_block_meta_serde!(PrunedSegmentMeta);

#[typetag::serde(name = "pruned_segment_meta")]
impl BlockMetaInfo for PrunedSegmentMeta {}
