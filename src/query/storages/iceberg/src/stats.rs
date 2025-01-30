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

use std::collections::HashMap;

use bigbytesdb_common_expression::types::Number;
use bigbytesdb_common_expression::types::F32;
use bigbytesdb_common_expression::types::F64;
use bigbytesdb_common_expression::Scalar;
use bigbytesdb_common_expression::TableField;
use bigbytesdb_common_expression::TableSchema;
use bigbytesdb_storages_common_table_meta::meta::ColumnStatistics;
use bigbytesdb_storages_common_table_meta::meta::StatisticsOfColumns;
use iceberg::spec::DataFile;
use iceberg::spec::Datum;
use iceberg::spec::PrimitiveLiteral;
use ordered_float::OrderedFloat;

/// Try to convert statistics in [`DataFile`] to [`StatisticsOfColumns`].
pub fn get_stats_of_data_file(schema: &TableSchema, df: &DataFile) -> Option<StatisticsOfColumns> {
    let mut stats: HashMap<u32, ColumnStatistics> = HashMap::with_capacity(schema.num_fields());
    for field in schema.fields.iter() {
        if let Some(stat) = get_column_stats(
            field,
            df.lower_bounds(),
            df.upper_bounds(),
            df.null_value_counts(),
        ) {
            stats.insert(field.column_id, stat);
        }
    }
    Some(stats)
}

/// Try get [`ColumnStatistics`] for one column.
fn get_column_stats(
    field: &TableField,
    lower: &HashMap<i32, Datum>,
    upper: &HashMap<i32, Datum>,
    null_counts: &HashMap<i32, u64>,
) -> Option<ColumnStatistics> {
    // The column id in iceberg is 1-based while the column id in Bigbytesdb is 0-based.
    let iceberg_col_id = field.column_id as i32 + 1;
    match (
        lower.get(&iceberg_col_id),
        upper.get(&iceberg_col_id),
        null_counts.get(&iceberg_col_id),
    ) {
        (Some(lo), Some(up), Some(nc)) => {
            let min = parse_datum(lo)?;
            let max = parse_datum(up)?;
            Some(ColumnStatistics::new(
                min, max, *nc, 0, // this field is not used.
                None,
            ))
        }
        (_, _, _) => None,
    }
}

/// TODO: we need to support more types.
fn parse_datum(data: &Datum) -> Option<Scalar> {
    match data.literal() {
        PrimitiveLiteral::Boolean(v) => Some(Scalar::Boolean(*v)),
        PrimitiveLiteral::Int(v) => Some(Scalar::Number(i32::upcast_scalar(*v))),
        PrimitiveLiteral::Long(v) => Some(Scalar::Number(i64::upcast_scalar(*v))),
        PrimitiveLiteral::Float(OrderedFloat(v)) => {
            Some(Scalar::Number(F32::upcast_scalar(F32::from(*v))))
        }
        PrimitiveLiteral::Double(OrderedFloat(v)) => {
            Some(Scalar::Number(F64::upcast_scalar(F64::from(*v))))
        }
        PrimitiveLiteral::String(v) => Some(Scalar::String(v.clone())),
        _ => None,
    }
}
