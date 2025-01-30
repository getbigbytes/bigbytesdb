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

use bigbytes_common_expression::arithmetics_type::ResultTypeOfBinary;
use bigbytes_common_expression::types::Number;
use bigbytes_common_expression::types::NumberDataType;
use bigbytes_common_expression::types::NumberType;
use bigbytes_common_expression::types::SimpleDomain;
use bigbytes_common_expression::types::ALL_INTEGER_TYPES;
use bigbytes_common_expression::types::F64;
use bigbytes_common_expression::vectorize_with_builder_2_arg;
use bigbytes_common_expression::with_integer_mapped_type;
use bigbytes_common_expression::FunctionDomain;
use bigbytes_common_expression::FunctionRegistry;
use bigbytes_functions_scalar_numeric_basic_arithmetic::numeric_basic_arithmetic::divide_function;
use bigbytes_functions_scalar_numeric_basic_arithmetic::register_basic_arithmetic;
use bigbytes_functions_scalar_numeric_basic_arithmetic::register_divide;
use bigbytes_functions_scalar_numeric_basic_arithmetic::register_intdiv;
use bigbytes_functions_scalar_numeric_basic_arithmetic::register_minus;
use bigbytes_functions_scalar_numeric_basic_arithmetic::register_modulo;
use bigbytes_functions_scalar_numeric_basic_arithmetic::register_multiply;
use bigbytes_functions_scalar_numeric_basic_arithmetic::register_plus;
use bigbytes_functions_scalar_numeric_basic_arithmetic::vectorize_modulo;
use num_traits::AsPrimitive;

pub fn register_integer_basic_arithmetic(registry: &mut FunctionRegistry) {
    for left in ALL_INTEGER_TYPES {
        for right in ALL_INTEGER_TYPES {
            with_integer_mapped_type!(|L| match left {
                NumberDataType::L => with_integer_mapped_type!(|R| match right {
                    NumberDataType::R => {
                        register_basic_arithmetic!(L, R, registry);
                    }
                    _ => unreachable!(),
                }),
                _ => unreachable!(),
            });
        }
    }
}
