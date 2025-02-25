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

use std::convert::TryInto;
use std::io::Cursor;
use std::marker::PhantomData;

use bigbytesdb_common_column::buffer::Buffer;
use bigbytesdb_common_expression::types::ArgType;
use bigbytesdb_common_expression::types::Number;
use bigbytesdb_common_expression::Column;
use bigbytesdb_common_expression::TableDataType;

use crate::compression::integer::decompress_integer;
use crate::compression::integer::IntegerType;
use crate::error::Result;
use crate::nested::InitNested;
use crate::nested::NestedState;
use crate::read::read_basic::*;
use crate::read::BufReader;
use crate::read::NativeReadBuf;
use crate::read::PageIterator;
use crate::PageMeta;

#[derive(Debug)]
pub struct IntegerNestedIter<I, V, T>
where
    I: Iterator<Item = Result<(u64, Vec<u8>)>> + PageIterator + Send + Sync,
    V: ArgType<Column = Buffer<T>, Scalar = T>,
    T: Number + IntegerType,
    Vec<u8>: TryInto<T::Bytes>,
{
    iter: I,
    data_type: TableDataType,
    init: Vec<InitNested>,
    scratch: Vec<u8>,
    _phantom: PhantomData<V>,
    _phantom2: PhantomData<T>,
}

impl<I, V, T> IntegerNestedIter<I, V, T>
where
    I: Iterator<Item = Result<(u64, Vec<u8>)>> + PageIterator + Send + Sync,
    V: ArgType<Column = Buffer<T>, Scalar = T>,
    T: Number + IntegerType,
    Vec<u8>: TryInto<T::Bytes>,
{
    pub fn new(iter: I, data_type: TableDataType, init: Vec<InitNested>) -> Self {
        Self {
            iter,
            data_type,
            init,
            scratch: vec![],
            _phantom: PhantomData,
            _phantom2: PhantomData,
        }
    }
}

impl<I, V, T> IntegerNestedIter<I, V, T>
where
    I: Iterator<Item = Result<(u64, Vec<u8>)>> + PageIterator + Send + Sync,
    V: ArgType<Column = Buffer<T>, Scalar = T>,
    T: Number + IntegerType,
    Vec<u8>: TryInto<T::Bytes>,
{
    fn deserialize(&mut self, num_values: u64, buffer: Vec<u8>) -> Result<(NestedState, Column)> {
        let mut reader = BufReader::with_capacity(buffer.len(), Cursor::new(buffer));
        let (nested, validity) = read_nested(&mut reader, &self.init, num_values as usize)?;
        let length = num_values as usize;

        let mut values = Vec::with_capacity(length);
        decompress_integer(&mut reader, length, &mut values, &mut self.scratch)?;
        assert_eq!(values.len(), length);

        let mut buffer = reader.into_inner().into_inner();
        self.iter.swap_buffer(&mut buffer);

        let column: Buffer<T> = values.into();
        let mut col = V::upcast_column(column);
        if self.data_type.is_nullable() {
            col = col.wrap_nullable(validity);
        }
        Ok((nested, col))
    }
}

impl<I, V, T> Iterator for IntegerNestedIter<I, V, T>
where
    I: Iterator<Item = Result<(u64, Vec<u8>)>> + PageIterator + Send + Sync,
    V: ArgType<Column = Buffer<T>, Scalar = T>,
    T: Number + IntegerType,
    Vec<u8>: TryInto<T::Bytes>,
{
    type Item = Result<(NestedState, Column)>;

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        match self.iter.nth(n) {
            Some(Ok((num_values, buffer))) => Some(self.deserialize(num_values, buffer)),
            Some(Err(err)) => Some(Result::Err(err)),
            None => None,
        }
    }

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok((num_values, buffer))) => Some(self.deserialize(num_values, buffer)),
            Some(Err(err)) => Some(Result::Err(err)),
            None => None,
        }
    }
}

pub fn read_nested_integer<V, T, R: NativeReadBuf>(
    reader: &mut R,
    data_type: TableDataType,
    init: Vec<InitNested>,
    page_metas: Vec<PageMeta>,
) -> Result<Vec<(NestedState, Column)>>
where
    V: ArgType<Column = Buffer<T>, Scalar = T>,
    T: Number + IntegerType,
{
    let mut scratch = vec![];
    let mut results = Vec::with_capacity(page_metas.len());
    for page_meta in page_metas {
        let num_values = page_meta.num_values as usize;
        let (nested, validity) = read_nested(reader, &init, num_values)?;

        let mut values = Vec::with_capacity(num_values);
        decompress_integer(reader, num_values, &mut values, &mut scratch)?;

        let column: Buffer<T> = values.into();
        let mut col = V::upcast_column(column);
        if data_type.is_nullable() {
            col = col.wrap_nullable(validity);
        }
        results.push((nested, col));
    }
    Ok(results)
}
