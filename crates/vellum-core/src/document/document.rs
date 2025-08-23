use crate::document::schema::Field;

#[derive(Debug, Clone, Default)]
pub struct Document {
    pub fields: Vec<Field>,
}

impl Document {
    pub fn new() -> Self {
        Self { fields: Vec::new() }
    }
    pub fn add_field(&mut self, field: Field) {
        self.fields.push(field);
    }
}