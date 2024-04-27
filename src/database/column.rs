#[derive(Debug, Clone)]
pub struct Column {
    pub key: String,
    pub r#type: ColumnType,
    pub constraints: ColumnConstraint,
}

impl Column {
    pub fn new(key: String, r#type: ColumnType, constraints: ColumnConstraint) -> Self {
        Self {
            key,
            r#type,
            constraints,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ColumnType {
    String,
    Integer,
    Float,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct ColumnConstraint: u8 {
        const NotNull    = 0b0000001;
        const Unique     = 0b0000010;
        const PrimaryKey = 0b0000100;
    }
}
