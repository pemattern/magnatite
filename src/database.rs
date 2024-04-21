use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::BadKeyError;
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone)]
pub struct Database {
    tables: HashMap<Key, Table>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            tables: HashMap::new(),
        }
    }

    pub fn set(&mut self, table_key: Key, record_key: Key, record: Record) {
        match self.tables.get_mut(&table_key) {
            Some(table) => {
                let _ = table.upsert_record(record_key, record);
            }
            None => {
                let mut new_table = Table::new(table_key.clone());
                let _ = new_table.upsert_record(record_key, record);
                self.tables.insert(table_key, new_table);
            }
        }
    }

    pub fn get(&self, table_key: &Key, record_key: &Key) -> Result<&Record, BadKeyError> {
        match self.tables.get(table_key) {
            Some(table) => match table.get_record(record_key) {
                Ok(entry) => Ok(entry),
                Err(error) => Err(error),
            },
            None => Err(BadKeyError::Table {
                key: table_key.clone(),
            }),
        }
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct Table {
    key: Key,
    records: HashMap<Key, Record>,
}

impl Table {
    pub fn new(table_key: Key) -> Self {
        Self {
            key: table_key,
            records: HashMap::new(),
        }
    }

    pub fn get_record(&self, record_key: &Key) -> Result<&Record, BadKeyError> {
        self.records.get(record_key).ok_or(BadKeyError::Record {
            key: record_key.clone(),
            table_key: self.key.clone(),
        })
    }

    pub fn upsert_record(&mut self, record_key: Key, record: Record) -> Result<(), BadKeyError> {
        match self.records.insert(record_key, record) {
            Some(_) => Ok(()),
            None => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Record {
    columns: HashMap<Key, ColumnValue>,
}

impl Record {
    pub fn new() -> Self {
        Self {
            columns: HashMap::new(),
        }
    }

    pub fn set_data(&mut self, data: HashMap<String, String>) {
        for column in data {
            self.columns
                .insert(Key::String(column.0), ColumnValue::String(column.1));
        }
    }

    pub fn set_column_value(&mut self, column_key: Key, value: ColumnValue) {
        self.columns.insert(column_key, value);
        ()
    }

    pub fn get_all_values(&self) -> &HashMap<Key, ColumnValue> {
        &self.columns
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColumnValue {
    ForeignKey(String),
    String(String),
    Integer(i64),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Key {
    Integer(i64),
    String(String),
    Uuid(Uuid),
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Key::Integer(value) => write!(f, "{}", value),
            Key::String(value) => write!(f, "{}", value),
            Key::Uuid(value) => write!(f, "{}", value),
        }
    }
}
