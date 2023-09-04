use ext_php_rs::prelude::*;
use ext_php_rs::{info_table_start, info_table_row, info_table_end, php_module};
use ext_php_rs::zend::ModuleEntry;
use std::collections::HashMap;


pub mod test2;
use crate::test2::_internal_php_greet;

pub mod test3;
use crate::test3::_internal_php_json_aggregate_count_occurrences;


pub mod test4;
use crate::test4::_internal_php_resize;
use crate::test4::ImageResize;


pub extern "C" fn php_module_info(_module: *mut ModuleEntry) {
    info_table_start!();
    info_table_row!("My first rust extension", "enabled");
    info_table_end!();
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.info_function(php_module_info)
}