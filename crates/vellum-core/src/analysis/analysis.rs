#[derive(Debug, Clone)]
pub struct Token {
    pub term: String,
    pub position: u32, // Defines the position of the token in the original text.
    pub start_offset: u32, // Byte offset where the token starts in the original text.
    pub end_offset: u32, // Byte offset where the token ends in the original text.
}

pub trait Analyzer {
    fn tokenize(&self, field: &str, text: &str) -> Vec<Token>;
}
/// Minimal analyzer: split on ASCII whitespace, positions increment by 1.
#[derive(Default)]
pub struct WhitespaceAnalyzer;

impl Analyzer for WhitespaceAnalyzer {
    fn tokenize(&self, field: &str, text: &str) -> Vec<Token> {
        let mut pos = 0u32;
        let mut out = Vec::new();
        let mut offset = 0u32;
        for part in text.split_whitespace() {
            let start = text[offset as usize..].find(part).unwrap_or(0) as u32 + offset;
            let end = start + part.len() as u32;

            out.push(Token {
                term: part.to_string(),
                position: pos,
                start_offset: start,
                end_offset: end,
            });
            pos += 1;
            offset = end + 1;
        }
        out
    }
}
