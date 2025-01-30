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

use std::fs::File;
use std::io::Write;
use std::path::Path;

use bigbytesdb_common_expression::hilbert_compact_state_list;

pub fn codegen_hilbert_lut() {
    let dest = Path::new("src/query/expression/src/hilbert");
    let path = dest.join("lut.rs");

    let mut file = File::create(path).expect("open");

    // Write the head.
    let codegen_src_path = file!();
    writeln!(
        file,
        "// Copyright 2024 Digitrans Inc
//
// Licensed under the Apache License, Version 2.0 (the \"License\");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an \"AS IS\" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// This code is generated by {codegen_src_path}. DO NOT EDIT.

use std::sync::LazyLock;"
    )
    .unwrap();

    writeln!(
        file,
        "
pub static LUT: LazyLock<Vec<&'static [u16]>> = LazyLock::new(|| vec![&LUT_2, &LUT_3, &LUT_4, &LUT_5]);
\n{}\n\n{}\n\n{}\n\n{}",
        generate_hilbert_lut(2),
        generate_hilbert_lut(3),
        generate_hilbert_lut(4),
        generate_hilbert_lut(5),
    )
    .unwrap();
    file.flush().unwrap();
}

fn generate_hilbert_lut(dimension: usize) -> String {
    let lut = hilbert_compact_state_list(dimension);
    // Generate the name of the array based on the size
    let lut_name = format!("LUT_{}", dimension);

    let size = lut.len();
    // Generate the array elements as a formatted string
    let array_elements: String = lut
        .into_iter()
        .enumerate()
        .map(|(i, v)| {
            if i % 16 == 15 {
                format!("{},\n    ", v)
            } else {
                format!("{}, ", v)
            }
        })
        .collect();

    // Create the final array code string with proper formatting
    let array_code = format!(
        "const {}: [u16; {}] = [\n    {}\n];",
        lut_name,
        size,
        array_elements.trim_end()
    );

    array_code
}
