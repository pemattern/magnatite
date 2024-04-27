use std::collections::HashMap;

use crate::error::{Error, Result};

use super::{
    cell::CellValue,
    column::{Column, ColumnConstraint, ColumnType},
    record::Record,
};

#[derive(Debug)]
pub struct Table {
    key: String,
    records: HashMap<String, Record>,
    columns: HashMap<String, Column>,
}

impl Table {
    pub fn new(key: String, columns: HashMap<String, Column>) -> Self {
        Self {
            key,
            records: HashMap::new(),
            columns,
        }
    }

    pub fn add_column(
        &mut self,
        key: String,
        r#type: ColumnType,
        constraints: ColumnConstraint,
    ) -> Result<()> {
        if self.columns.contains_key(&key) {
            return Err(Error::ColumnAlreadyExists {
                key,
                table_key: self.key.clone(),
            });
        }

        if constraints.contains(ColumnConstraint::PrimaryKey)
            && self
                .columns
                .iter()
                .any(|c| c.1.constraints.contains(ColumnConstraint::PrimaryKey))
        {
            return Err(Error::PrimaryColumnAlreadyExists {
                key,
                table_key: self.key.clone(),
            });
        }

        let column = Column::new(key.clone(), r#type, constraints);
        self.columns.insert(key, column);
        Ok(())
    }

    pub fn add_record(
        &mut self,
        key: String,
        values: HashMap<&Column, CellValue>,
    ) -> Result<Record> {
        if self.records.contains_key(&key) {
            return Err(Error::RecordAlreadyExists {
                key,
                table_key: self.key.clone(),
            });
        }
        let mut record = Record::new();
        for value in values {
            record.insert(value.0.key.clone(), value.1);
        }
        Ok(record)
    }

    pub fn get_record(&self, key: &String) -> Result<&Record> {
        self.records.get(key).ok_or(Error::BadRecordKey {
            key: key.clone(),
            table_key: self.key.clone(),
        })
    }

    pub fn get_record_mut(&mut self, key: &String) -> Result<&mut Record> {
        self.records.get_mut(key).ok_or(Error::BadRecordKey {
            key: key.clone(),
            table_key: self.key.clone(),
        })
    }
}
