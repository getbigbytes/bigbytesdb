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

use std::io;
use std::sync::mpsc::SyncSender;

use bigbytes_common_meta_types::raft_types;
use tokio::sync::oneshot;

pub enum CallbackData {
    Oneshot(oneshot::Sender<Result<(), io::Error>>),
    SyncOneshot(SyncSender<Result<(), io::Error>>),
    IOFlushed(raft_types::IOFlushed),
}
