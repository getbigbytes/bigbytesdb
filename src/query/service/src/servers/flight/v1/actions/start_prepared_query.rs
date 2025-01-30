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

use bigbytesdb_common_base::runtime::ThreadTracker;
use bigbytesdb_common_exception::Result;
use log::debug;

use crate::servers::flight::v1::exchange::DataExchangeManager;

pub static START_PREPARED_QUERY: &str = "/actions/start_prepared_query";

pub async fn start_prepared_query(id: String) -> Result<()> {
    let mut tracking_payload = ThreadTracker::new_tracking_payload();
    tracking_payload.query_id = Some(id.clone());
    let _guard = ThreadTracker::tracking(tracking_payload);

    debug!("start prepared query {}", id);
    if let Err(cause) = DataExchangeManager::instance().execute_partial_query(&id) {
        DataExchangeManager::instance().on_finished_query(&id, Some(cause.clone()));
        return Err(cause);
    }
    Ok(())
}
