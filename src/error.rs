use std::fmt::{Display, Formatter};

use sqlparser::parser::ParserError;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BadSql {
        error: ParserError,
    },
    BadTableKey {
        key: String,
    },
    BadRecordKey {
        key: String,
        table_key: String,
    },
    BadColumnKey {
        key: String,
        table_key: String,
    },
    DataTypeMismatch {
        column_key: String,
        expected: String,
        received: String,
    },
    TableAlreadyExists {
        key: String,
    },
    RecordAlreadyExists {
        key: String,
        table_key: String,
    },
    ColumnAlreadyExists {
        key: String,
        table_key: String,
    },
    PrimaryColumnAlreadyExists {
        key: String,
        table_key: String,
    },
    NotImplemented,
    Placeholder {
        text: String,
    },
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Error::BadSql { error } => write!(f, "{}", error),
            Error::BadTableKey { key } => write!(f, "table with key `{}` not found", key),
            Error::BadRecordKey { key, table_key } => write!(
                f,
                "record with key `{}` not found in table `{}`",
                key, table_key
            ),
            Error::BadColumnKey { key, table_key } => write!(
                f,
                "column with key `{}` not found in table `{}`",
                key, table_key
            ),
            Error::DataTypeMismatch {
                column_key,
                expected,
                received,
            } => write!(
                f,
                "column `{}` with type `{}` cannot accept values of type `{}`",
                column_key, expected, received,
            ),
            Error::NotImplemented => write!(f, "feature not implemented"),
            Error::TableAlreadyExists { key } => {
                write!(f, "table with key `{}` already exists", key)
            }
            Error::RecordAlreadyExists { key, table_key } => {
                write!(
                    f,
                    "record with key `{}` already exists in table `{}`",
                    key, table_key
                )
            }
            Error::ColumnAlreadyExists { key, table_key } => write!(
                f,
                "column with key `{}` already exists on table `{}`",
                key, table_key
            ),
            Error::PrimaryColumnAlreadyExists { key, table_key } => write!(
                f,
                "cannot add primary column `{}` to table `{}` as it already has a primary column",
                key, table_key
            ),
            Error::Placeholder { text } => write!(f, "{}", text),
        }
    }
}
