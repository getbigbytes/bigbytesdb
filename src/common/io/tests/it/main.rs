#![feature(cursor_split)]
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
#![feature(assert_matches)]

extern crate core;

mod binary_read;
mod binary_write;
mod bincode_serialization;
mod borsh_serialization;
mod cursor_ext;
mod escape;
mod interval;
mod serialization_format_compatability;
