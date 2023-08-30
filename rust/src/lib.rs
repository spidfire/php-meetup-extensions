// Copyright (c) 2022 PHPER Framework Team
// PHPER is licensed under Mulan PSL v2.
// You can use this software according to the terms and conditions of the Mulan
// PSL v2. You may obtain a copy of Mulan PSL v2 at:
//          http://license.coscl.org.cn/MulanPSL2
// THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY
// KIND, EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO
// NON-INFRINGEMENT, MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
// See the Mulan PSL v2 for more details.

use phper::{functions::Argument, modules::Module, php_get_module, values::ZVal, arrays::ZArray};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use serde_json::Value;

/// The php function, receive arguments with type `ZVal`.
fn json_aggregate_count_occurrences(arguments: &mut [ZVal]) -> phper::Result<ZArray> {
 
    let filename = arguments[0].expect_z_str()?.to_str()?;
    let key = arguments[1].expect_z_str()?.to_str()?;


    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut severities_count: HashMap<String, i32> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let json: Value = serde_json::from_str(&line).unwrap();
        if let Some(severity) = json[key].as_str() {
            *severities_count.entry(severity.to_string()).or_insert(0) += 1;
        } else {
            *severities_count.entry("null".to_string()).or_insert(0) += 1;
        }
    }
    let mut result = ZArray::new();
    for (key, value) in severities_count {
        result.insert(key.as_bytes(), ZVal::from(value as i64));
    }

    Ok(result)
}

/// This is the entry of php extension, the attribute macro `php_get_module`
/// will generate the `extern "C" fn`.
#[php_get_module]
pub fn get_module() -> Module {
    // New `Module` with extension info.
    let mut module = Module::new(
        env!("CARGO_CRATE_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_AUTHORS"),
    );

    // Register function `json_aggregate_count_occurrences`, with one argument `name`.
    module
        .add_function("json_aggregate_count_occurrences", json_aggregate_count_occurrences)
        .argument(Argument::by_val("name"));

    module
}