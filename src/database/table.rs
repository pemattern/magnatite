use std::collections::HashMap;

use crate::error::{Error, Result};

use super::{
    cell::CellValue,
    column::{Column, ColumnConstraint, DataType},
    record::Record,
};

#[derive(Debug)]
pub struct Table {
    pub key: String,
    records: Vec<Record>,
    columns: Vec<Column>,
}

impl Table {
    pub fn new(key: String, columns: Option<Vec<Column>>) -> Self {
        println!("Created table {}", &key);
        Self {
            key,
            records: Vec::new(),
            columns: columns.unwrap_or_else(Vec::new),
        }
    }

    pub fn add_column(
        &mut self,
        key: String,
        r#type: DataType,
        constraints: ColumnConstraint,
    ) -> Result<&Column> {
        if self.columns.iter().any(|c| c.key == key) {
            return Err(Error::ColumnAlreadyExists {
                key: key.clone(),
                table_key: self.key.clone(),
            });
        }

        if constraints.contains(ColumnConstraint::PrimaryKey)
            && self
                .columns
                .iter()
                .any(|c| c.constraints.contains(ColumnConstraint::PrimaryKey))
        {
            return Err(Error::PrimaryColumnAlreadyExists {
                key: key.clone(),
                table_key: self.key.clone(),
            });
        }

        let column = Column::new(key, r#type, constraints);
        self.columns.push(column);
        Ok(self.columns.last().unwrap())
    }

    pub fn add_record(&mut self, values: HashMap<String, CellValue>) -> Result<&Record> {
        let record = Record::new();
        for value in values {
            let Some(column) = self.columns.iter().find(|c| c.key == value.0) else {
                return Err(Error::BadColumnKey {
                    key: value.0,
                    table_key: self.key.clone(),
                });
            };

            if !column.accepts_value(&value.1) {
                return Err(Error::DataTypeMismatch {
                    column_key: column.key.clone(),
                    expected: column.r#type.to_string(),
                    received: value.1.to_string(),
                });
            }

            // TODO: Check constraints
        }
        println!("Added record {:?}", record);
        self.records.push(record);
        Ok(self.records.last().unwrap())
    }
}
