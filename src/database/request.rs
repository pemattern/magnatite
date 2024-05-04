use std::collections::HashMap;

use super::{cell::CellValue, column::Column};

#[derive(Debug)]
pub enum DatabaseRequest {
    CreateTable {
        key: String,
        columns: Option<Vec<Column>>,
    },
    InsertInto {
        key: String,
        data: HashMap<String, CellValue>,
    },
}
