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

use bigbytesdb_common_exception::ErrorCode;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_sql::plans::SetPriorityPlan;

use crate::interpreters::Interpreter;
use crate::interpreters::SetPriorityInterpreter;
use crate::servers::flight::v1::actions::create_session;

pub static SET_PRIORITY: &str = "/actions/set_priority";

pub async fn set_priority(plan: SetPriorityPlan) -> Result<bool> {
    let session = create_session()?;
    let query_context = session.create_query_context().await?;
    let interpreter = SetPriorityInterpreter::from_flight(query_context, plan)?;
    match interpreter.execute2().await {
        Ok(_) => Ok(true),
        Err(cause) => match cause.code() == ErrorCode::UNKNOWN_SESSION {
            true => Ok(false),
            false => Err(cause),
        },
    }
}
