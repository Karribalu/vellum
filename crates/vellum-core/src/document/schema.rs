#[derive(Debug, Clone)]
pub struct FieldType {
    pub indexed: bool,
    pub stored: bool,
    pub with_positions: bool,
    pub with_offsets: bool,
}

impl FieldType {
    pub const TEXT_INDEXED: FieldType = FieldType {
        indexed: true,
        stored: false,
        with_positions: true,
        with_offsets: false,
    };
    pub const TEXT_STORED: FieldType = FieldType {
        indexed: false,
        stored: true,
        with_positions: false,
        with_offsets: false,
    };
    pub const TEXT_INDEXED_AND_STORED: FieldType = FieldType {
        indexed: true,
        stored: true,
        with_positions: true,
        with_offsets: false,
    };
}

#[derive(Debug, Clone)]
pub enum FieldValue {
    Text(String),
    // Extend later (numeric, bytes, etc.)
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub value: FieldValue,
    pub field_type: FieldType,
}

impl Field {
    pub fn text(name: impl Into<String>, value: impl Into<String>, field_type: FieldType) -> Self {
        Self {
            name: name.into(),
            value: FieldValue::Text(value.into()),
            field_type,
        }
    }
}

