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

use bigbytesdb_common_exception::Result;
use bigbytesdb_common_expression::eval_function;
use bigbytesdb_common_expression::types::AnyType;
use bigbytesdb_common_expression::types::BooleanType;
use bigbytesdb_common_expression::types::DataType;
use bigbytesdb_common_expression::Evaluator;
use bigbytesdb_common_expression::Expr;
use bigbytesdb_common_expression::FunctionContext;
use bigbytesdb_common_expression::Value;
use bigbytesdb_common_functions::BUILTIN_FUNCTIONS;

pub(crate) fn get_and(
    filter1: Value<BooleanType>,
    filter2: Value<BooleanType>,
    func_ctx: &FunctionContext,
    rows: usize,
) -> Result<(Value<AnyType>, DataType)> {
    eval_function(
        None,
        "and",
        [
            (filter1.upcast(), DataType::Boolean),
            (filter2.upcast(), DataType::Boolean),
        ],
        func_ctx,
        rows,
        &BUILTIN_FUNCTIONS,
    )
}

pub(crate) fn get_not(
    filter: Value<BooleanType>,
    func_ctx: &FunctionContext,
    rows: usize,
) -> Result<(Value<AnyType>, DataType)> {
    eval_function(
        None,
        "not",
        [(filter.upcast(), DataType::Boolean)],
        func_ctx,
        rows,
        &BUILTIN_FUNCTIONS,
    )
}

pub(crate) fn get_or(
    filter1: Value<BooleanType>,
    filter2: Value<BooleanType>,
    func_ctx: &FunctionContext,
    rows: usize,
) -> Result<(Value<AnyType>, DataType)> {
    eval_function(
        None,
        "or",
        [
            (filter1.upcast(), DataType::Boolean),
            (filter2.upcast(), DataType::Boolean),
        ],
        func_ctx,
        rows,
        &BUILTIN_FUNCTIONS,
    )
}

pub(crate) fn expr2prdicate(evaluator: &Evaluator, filter: &Expr) -> Result<Value<BooleanType>> {
    Ok(evaluator
        .run(filter)
        .map_err(|e| e.add_message("eval filter failed:"))?
        .try_downcast::<BooleanType>()
        .unwrap())
}
