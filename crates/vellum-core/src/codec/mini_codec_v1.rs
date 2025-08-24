use crate::index::segment::SegmentWriter;
use crate::index::terms::Term;
use crate::store::directory::Directory;
use std::collections::BTreeMap;
use std::io::{BufWriter, Seek, Write};

/**
Minimal on-disk layout for a segment, version 1.
- {seg}.si : segment info (a very simple format)
- {seg}.terms: sorted terms : "<field>\t<term>\t<df>\t<offset>\n"
- {seg}.post: posting bytes; each posting is encoded as:
    [varint doc_freq][delta-docIds][varint tfs][positions...]
    where delta-docIds are the gaps between docIds, and positions are the term positions in the document.
*/
/// This is a toy format to get started; not compatible with Lucene.
pub struct MiniCodecV1;

impl MiniCodecV1 {
    pub fn write_segment<D: Directory>(
        dir: &D,
        segw: &mut SegmentWriter<'_>,
    ) -> std::io::Result<()> {
        let post_name = format!("{}.post", segw.seg_info.name);
        let terms_name = format!("{}.terms", segw.seg_info.name);
        let si_name = format!("{}.si", segw.seg_info.name);

        let mut post_out = BufWriter::new(dir.create_output(&post_name)?);
        let mut terms_to_offset: BTreeMap<Term, (u64, u32)> = BTreeMap::new(); // term -> (offset in post, df)

        for (term, plist) in segw.terms.iter_mut() {
            plist.sort_by_doc_id();
            let offset = post_out.stream_position()?;
            write_varint(&mut post_out, plist.postings.len() as u64)?; // doc freq
            let mut last_doc_id = 0u32;
            for posting in &plist.postings {
                let delta = posting.doc_id.0 - last_doc_id;
                last_doc_id = posting.doc_id.0;
                write_varint(&mut post_out, delta as u64)?; // delta doc id
                write_varint(&mut post_out, posting.freq as u64)?; // term freq

                for pos in &posting.positions {
                    write_varint(&mut post_out, *pos as u64)?; // positions
                }
            }
            terms_to_offset.insert(term.clone(), (offset, plist.postings.len() as u32));
        }
        post_out.flush()?;

        let mut terms_out = BufWriter::new(dir.create_output(&terms_name)?);
        for (term, (offset, df)) in &terms_to_offset {
            writeln!(
                terms_out,
                "{}\t{}\t{}\t{}",
                term.field, term.text, df, offset
            )?;
        }
        terms_out.flush()?;
        let mut si_out = BufWriter::new(dir.create_output(&si_name)?);
        writeln!(si_out, "name={}", segw.seg_info.name)?;
        writeln!(si_out, "max_doc={}", segw.seg_info.max_doc)?;
        writeln!(si_out, "codec={}", segw.seg_info.codec)?;
        writeln!(si_out, "files={},{},{}", si_name, terms_name, post_name)?;

        si_out.flush()?;
        segw.seg_info.files = vec![si_name, terms_name, post_name];
        Ok(())
    }
}

// We are using little endian format here
fn write_varint<W: std::io::Write>(mut w: W, mut v: u64) -> std::io::Result<()> {
    while v >= 0x80 {
        w.write_all(&[((v as u8) & 0x7F) | 0x80])?;
        v >>= 7;
    }
    w.write_all(&[v as u8])?;
    Ok(())
}
