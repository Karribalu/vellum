use std::sync::atomic::AtomicU64;

use crate::{
    analysis::analysis::Analyzer, codec::mini_codec_v1::MiniCodecV1, document::document::Document,
    index::segment::SegmentWriter, store::directory::Directory,
};

pub struct IndexWriter<D: Directory> {
    dir: D,
    analyzer: Box<dyn Analyzer>,
    seg_seq: AtomicU64,
    current: Option<SegmentWriter<'static>>, // We will update to use lifetimes later
}

impl<D: Directory> IndexWriter<D> {
    pub fn new(dir: D, analyzer: Box<dyn Analyzer>) -> Self {
        Self {
            dir,
            analyzer,
            seg_seq: AtomicU64::new(0),
            current: None,
        }
    }
    fn next_segment_name(&self) -> String {
        let seq = self
            .seg_seq
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        format!("seg_{}", seq)
    }

    pub fn add_document(&mut self, doc: &Document) {
        if self.current.is_none() {
            let seg_name = self.next_segment_name();
            let analyzer = self.analyzer.as_ref();
            let seg_writer = SegmentWriter::new(seg_name, analyzer);
            // Safety: We are using 'static here for simplicity; in a real implementation, we would manage lifetimes properly.
            let seg_writer_static: SegmentWriter<'static> =
                unsafe { std::mem::transmute(seg_writer) };
            self.current = Some(seg_writer_static);
        }

        if let Some(seg_writer) = &mut self.current {
            seg_writer.add_document(doc);
        }
    }

    pub fn flush(&mut self) -> std::io::Result<()> {
        if let Some(seg_writer) = &mut self.current {
            MiniCodecV1::write_segment(&self.dir, seg_writer)?;
            self.current = None; // Reset current segment writer after flushing
        }
        Ok(())
    }
}

// Tests for IndexWriter
mod tests {
    use crate::{
        analysis::analysis::WhitespaceAnalyzer,
        document::{
            document::Document,
            schema::{Field, FieldType},
        },
        store::{
            directory::{Directory, FSDirectory},
            index_writer::IndexWriter,
        },
    };

    fn get_dummy_document() -> Document {
        let field = Field::text(
            "test",
            "This is a test document.",
            FieldType::TEXT_INDEXED_AND_STORED,
        );
        let mut doc = Document::new();
        doc.add_field(field);
        doc
    }

    #[test]
    fn verify_indexing_document() {
        // Create a filesystem directory in the OS home dir so MiniCodecV1 can write files.
        let tmp = std::env::home_dir().unwrap().join(format!(
            "vellum_test_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = std::fs::remove_dir_all(&tmp); // ignore error if not present
        let _ = std::fs::create_dir_all(&tmp);

        let analyzer = Box::new(WhitespaceAnalyzer::default());
        let fsdir = FSDirectory::new(&tmp).expect("create fs dir");
        let mut iw = IndexWriter::new(fsdir, analyzer);

        // Add one document and flush the segment to disk.
        iw.add_document(&get_dummy_document());
        iw.flush().expect("flush segment");

        // Check that the segment files were created and that max_doc is 1.
        let si_path = tmp.join("seg_0.si");
        let terms_path = tmp.join("seg_0.terms");
        let post_path = tmp.join("seg_0.post");
        println!("si_path: {:?}", si_path);
        assert!(si_path.exists(), "si file should exist");
        assert!(terms_path.exists(), "terms file should exist");
        assert!(post_path.exists(), "post file should exist");

        let si_contents = std::fs::read_to_string(&si_path).expect("read si");
        assert!(
            si_contents.contains("max_doc=1"),
            "si should record max_doc=1"
        );

        // Cleanup the temporary directory; ignore errors.
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
