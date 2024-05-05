use std::fmt;

use super::cell::CellValue;

#[derive(Debug)]
pub struct Column {
    pub key: String,
    pub r#type: DataType,
    pub constraints: ColumnConstraint,
}

impl Column {
    pub fn new(key: String, r#type: DataType, constraints: ColumnConstraint) -> Self {
        Self {
            key,
            r#type,
            constraints,
        }
    }

    pub fn accepts_value(&self, cell_value: &CellValue) -> bool {
        match self.r#type {
            DataType::String => match cell_value {
                CellValue::String(_) => true,
                _ => false,
            },
            DataType::Integer => match cell_value {
                CellValue::Integer(_) => true,
                _ => false,
            },
            DataType::Float => match cell_value {
                CellValue::Float(_) => true,
                _ => false,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum DataType {
    String,
    Integer,
    Float,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DataType::String => write!(f, "String"),
            DataType::Integer => write!(f, "Integer"),
            DataType::Float => write!(f, "Float"),
        }
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ColumnConstraint: u8 {
        const NotNull    = 0b0000001;
        const Unique     = 0b0000010;
        const PrimaryKey = 0b0000100;
    }
}
