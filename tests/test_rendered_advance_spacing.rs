//! Tests for rendered_advance including Tc/Tw character and word spacing.
//!
//! rendered_advance is the full cursor advance after a character (equiv. to
//! Poppler's dx): glyph advance + Tc (+ Tw for U+0020). This differs from
//! advance_width, which is only the glyph's own width.

use pdf_oxide::PdfDocument;

/// Build a minimal 1-page PDF where the content stream sets character spacing
/// (Tc) to a known value before rendering a short string.
fn pdf_with_char_spacing(tc: f32) -> Vec<u8> {
    let content = format!(
        "BT /F0 12 Tf 1 0 0 1 100 700 Tm {tc} Tc (ABC) Tj ET\n"
    );

    let mut out: Vec<u8> = Vec::new();
    let mut offsets: Vec<usize> = vec![0];
    out.extend_from_slice(b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n");

    let push = |out: &mut Vec<u8>, offsets: &mut Vec<usize>, body: &str| {
        offsets.push(out.len());
        let id = offsets.len() - 1;
        out.extend_from_slice(format!("{id} 0 obj\n{body}\nendobj\n").as_bytes());
    };

    push(&mut out, &mut offsets, "<< /Type /Catalog /Pages 2 0 R >>");
    push(&mut out, &mut offsets, "<< /Type /Pages /Kids [3 0 R] /Count 1 >>");
    push(
        &mut out,
        &mut offsets,
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 600 900] \
         /Resources << /Font << /F0 5 0 R >> >> /Contents 4 0 R >>",
    );
    push(
        &mut out,
        &mut offsets,
        &format!("<< /Length {} >>\nstream\n{content}\nendstream", content.len() + 1),
    );
    push(&mut out, &mut offsets, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>");

    let xref_offset = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n", offsets.len()).as_bytes());
    out.extend_from_slice(b"0000000000 65535 f \n");
    for &off in &offsets[1..] {
        out.extend_from_slice(format!("{off:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref_offset}\n%%EOF\n",
            offsets.len()
        )
        .as_bytes(),
    );
    out
}

/// Build a minimal 1-page PDF with both character spacing (Tc) and word
/// spacing (Tw) set, and a string that includes a space character.
fn pdf_with_word_spacing(tc: f32, tw: f32) -> Vec<u8> {
    let content = format!(
        "BT /F0 12 Tf 1 0 0 1 100 700 Tm {tc} Tc {tw} Tw (A B) Tj ET\n"
    );

    let mut out: Vec<u8> = Vec::new();
    let mut offsets: Vec<usize> = vec![0];
    out.extend_from_slice(b"%PDF-1.4\n%\xE2\xE3\xCF\xD3\n");

    let push = |out: &mut Vec<u8>, offsets: &mut Vec<usize>, body: &str| {
        offsets.push(out.len());
        let id = offsets.len() - 1;
        out.extend_from_slice(format!("{id} 0 obj\n{body}\nendobj\n").as_bytes());
    };

    push(&mut out, &mut offsets, "<< /Type /Catalog /Pages 2 0 R >>");
    push(&mut out, &mut offsets, "<< /Type /Pages /Kids [3 0 R] /Count 1 >>");
    push(
        &mut out,
        &mut offsets,
        "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 600 900] \
         /Resources << /Font << /F0 5 0 R >> >> /Contents 4 0 R >>",
    );
    push(
        &mut out,
        &mut offsets,
        &format!("<< /Length {} >>\nstream\n{content}\nendstream", content.len() + 1),
    );
    push(&mut out, &mut offsets, "<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>");

    let xref_offset = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n", offsets.len()).as_bytes());
    out.extend_from_slice(b"0000000000 65535 f \n");
    for &off in &offsets[1..] {
        out.extend_from_slice(format!("{off:010} 00000 n \n").as_bytes());
    }
    out.extend_from_slice(
        format!(
            "trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref_offset}\n%%EOF\n",
            offsets.len()
        )
        .as_bytes(),
    );
    out
}

/// `rendered_advance` for a non-space character must exceed `advance_width` by
/// exactly Tc (character spacing) when Tc > 0.
#[test]
fn rendered_advance_includes_char_spacing() {
    let tc = 2.0_f32;
    let pdf = pdf_with_char_spacing(tc);
    let tmp = tempfile::NamedTempFile::new().expect("temp");
    std::fs::write(tmp.path(), &pdf).unwrap();

    let doc = PdfDocument::open(tmp.path()).expect("open");
    let chars = doc.extract_chars(0).expect("extract_chars");

    // Filter to non-whitespace chars from the "ABC" string.
    let letters: Vec<_> = chars.iter().filter(|c| !c.char.is_whitespace()).collect();
    assert!(!letters.is_empty(), "expected to extract 'A', 'B', 'C'");

    for ch in &letters {
        let delta = ch.rendered_advance - ch.advance_width;
        assert!(
            (delta - tc).abs() < 0.5,
            "char {:?}: rendered_advance - advance_width = {delta:.3}, expected Tc={tc}",
            ch.char
        );
    }
}

/// `rendered_advance` for a space character must exceed `advance_width` by
/// Tc + Tw when both character spacing and word spacing are set.
#[test]
fn rendered_advance_includes_word_spacing_for_space() {
    let tc = 1.0_f32;
    let tw = 3.0_f32;
    let pdf = pdf_with_word_spacing(tc, tw);
    let tmp = tempfile::NamedTempFile::new().expect("temp");
    std::fs::write(tmp.path(), &pdf).unwrap();

    let doc = PdfDocument::open(tmp.path()).expect("open");
    let chars = doc.extract_chars(0).expect("extract_chars");

    let spaces: Vec<_> = chars.iter().filter(|c| c.char == ' ').collect();
    assert!(!spaces.is_empty(), "expected a space character in 'A B'");

    for sp in &spaces {
        let delta = sp.rendered_advance - sp.advance_width;
        let expected = tc + tw;
        assert!(
            (delta - expected).abs() < 0.5,
            "space: rendered_advance - advance_width = {delta:.3}, expected Tc+Tw={expected}"
        );
    }
}

/// With Tc = 0 and Tw = 0, rendered_advance must equal advance_width for every
/// character (no extra spacing is added).
#[test]
fn rendered_advance_equals_advance_width_when_no_spacing() {
    let pdf = pdf_with_char_spacing(0.0);
    let tmp = tempfile::NamedTempFile::new().expect("temp");
    std::fs::write(tmp.path(), &pdf).unwrap();

    let doc = PdfDocument::open(tmp.path()).expect("open");
    let chars = doc.extract_chars(0).expect("extract_chars");

    let letters: Vec<_> = chars.iter().filter(|c| !c.char.is_whitespace()).collect();
    assert!(!letters.is_empty(), "expected to extract 'A', 'B', 'C'");

    for ch in &letters {
        let delta = (ch.rendered_advance - ch.advance_width).abs();
        assert!(
            delta < 0.1,
            "char {:?}: rendered_advance ({:.3}) should equal advance_width ({:.3}) when Tc=0",
            ch.char,
            ch.rendered_advance,
            ch.advance_width
        );
    }
}
