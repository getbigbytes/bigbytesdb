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

use bigbytes_common_catalog::plan::PartInfo;
use bigbytes_common_catalog::plan::PartStatistics;
use bigbytes_common_catalog::plan::Partitions;
use bigbytes_common_catalog::plan::PartitionsShuffleKind;
use bigbytes_common_catalog::plan::StageTableInfo;
use bigbytes_common_catalog::table_context::TableContext;
use bigbytes_storages_common_stage::SingleFilePartition;

pub async fn read_partitions_simple(
    ctx: Arc<dyn TableContext>,
    stage_table_info: &StageTableInfo,
) -> bigbytes_common_exception::Result<(PartStatistics, Partitions)> {
    let thread_num = ctx.get_settings().get_max_threads()? as usize;

    let files = if let Some(files) = &stage_table_info.files_to_copy {
        files.clone()
    } else {
        stage_table_info.list_files(thread_num, None).await?
    };
    let size = files.iter().map(|f| f.size as usize).sum();
    // assuming all fields are empty
    let max_rows = std::cmp::max(size / (stage_table_info.schema.fields.len() + 1), 1);
    let statistics = PartStatistics {
        snapshot: None,
        read_rows: max_rows,
        read_bytes: size,
        partitions_scanned: 0,
        partitions_total: files.len(),
        is_exact: false,
        pruning_stats: Default::default(),
    };

    let partitions = files
        .into_iter()
        .map(|v| {
            let part = SingleFilePartition {
                path: v.path.clone(),
                size: v.size as usize,
            };
            let part_info: Box<dyn PartInfo> = Box::new(part);
            Arc::new(part_info)
        })
        .collect::<Vec<_>>();

    Ok((
        statistics,
        Partitions::create(PartitionsShuffleKind::Seq, partitions),
    ))
}
