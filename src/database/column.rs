use super::cell::CellValue;

#[derive(Debug)]
pub struct Column {
    pub key: String,
    pub r#type: CellValue,
    pub constraints: ColumnConstraint,
}

impl Column {
    pub fn new(key: String, r#type: CellValue, constraints: ColumnConstraint) -> Self {
        Self {
            key,
            r#type,
            constraints,
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
