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

use std::any::Any;
use std::sync::Arc;

use bigbytesdb_common_exception::Result;
use bigbytesdb_common_pipeline_core::processors::Event;
use bigbytesdb_common_pipeline_core::processors::OutputPort;
use bigbytesdb_common_pipeline_core::processors::Processor;
use bigbytesdb_common_pipeline_core::processors::ProcessorPtr;

pub struct EmptySource {
    output: Arc<OutputPort>,
}

impl EmptySource {
    pub fn create(output: Arc<OutputPort>) -> Result<ProcessorPtr> {
        Ok(ProcessorPtr::create(Box::new(EmptySource { output })))
    }
}

impl Processor for EmptySource {
    fn name(&self) -> String {
        "EmptySource".to_string()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }

    fn event(&mut self) -> Result<Event> {
        self.output.finish();
        Ok(Event::Finished)
    }
}
