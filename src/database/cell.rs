use std::fmt;

#[derive(Debug)]
pub enum CellValue {
    String(String),
    Integer(i64),
    Float(f64),
}

impl fmt::Display for CellValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CellValue::String(_) => write!(f, "String"),
            CellValue::Integer(_) => write!(f, "Integer"),
            CellValue::Float(_) => write!(f, "Float"),
        }
    }
}
