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
use bigbytesdb_common_expression::Column;
use bigbytesdb_common_expression::DataBlock;
use bigbytesdb_common_expression::TableSchemaRef;
use bigbytesdb_common_meta_app::principal::TsvFileFormatParams;

use crate::field_encoder::helpers::write_tsv_escaped_string;
use crate::field_encoder::FieldEncoderCSV;
use crate::output_format::OutputFormat;
use crate::FileFormatOptionsExt;

pub type TSVOutputFormat = TSVOutputFormatBase<false, false>;
pub type TSVWithNamesOutputFormat = TSVOutputFormatBase<true, false>;
pub type TSVWithNamesAndTypesOutputFormat = TSVOutputFormatBase<true, true>;

pub struct TSVOutputFormatBase<const WITH_NAMES: bool, const WITH_TYPES: bool> {
    schema: TableSchemaRef,
    field_encoder: FieldEncoderCSV,
    field_delimiter: u8,
    record_delimiter: Vec<u8>,
}

impl<const WITH_NAMES: bool, const WITH_TYPES: bool> TSVOutputFormatBase<WITH_NAMES, WITH_TYPES> {
    pub fn create(
        schema: TableSchemaRef,
        params: &TsvFileFormatParams,
        options_ext: &FileFormatOptionsExt,
    ) -> Self {
        let field_encoder = FieldEncoderCSV::create_tsv(params, options_ext);
        Self {
            schema,
            field_encoder,
            field_delimiter: params.field_delimiter.as_bytes()[0],
            record_delimiter: params.record_delimiter.as_bytes().to_vec(),
        }
    }

    fn serialize_strings(&self, values: Vec<String>) -> Vec<u8> {
        let mut buf = vec![];
        let fd = self.field_delimiter;

        for (col_index, v) in values.iter().enumerate() {
            if col_index != 0 {
                buf.push(fd);
            }
            write_tsv_escaped_string(v.as_bytes(), &mut buf, self.field_delimiter);
        }

        buf.extend_from_slice(&self.record_delimiter);
        buf
    }
}

impl<const WITH_NAMES: bool, const WITH_TYPES: bool> OutputFormat
    for TSVOutputFormatBase<WITH_NAMES, WITH_TYPES>
{
    fn serialize_block(&mut self, block: &DataBlock) -> Result<Vec<u8>> {
        let rows_size = block.num_rows();
        let mut buf = Vec::with_capacity(block.memory_size());

        let fd = self.field_delimiter;
        let rd = &self.record_delimiter;

        let columns: Vec<Column> = block
            .convert_to_full()
            .columns()
            .iter()
            .map(|column| column.value.clone().into_column().unwrap())
            .collect();

        for row_index in 0..rows_size {
            for (col_index, column) in columns.iter().enumerate() {
                if col_index != 0 {
                    buf.push(fd);
                }
                self.field_encoder.write_field(column, row_index, &mut buf);
            }
            buf.extend_from_slice(rd)
        }
        Ok(buf)
    }

    fn serialize_prefix(&self) -> Result<Vec<u8>> {
        let mut buf = vec![];
        if WITH_NAMES {
            let names = self
                .schema
                .fields()
                .iter()
                .map(|f| f.name().to_string())
                .collect::<Vec<_>>();
            buf.extend_from_slice(&self.serialize_strings(names));
            if WITH_TYPES {
                let types = self
                    .schema
                    .fields()
                    .iter()
                    .map(|f| f.data_type().to_string())
                    .collect::<Vec<_>>();
                buf.extend_from_slice(&self.serialize_strings(types));
            }
        }
        Ok(buf)
    }
    fn finalize(&mut self) -> Result<Vec<u8>> {
        Ok(vec![])
    }
}
