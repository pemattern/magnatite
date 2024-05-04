use super::{column::Column, request::DatabaseRequest, table::Table};
use crate::error::{Error, Result};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Database {
    tables: Vec<Table>,
}

impl Database {
    pub fn new() -> Self {
        Self { tables: Vec::new() }
    }

    pub fn process_requests(&mut self, requests: Vec<DatabaseRequest>) -> Result<()> {
        for request in requests {
            match request {
                DatabaseRequest::CreateTable { key, columns } => {
                    let _ = match self.add_table(key, columns) {
                        Ok(_) => {} // TODO: replace with success infos
                        Err(error) => return Err(error),
                    };
                }
                DatabaseRequest::InsertInto { key, data } => {
                    let table = match self.get_table_mut(&key) {
                        Ok(table) => table,
                        Err(error) => return Err(error),
                    };
                    let _ = match table.add_record(data) {
                        Ok(_) => {}
                        Err(error) => return Err(error),
                    };
                }
            }
        }

        Ok(())
    }

    fn add_table(&mut self, key: String, columns: Option<Vec<Column>>) -> Result<&Table> {
        if self.tables.iter().any(|t| t.key == key) {
            return Err(Error::TableAlreadyExists { key: key.clone() });
        }

        let table = Table::new(key, columns);
        self.tables.push(table);
        self.tables.last().ok_or(Error::NotImplemented)
    }

    fn get_table(&self, key: &String) -> Result<&Table> {
        self.tables
            .iter()
            .find(|t| t.key == *key)
            .ok_or(Error::BadTableKey { key: key.clone() })
    }

    fn get_table_mut(&mut self, key: &String) -> Result<&mut Table> {
        self.tables
            .iter_mut()
            .find(|t| t.key == *key)
            .ok_or(Error::BadTableKey { key: key.clone() })
    }
}
