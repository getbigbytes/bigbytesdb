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

use crate::MetaAPIError;
use crate::MetaNetworkError;

/// Errors raised when invoking an RPC.
///
/// It includes two sub errors:
/// - Error that occurs when sending the RPC.
/// - Error that is returned by remove service.
#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum ForwardRPCError {
    #[error(transparent)]
    NetworkError(#[from] MetaNetworkError),

    #[error(transparent)]
    RemoteError(#[from] MetaAPIError),
}

impl From<ForwardRPCError> for MetaAPIError {
    fn from(e: ForwardRPCError) -> Self {
        match e {
            ForwardRPCError::NetworkError(e) => e.into(),
            ForwardRPCError::RemoteError(e) => {
                //
                match e {
                    MetaAPIError::DataError(e) => MetaAPIError::RemoteError(e),
                    _ => e,
                }
            }
        }
    }
}
