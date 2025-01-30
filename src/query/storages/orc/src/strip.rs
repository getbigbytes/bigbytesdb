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

use bigbytes_common_expression::local_block_meta_serde;
use bigbytes_common_expression::BlockMetaInfo;
use orc_rust::stripe::Stripe;

use crate::hashable_schema::HashableSchema;

#[derive(Debug)]
pub struct StripeInMemory {
    pub path: String,
    pub stripe: Stripe,
    pub schema: Option<HashableSchema>,
}

local_block_meta_serde!(StripeInMemory);

#[typetag::serde(name = "orc_stripe")]
impl BlockMetaInfo for StripeInMemory {}
