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

use bigbytesdb_common_ast::ast::DeclareItem;
use bigbytesdb_common_ast::ast::ScriptStatement;
use bigbytesdb_common_ast::parser::run_parser;
use bigbytesdb_common_ast::parser::script::script_block;
use bigbytesdb_common_ast::parser::tokenize_sql;
use bigbytesdb_common_ast::parser::ParseMode;
use bigbytesdb_common_exception::Result;
use bigbytesdb_common_expression::block_debug::box_render;
use bigbytesdb_common_expression::types::StringType;
use bigbytesdb_common_expression::DataBlock;
use bigbytesdb_common_expression::FromData;
use bigbytesdb_common_script::compile;
use bigbytesdb_common_script::Executor;
use bigbytesdb_common_script::ReturnValue;
use bigbytesdb_common_sql::plans::ExecuteImmediatePlan;
use bigbytesdb_common_storages_fuse::TableContext;

use crate::interpreters::util::ScriptClient;
use crate::interpreters::Interpreter;
use crate::pipelines::PipelineBuildResult;
use crate::sessions::QueryContext;

#[derive(Debug)]
pub struct ExecuteImmediateInterpreter {
    ctx: Arc<QueryContext>,
    plan: ExecuteImmediatePlan,
}

impl ExecuteImmediateInterpreter {
    pub fn try_create(ctx: Arc<QueryContext>, plan: ExecuteImmediatePlan) -> Result<Self> {
        Ok(ExecuteImmediateInterpreter { ctx, plan })
    }
}

#[async_trait::async_trait]
impl Interpreter for ExecuteImmediateInterpreter {
    fn name(&self) -> &str {
        "ExecuteImmediateInterpreter"
    }

    fn is_ddl(&self) -> bool {
        false
    }

    #[fastrace::trace]
    #[async_backtrace::framed]
    async fn execute2(&self) -> Result<PipelineBuildResult> {
        let res: Result<_> = try {
            let settings = self.ctx.get_settings();
            let sql_dialect = settings.get_sql_dialect()?;
            let tokens = tokenize_sql(&self.plan.script)?;
            let mut ast = run_parser(
                &tokens,
                sql_dialect,
                ParseMode::Template,
                false,
                script_block,
            )?;

            let mut src = vec![];
            for declare in ast.declares {
                match declare {
                    DeclareItem::Var(declare) => src.push(ScriptStatement::LetVar { declare }),
                    DeclareItem::Set(declare) => {
                        src.push(ScriptStatement::LetStatement { declare })
                    }
                }
            }
            src.append(&mut ast.body);
            let compiled = compile(&src)?;

            let client = ScriptClient {
                ctx: self.ctx.clone(),
            };
            let mut executor = Executor::load(ast.span, client, compiled);
            let script_max_steps = settings.get_script_max_steps()?;
            let result = executor.run(script_max_steps as usize).await?;

            match result {
                Some(ReturnValue::Var(scalar)) => {
                    PipelineBuildResult::from_blocks(vec![DataBlock::new_from_columns(vec![
                        StringType::from_data(vec![scalar.to_string()]),
                    ])])?
                }
                Some(ReturnValue::Set(set)) => {
                    let rendered_table = box_render(
                        &set.schema,
                        &[set.block.clone()],
                        usize::MAX,
                        usize::MAX,
                        usize::MAX,
                        true,
                    )?;
                    let lines = rendered_table.lines().map(|x| x.to_string()).collect();
                    PipelineBuildResult::from_blocks(vec![DataBlock::new_from_columns(vec![
                        StringType::from_data(lines),
                    ])])?
                }
                None => PipelineBuildResult::from_blocks(vec![DataBlock::new_from_columns(vec![
                    StringType::from_data(Vec::<String>::new()),
                ])])?,
            }
        };

        res.map_err(|err| err.display_with_sql(&self.plan.script))
    }
}
