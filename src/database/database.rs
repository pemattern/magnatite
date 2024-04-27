use super::{column::Column, table::Table};
use crate::error::{Error, Result};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Database {
    tables: HashMap<String, Table>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn add_table(&mut self, key: String, columns: HashMap<String, Column>) -> Result<()> {
        if self.tables.contains_key(&key) {
            return Err(Error::TableAlreadyExists { key });
        }

        let table = Table::new(key.clone(), columns);
        self.tables.insert(key, table);
        Ok(())
    }

    pub fn get_table(&self, key: &String) -> Result<&Table> {
        self.tables
            .get(key)
            .ok_or(Error::BadTableKey { key: key.clone() })
    }

    pub fn get_table_mut(&mut self, key: &String) -> Result<&mut Table> {
        self.tables
            .get_mut(key)
            .ok_or(Error::BadTableKey { key: key.clone() })
    }
}
