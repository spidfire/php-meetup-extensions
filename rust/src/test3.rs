use ext_php_rs::php_function;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use serde_json::Value;
use ext_php_rs::prelude::*;



#[php_function]
fn json_aggregate_count_occurrences(filename: String, key: String) -> PhpResult<HashMap<String, i32>> {
 

    let file = File::open(filename.clone()).map_err(|_| format!("Could not open file: {} !", filename))?;
    let reader = BufReader::new(file);
    let mut severities_count: HashMap<String, i32> = HashMap::new();

    for line in reader.lines() {
        let line = line.map_err(|_| "Could not convert into u32")?;
        let json: Value = serde_json::from_str(&line).unwrap();
        if let Some(severity) = json[key.clone()].as_str() {
            *severities_count.entry(severity.to_string()).or_insert(0) += 1;
        } else {
            *severities_count.entry("null".to_string()).or_insert(0) += 1;
        }
    }


    Ok(severities_count)
}
