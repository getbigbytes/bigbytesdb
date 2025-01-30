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

use bigbytes_common_ast::ast::MatchOperation;
use bigbytes_common_ast::ast::MatchedClause;
use bigbytes_common_ast::ast::MergeUpdateExpr;
use bigbytes_common_ast::ast::TableReference;
use bigbytes_common_ast::ast::UpdateStmt;
use bigbytes_common_exception::ErrorCode;
use bigbytes_common_exception::Result;

use crate::binder::bind_mutation::bind::Mutation;
use crate::binder::bind_mutation::bind::MutationStrategy;
use crate::binder::bind_mutation::mutation_expression::MutationExpression;
use crate::binder::util::TableIdentifier;
use crate::binder::Binder;
use crate::plans::Plan;
use crate::BindContext;

impl Binder {
    #[async_backtrace::framed]
    pub(in crate::planner::binder) async fn bind_update(
        &mut self,
        bind_context: &mut BindContext,
        stmt: &UpdateStmt,
    ) -> Result<Plan> {
        let UpdateStmt {
            table,
            update_list,
            selection,
            with,
            ..
        } = stmt;

        self.init_cte(bind_context, with)?;

        let target_table_identifier = if let TableReference::Table {
            catalog,
            database,
            table,
            alias,
            ..
        } = table
        {
            TableIdentifier::new(self, catalog, database, table, alias)
        } else {
            // We do not support USING clause yet.
            return Err(ErrorCode::Internal(
                "should not happen, parser should have report error already",
            ));
        };

        let update_exprs = update_list
            .iter()
            .map(|update_expr| MergeUpdateExpr {
                table: None,
                name: update_expr.name.clone(),
                expr: update_expr.expr.clone(),
            })
            .collect::<Vec<_>>();
        let matched_clause = MatchedClause {
            selection: None,
            operation: MatchOperation::Update {
                update_list: update_exprs,
                is_star: false,
            },
        };

        let mutation = Mutation {
            target_table_identifier,
            expression: MutationExpression::Update {
                target: table.clone(),
                filter: selection.clone(),
            },
            strategy: MutationStrategy::MatchedOnly,
            matched_clauses: vec![matched_clause],
            unmatched_clauses: vec![],
        };

        self.bind_mutation(bind_context, mutation).await
    }
}
