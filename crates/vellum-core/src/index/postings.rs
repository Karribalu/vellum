#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DocId(pub u32);

#[derive(Debug, Clone)]
pub struct Posting {
    pub doc_id: DocId,
    pub positions: Vec<u32>, // Positions of the term in the document
    pub freq: u32,
}

impl Posting {
    pub fn new(doc_id: DocId) -> Self {
        Self {
            doc_id,
            positions: Vec::new(),
            freq: 0,
        }
    }

    pub fn add_position(&mut self, position: u32) {
        self.positions.push(position);
        self.freq += 1;
    }
}

#[derive(Debug, Clone)]
pub struct PostingsList {
    pub postings: Vec<Posting>,
}

impl PostingsList {
    pub fn new() -> Self {
        Self {
            postings: Vec::new(),
        }
    }

    pub fn add_occurrence(&mut self, doc_id: DocId, maybe_pos: Option<u32>) {
        // We are implementing a single threaded indexed
        // We will process documents in order
        // While we are processing multiple postings of a single document,
        // We can use the last posting in the vector
        match self.postings.last_mut() {
            Some(last) if last.doc_id == doc_id => {
                if let Some(pos) = maybe_pos {
                    last.add_position(pos);
                } else {
                    last.freq += 1;
                }
                return;
            }
            _ => {}
        }
        let mut posting = Posting::new(doc_id);
        posting.freq = 1;
        if let Some(pos) = maybe_pos {
            posting.positions.push(pos);
        }
        self.postings.push(posting);
    }

    pub fn add_posting(&mut self, posting: Posting) {
        self.postings.push(posting);
    }

    pub fn get_posting(&self, doc_id: DocId) -> Option<&Posting> {
        self.postings.iter().find(|p| p.doc_id == doc_id)
    }

    pub fn sort_by_doc_id(&mut self) {
        self.postings.sort_by_key(|p| p.doc_id);
    }
}
