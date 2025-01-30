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

use bigbytes_common_base::base::tokio;
use bigbytes_common_base::base::tokio::sync::mpsc::channel;
use bigbytes_common_base::base::tokio::sync::mpsc::Receiver;
use bigbytes_common_exception::Result;
use bigbytes_common_expression::types::Int32Type;
use bigbytes_common_expression::DataBlock;
use bigbytes_common_expression::DataField;
use bigbytes_common_expression::DataSchemaRefExt;
use bigbytes_common_expression::FromData;
use bigbytes_common_expression::SortColumnDescription;
use bigbytes_common_pipeline_core::processors::InputPort;
use bigbytes_common_pipeline_core::processors::OutputPort;
use bigbytes_common_pipeline_core::processors::ProcessorPtr;
use bigbytes_common_pipeline_core::Pipe;
use bigbytes_common_pipeline_core::PipeItem;
use bigbytes_common_pipeline_core::Pipeline;
use bigbytes_common_pipeline_sinks::SyncSenderSink;
use bigbytes_common_pipeline_sources::BlocksSource;
use bigbytes_query::pipelines::executor::ExecutorSettings;
use bigbytes_query::pipelines::executor::QueryPipelineExecutor;
use bigbytes_query::sessions::QueryContext;
use bigbytes_query::test_kits::TestFixture;
use rand::rngs::ThreadRng;
use rand::Rng;

fn create_source_pipe(ctx: Arc<QueryContext>, data: Vec<Vec<DataBlock>>) -> Result<Pipe> {
    use parking_lot::Mutex;

    let size = data.len();
    let mut items = Vec::with_capacity(size);

    for blocks in data.into_iter() {
        let output = OutputPort::create();
        items.push(PipeItem::create(
            BlocksSource::create(
                ctx.clone(),
                output.clone(),
                Arc::new(Mutex::new(VecDeque::from(blocks))),
            )?,
            vec![],
            vec![output],
        ));
    }
    Ok(Pipe::create(0, size, items))
}

fn create_sink_pipe(size: usize) -> Result<(Vec<Receiver<Result<DataBlock>>>, Pipe)> {
    let mut rxs = Vec::with_capacity(size);
    let mut items = Vec::with_capacity(size);
    for _ in 0..size {
        let input = InputPort::create();
        let (tx, rx) = channel(1000);
        rxs.push(rx);
        items.push(PipeItem::create(
            ProcessorPtr::create(SyncSenderSink::create(tx, input.clone())),
            vec![input],
            vec![],
        ));
    }

    Ok((rxs, Pipe::create(size, 0, items)))
}

fn prepare_multi_input_and_result(
    data: Vec<Vec<Vec<i32>>>,
    limit: Option<usize>,
) -> (Vec<Vec<DataBlock>>, DataBlock) {
    use itertools::Itertools;
    let input = data
        .clone()
        .into_iter()
        .map(|v| {
            v.into_iter()
                .map(|v| DataBlock::new_from_columns(vec![Int32Type::from_data(v)]))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let result = data
        .into_iter()
        .flatten()
        .flatten()
        .sorted()
        .take(limit.unwrap_or(usize::MAX))
        .collect::<Vec<_>>();
    let result = DataBlock::new_from_columns(vec![Int32Type::from_data(result)]);

    (input, result)
}

fn prepare_single_input_and_result(
    data: Vec<Vec<i32>>,
    limit: Option<usize>,
) -> (Vec<DataBlock>, DataBlock) {
    let (mut input, expected) = prepare_multi_input_and_result(vec![data], limit);
    (input.remove(0), expected)
}

fn check_result(result: Vec<DataBlock>, expected: DataBlock) {
    use bigbytes_common_expression::block_debug::pretty_format_blocks;

    if expected.is_empty() {
        if !result.is_empty() && !DataBlock::concat(&result).unwrap().is_empty() {
            panic!(
                "\nexpected empty block, but got:\n {}",
                pretty_format_blocks(&result).unwrap()
            )
        }
        return;
    }

    let expected_rows = expected.num_rows();
    let expected = pretty_format_blocks(&[expected]).unwrap();
    let result_rows: usize = result.iter().map(|v| v.num_rows()).sum();
    let result = pretty_format_blocks(&result).unwrap();
    assert_eq!(
        expected, result,
        "\nexpected (num_rows = {expected_rows}):\n{expected}\nactual (num_rows = {result_rows}):\n{result}",
    );
}

mod k_way;
mod spill;
