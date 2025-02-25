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

#[macro_use]
extern crate afl;

use bigbytesdb_common_ast::parser::parse_expr;
use bigbytesdb_common_ast::parser::tokenize_sql;
use bigbytesdb_common_ast::Backtrace;

fn main() {
    loop {
        fuzz!(|text: String| {
            let backtrace = Backtrace::new();
            let tokens = tokenize_sql(&text).unwrap();
            let _ = parse_expr(&tokens, &backtrace);
        });
    }
}
