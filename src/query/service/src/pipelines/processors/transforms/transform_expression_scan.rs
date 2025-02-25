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

use std::collections::VecDeque;
use std::sync::Arc;

use bigbytesdb_common_exception::Result;
use bigbytesdb_common_expression::BlockEntry;
use bigbytesdb_common_expression::DataBlock;
use bigbytesdb_common_expression::Evaluator;
use bigbytesdb_common_expression::Expr;
use bigbytesdb_common_expression::FunctionContext;
use bigbytesdb_common_functions::BUILTIN_FUNCTIONS;
use bigbytesdb_common_pipeline_transforms::processors::BlockingTransform;
use bigbytesdb_common_pipeline_transforms::processors::BlockingTransformer;

use crate::pipelines::processors::InputPort;
use crate::pipelines::processors::OutputPort;
use crate::pipelines::processors::Processor;

pub struct TransformExpressionScan {
    values: Vec<Vec<Expr>>,
    output_buffer: VecDeque<DataBlock>,
    func_ctx: FunctionContext,
}

impl TransformExpressionScan {
    pub fn create(
        input: Arc<InputPort>,
        output: Arc<OutputPort>,
        values: Vec<Vec<Expr>>,
        func_ctx: FunctionContext,
    ) -> Box<dyn Processor> {
        BlockingTransformer::create(input, output, TransformExpressionScan {
            values,
            output_buffer: VecDeque::new(),
            func_ctx,
        })
    }
}

impl BlockingTransform for TransformExpressionScan {
    const NAME: &'static str = "TransformExpressionScan";

    fn consume(&mut self, input: DataBlock) -> Result<()> {
        let evaluator = Evaluator::new(&input, &self.func_ctx, &BUILTIN_FUNCTIONS);
        for row in self.values.iter() {
            let mut columns = Vec::with_capacity(row.len());
            for expr in row {
                let result = evaluator.run(expr)?;
                let column = BlockEntry::new(expr.data_type().clone(), result);
                columns.push(column);
            }
            self.output_buffer
                .push_back(DataBlock::new(columns, input.num_rows()));
        }
        Ok(())
    }

    fn transform(&mut self) -> Result<Option<DataBlock>> {
        match !self.output_buffer.is_empty() {
            true => Ok(Some(self.output_buffer.pop_front().unwrap())),
            false => Ok(None),
        }
    }
}
