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

use bigbytes_common_ast::ast::DeclareItem;
use bigbytes_common_ast::ast::DeclareVar;
use bigbytes_common_ast::ast::Identifier;
use bigbytes_common_ast::ast::ScriptStatement;
use bigbytes_common_ast::parser::run_parser;
use bigbytes_common_ast::parser::script::script_block;
use bigbytes_common_ast::parser::tokenize_sql;
use bigbytes_common_ast::parser::ParseMode;
use bigbytes_common_exception::Result;
use bigbytes_common_expression::block_debug::box_render;
use bigbytes_common_expression::types::StringType;
use bigbytes_common_expression::DataBlock;
use bigbytes_common_expression::FromData;
use bigbytes_common_script::compile;
use bigbytes_common_script::Executor;
use bigbytes_common_script::ReturnValue;
use bigbytes_common_sql::plans::CallProcedurePlan;
use bigbytes_common_storages_fuse::TableContext;

use crate::interpreters::util::ScriptClient;
use crate::interpreters::Interpreter;
use crate::pipelines::PipelineBuildResult;
use crate::sessions::QueryContext;

#[derive(Debug)]
pub struct CallProcedureInterpreter {
    ctx: Arc<QueryContext>,
    plan: CallProcedurePlan,
}

impl CallProcedureInterpreter {
    pub fn try_create(ctx: Arc<QueryContext>, plan: CallProcedurePlan) -> Result<Self> {
        Ok(CallProcedureInterpreter { ctx, plan })
    }
}

#[async_trait::async_trait]
impl Interpreter for CallProcedureInterpreter {
    fn name(&self) -> &str {
        "ProcedureCall"
    }

    fn is_ddl(&self) -> bool {
        false
    }

    #[fastrace::trace]
    #[async_backtrace::framed]
    async fn execute2(&self) -> Result<PipelineBuildResult> {
        let res: Result<_> = try {
            let mut src = vec![];
            for (arg, arg_name) in self.plan.args.iter().zip(self.plan.arg_names.iter()) {
                src.push(ScriptStatement::LetVar {
                    declare: DeclareVar {
                        span: None,
                        name: Identifier::from_name(None, arg_name),
                        default: arg.clone(),
                    },
                });
            }
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
