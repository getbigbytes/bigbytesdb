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

use bigbytes_common_exception::Result;
use bigbytes_common_meta_app::tenant::TenantQuota;
use bigbytes_common_meta_types::seq_value::SeqV;
use bigbytes_common_meta_types::MatchSeq;

#[async_trait::async_trait]
pub trait QuotaApi: Sync + Send {
    async fn get_quota(&self, seq: MatchSeq) -> Result<SeqV<TenantQuota>>;

    async fn set_quota(&self, quota: &TenantQuota, seq: MatchSeq) -> Result<u64>;
}
