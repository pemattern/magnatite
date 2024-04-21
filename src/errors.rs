use std::fmt::{Display, Formatter};

use crate::database::Key;

#[derive(Debug)]
pub enum BadKeyError {
    Table { key: Key },
    Record { key: Key, table_key: Key },
    Column { key: Key, table_key: Key },
}

impl Display for BadKeyError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            BadKeyError::Table { key } => write!(f, "table with key `{}` not found", key),
            BadKeyError::Record { key, table_key } => write!(
                f,
                "record with key `{}` not found in table `{}`",
                key, table_key
            ),
            BadKeyError::Column { key, table_key } => write!(
                f,
                "column with key `{}` not found in table `{}`",
                key, table_key
            ),
        }
    }
}
