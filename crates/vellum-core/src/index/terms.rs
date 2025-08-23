#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Term {
    pub field: String,
    pub text: String,
}
impl Term {
    pub fn new(field: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            text: text.into(),
        }
    }
}

impl Ord for Term {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.field.cmp(&other.field) {
            std::cmp::Ordering::Equal => self.text.cmp(&other.text),
            ord => ord,
        }
    }
}
impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}