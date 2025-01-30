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
use bigbytesdb_common_expression::Expr;
use bigbytesdb_common_expression::Scalar;

pub fn find_eq_filter(expr: &Expr<String>, visitor: &mut impl FnMut(&str, &Scalar) -> Result<()>) {
    match expr {
        Expr::Constant { .. } | Expr::ColumnRef { .. } => {}
        Expr::Cast { expr, .. } => find_eq_filter(expr, visitor),
        Expr::FunctionCall { function, args, .. } => {
            // Like: select * from (select * from system.tables where database='default') where name='t'
            // push downs: [filters: [and_filters(and_filters(tables.database (#1) = 'default', tables.name (#2) = 't'), tables.database (#1) = 'default')], limit: NONE]
            // database generate twice, so when call find_eq_filter, should check uniq.
            if function.signature.name == "eq" {
                match args.as_slice() {
                    [Expr::ColumnRef { id, .. }, Expr::Constant { scalar, .. }]
                    | [Expr::Constant { scalar, .. }, Expr::ColumnRef { id, .. }] => {
                        let _ = visitor(id, scalar);
                    }
                    _ => {}
                }
            } else if function.signature.name == "and_filters" {
                // only support this:
                // 1. where xx and xx and xx
                // 2. filter: Column `table`, Column `database`
                for arg in args {
                    find_eq_filter(arg, visitor)
                }
            }
        }
        Expr::LambdaFunctionCall { args, .. } => {
            for arg in args {
                find_eq_filter(arg, visitor)
            }
        }
    }
}

pub fn find_gt_filter(expr: &Expr<String>, visitor: &mut impl FnMut(&str, &Scalar)) {
    match expr {
        Expr::Constant { .. } | Expr::ColumnRef { .. } => {}
        Expr::Cast { expr, .. } => find_gt_filter(expr, visitor),
        Expr::FunctionCall { function, args, .. } => {
            if function.signature.name == "gt" || function.signature.name == "gte" {
                match args.as_slice() {
                    [Expr::ColumnRef { id, .. }, Expr::Constant { scalar, .. }]
                    | [Expr::Constant { scalar, .. }, Expr::ColumnRef { id, .. }] => {
                        visitor(id, scalar);
                    }
                    _ => {}
                }
            } else if function.signature.name == "and_filters" {
                // only support this:
                // 1. where xx and xx and xx
                // 2. filter: Column `table`, Column `database`
                for arg in args {
                    find_gt_filter(arg, visitor)
                }
            }
        }
        Expr::LambdaFunctionCall { args, .. } => {
            for arg in args {
                find_gt_filter(arg, visitor)
            }
        }
    }
}

pub fn find_lt_filter(expr: &Expr<String>, visitor: &mut impl FnMut(&str, &Scalar)) {
    match expr {
        Expr::Constant { .. } | Expr::ColumnRef { .. } => {}
        Expr::Cast { expr, .. } => find_lt_filter(expr, visitor),
        Expr::FunctionCall { function, args, .. } => {
            if function.signature.name == "lt" || function.signature.name == "lte" {
                match args.as_slice() {
                    [Expr::ColumnRef { id, .. }, Expr::Constant { scalar, .. }]
                    | [Expr::Constant { scalar, .. }, Expr::ColumnRef { id, .. }] => {
                        visitor(id, scalar);
                    }
                    _ => {}
                }
            } else if function.signature.name == "and_filters" {
                // only support this:
                // 1. where xx and xx and xx
                // 2. filter: Column `table`, Column `database`
                for arg in args {
                    find_lt_filter(arg, visitor)
                }
            }
        }
        Expr::LambdaFunctionCall { args, .. } => {
            for arg in args {
                find_lt_filter(arg, visitor)
            }
        }
    }
}

pub fn find_eq_or_filter(
    expr: &Expr<String>,
    visitor: &mut impl FnMut(&str, &Scalar) -> Result<()>,
    mut invalid_optimize: bool,
) -> bool {
    if invalid_optimize {
        return true;
    }

    fn inner(
        expr: &Expr<String>,
        visitor: &mut impl FnMut(&str, &Scalar) -> Result<()>,
        invalid_optimize: &mut bool,
    ) {
        match expr {
            Expr::Constant { .. } | Expr::ColumnRef { .. } => {}
            Expr::Cast { expr, .. } => inner(expr, visitor, invalid_optimize),
            Expr::FunctionCall { function, args, .. } => {
                // Like: select * from (select * from system.tables where database='default') where name='t'
                // push downs: [filters: [and_filters(and_filters(tables.database (#1) = 'default', tables.name (#2) = 't'), tables.database (#1) = 'default')], limit: NONE]
                // database generate twice, so when call find_eq_filter, should check uniq.
                match function.signature.name.as_str() {
                    "eq" => {
                        if let [Expr::ColumnRef { id, .. }, Expr::Constant { scalar, .. }]
                        | [Expr::Constant { scalar, .. }, Expr::ColumnRef { id, .. }] =
                            args.as_slice()
                        {
                            let _ = visitor(id, scalar);
                        }
                    }
                    "and_filters" => {
                        for arg in args {
                            inner(arg, visitor, invalid_optimize);
                            if *invalid_optimize {
                                return;
                            }
                        }
                    }
                    "or" => {
                        // Check for conflicting column references in "or" conditions.
                        if args.windows(2).any(|w| {
                            match (&w[0], &w[1]) {
                                // e.g. will get all tables
                                // database = 'a' or name = 't'
                                // name = 't' or table_id = 123
                                (
                                    Expr::FunctionCall {
                                        function: lfunc,
                                        args: largs,
                                        ..
                                    },
                                    Expr::FunctionCall {
                                        function: rfunc,
                                        args: rargs,
                                        ..
                                    },
                                ) => {
                                    if lfunc.signature.name == "eq" && rfunc.signature.name == "eq" {
                                        match (largs.as_slice(), rargs.as_slice()) {
                                            (
                                                [Expr::ColumnRef { id, .. }, Expr::Constant { .. }]
                                                | [Expr::Constant { .. }, Expr::ColumnRef { id, .. }],
                                                [Expr::ColumnRef { id: rid, .. }, Expr::Constant { .. }]
                                                | [Expr::Constant { .. }, Expr::ColumnRef { id: rid, .. }],
                                            ) => id != rid,
                                            _ => false,
                                        }
                                    } else {
                                        false
                                    }
                                }
                                _ => false,
                            }
                        }) {
                            *invalid_optimize = true;
                            return;
                        }

                        for arg in args {
                            inner(arg, visitor, invalid_optimize);
                            if *invalid_optimize {
                                return;
                            }
                        }
                    }
                    // show drop tables will specific is_not_null(dropped_on)
                    "is_not_null" => {
                        if let Expr::ColumnRef { id, .. } = args[0].clone() {
                            if id != "dropped_on" {
                                *invalid_optimize = true;
                            }
                        }
                    }
                    _ => {
                        // Any other function makes it invalid.
                        *invalid_optimize = true;
                    }
                }
            }
            Expr::LambdaFunctionCall { args, .. } => {
                for arg in args {
                    inner(arg, visitor, invalid_optimize);
                    if *invalid_optimize {
                        return;
                    }
                }
            }
        }
    }

    inner(expr, visitor, &mut invalid_optimize);
    invalid_optimize
}
