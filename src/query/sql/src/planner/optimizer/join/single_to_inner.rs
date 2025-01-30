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

use std::sync::Arc;

use bigbytes_common_exception::Result;

use crate::optimizer::SExpr;
use crate::plans::JoinType;
use crate::plans::RelOperator;

// The SingleToInnerOptimizer will convert some single join to inner join.
pub struct SingleToInnerOptimizer {}

impl SingleToInnerOptimizer {
    pub fn new() -> Self {
        SingleToInnerOptimizer {}
    }

    pub fn run(self, s_expr: &SExpr) -> Result<SExpr> {
        Self::single_to_inner(s_expr)
    }

    #[recursive::recursive]
    fn single_to_inner(s_expr: &SExpr) -> Result<SExpr> {
        let mut s_expr = if let RelOperator::Join(join) = s_expr.plan.as_ref()
            && join.single_to_inner.is_some()
        {
            let mut join = join.clone();
            join.join_type = JoinType::Inner;
            s_expr.replace_plan(Arc::new(RelOperator::Join(join)))
        } else {
            s_expr.clone()
        };
        let mut children_changed = false;
        let mut children = Vec::with_capacity(s_expr.arity());
        for child in s_expr.children() {
            let new_child = Self::single_to_inner(child)?;
            if !new_child.eq(child) {
                children_changed = true;
            }
            children.push(Arc::new(new_child));
        }
        if children_changed {
            s_expr = s_expr.replace_children(children);
        }

        Ok(s_expr)
    }
}
