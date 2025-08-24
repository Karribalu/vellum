use crate::analysis::analysis::Analyzer;
use crate::document::document::Document;
use crate::document::schema::FieldValue::Text;
use crate::index::postings::{DocId, PostingsList};
use crate::index::terms::Term;
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone)]
pub struct SegmentInfo {
    pub name: String,
    pub max_doc: u32,
    pub files: Vec<String>,
    pub codec: String, // We are not using codec yet, but we will need it in the future
}

impl SegmentInfo {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            max_doc: 0,
            files: Vec::new(),
            codec: "MiniCodecV1".to_string(),
        }
    }
}

pub struct SegmentWriter<'a> {
    pub seg_info: SegmentInfo,
    analyzer: &'a dyn Analyzer,
    pub terms: BTreeMap<Term, PostingsList>,
    pub stored: Vec<HashMap<String, String>>,
    next_doc_id: u32,
}

impl<'a> SegmentWriter<'a> {
    pub fn new(name: impl Into<String>, analyzer: &'a dyn Analyzer) -> Self {
        Self {
            seg_info: SegmentInfo::new(name),
            analyzer,
            terms: BTreeMap::new(),
            stored: vec![],
            next_doc_id: 0,
        }
    }

    pub fn add_document(&mut self, doc: &Document) {
        let doc_id = DocId(self.next_doc_id);

        let mut stored_map = HashMap::new();

        for field in &doc.fields {
            match (
                &field.value,
                &field.field_type.indexed,
                field.field_type.stored,
            ) {
                (Text(text), true, stored) => {
                    for token in self.analyzer.tokenize(&field.name, text) {
                        let term = Term::new(&field.name, token.term);
                        let entry = self.terms.entry(term).or_insert_with(PostingsList::new);
                        entry.add_occurrence(doc_id, Some(token.position))
                    }
                    if stored {
                        stored_map.insert(field.name.clone(), text.clone());
                    }
                }
                (Text(text), false, true) => {
                    stored_map.insert(field.name.clone(), text.clone());
                }
                _ => {}
            }
        }
        if !stored_map.is_empty() {
            self.stored.push(stored_map);
        } else {
            self.stored.push(HashMap::new());
        }
        self.next_doc_id += 1;
        self.seg_info.max_doc = self.next_doc_id;
    }
}
