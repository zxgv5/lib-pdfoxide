//! Text rasterizer - renders PDF text using tiny-skia.
//!
//! Text rendering in PDF is complex because:
//! - Fonts may be embedded or use standard PDF fonts
//! - Character encoding varies (identity-H, MacRoman, custom ToUnicode, etc.)
//! - Glyph positioning is explicit via TJ arrays
//!
//! This module provides a text rendering implementation that:
//! - Uses system fonts as fallback when embedded fonts aren't available
//! - Renders text using rustybuzz for shaping and tiny-skia for drawing glyph paths

use super::create_fill_paint;
use crate::content::operators::TextElement;
use crate::content::GraphicsState;
use crate::document::PdfDocument;
use crate::error::{Error, Result};
use crate::object::Object;

use tiny_skia::{Paint, PathBuilder, Pixmap, Transform};
use ttf_parser::OutlineBuilder;

/// Outline builder that converts ttf-parser paths to tiny-skia paths.
struct SkiaOutlineBuilder<'a>(&'a mut PathBuilder);

impl<'a> OutlineBuilder for SkiaOutlineBuilder<'a> {
    fn move_to(&mut self, x: f32, y: f32) {
        self.0.move_to(x, y);
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.0.line_to(x, y);
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.0.quad_to(x1, y1, x, y);
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.0.cubic_to(x1, y1, x2, y2, x, y);
    }
    fn close(&mut self) {
        self.0.close();
    }
}

/// Rasterizer for PDF text operations.
pub struct TextRasterizer {
    /// Font database for system font fallback
    fontdb: fontdb::Database,
}

impl TextRasterizer {
    /// Create a new text rasterizer.
    pub fn new() -> Self {
        let mut fontdb = fontdb::Database::new();
        fontdb.load_system_fonts();
        Self { fontdb }
    }

    /// Render a text string (Tj operator).
    /// Returns the total horizontal advance in PDF points.
    pub fn render_text(
        &self,
        pixmap: &mut Pixmap,
        text: &[u8],
        base_transform: Transform,
        gs: &GraphicsState,
        resources: &Object,
        doc: &mut PdfDocument,
        clip_mask: Option<&tiny_skia::Mask>,
    ) -> Result<f32> {
        // Get font info from resources/doc
        let font_info = if let Some(font_name) = &gs.font_name {
            self.get_font_info(doc, resources, font_name).ok()
        } else {
            None
        };

        // Convert raw PDF bytes to Unicode string using font encoding
        let unicode_text = self.decode_text_to_unicode(text, font_info.as_ref());

        // Create paint from fill color
        let paint = create_fill_paint(gs, "Normal");

        // Find and load font
        let pdf_font_name = gs.font_name.as_deref().unwrap_or("Helvetica");
        if let Some((font_data, index)) = self.load_font_data(pdf_font_name) {
            Ok(self.render_unicode_text(
                pixmap,
                &unicode_text,
                &font_data,
                index,
                &paint,
                base_transform,
                gs,
                clip_mask,
            )?)
        } else {
            log::warn!("No system font found for {}, using fallback", pdf_font_name);
            // Fallback to simple rendering if font not found
            Ok(self.render_text_fallback(
                pixmap,
                &unicode_text,
                &paint,
                base_transform,
                gs,
                clip_mask,
            )?)
        }
    }

    /// Decode raw PDF text bytes to a Unicode string based on font type.
    fn decode_text_to_unicode(
        &self,
        text: &[u8],
        font_info: Option<&crate::fonts::FontInfo>,
    ) -> String {
        if let Some(info) = font_info {
            let mut result = String::new();
            let is_type0 = info.subtype == "Type0";

            let mut i = 0;
            while i < text.len() {
                let code = if is_type0 && i + 1 < text.len() {
                    // Type0 fonts usually use 2-byte CIDs
                    let c = ((text[i] as u32) << 8) | (text[i + 1] as u32);
                    i += 2;
                    c
                } else {
                    // Simple fonts use 1-byte character codes
                    let c = text[i] as u32;
                    i += 1;
                    c
                };

                if let Some(u) = info.char_to_unicode(code) {
                    result.push_str(&u);
                } else if code < 256 {
                    // Fallback for simple printable ASCII if mapping fails
                    let ch = code as u8 as char;
                    if ch.is_ascii_graphic() || ch == ' ' {
                        result.push(ch);
                    } else {
                        result.push('\u{FFFD}');
                    }
                } else {
                    result.push('\u{FFFD}');
                }
            }
            result
        } else {
            String::from_utf8_lossy(text).to_string()
        }
    }

    /// Render a TJ array (text with positioning adjustments).
    /// Returns the total horizontal advance in PDF points.
    pub fn render_tj_array(
        &self,
        pixmap: &mut Pixmap,
        array: &[TextElement],
        base_transform: Transform,
        gs: &GraphicsState,
        resources: &Object,
        doc: &mut PdfDocument,
        clip_mask: Option<&tiny_skia::Mask>,
    ) -> Result<f32> {
        let mut current_gs = gs.clone();
        let mut total_advance: f32 = 0.0;

        for element in array {
            match element {
                TextElement::String(text) => {
                    let advance = self.render_text(
                        pixmap,
                        text,
                        base_transform,
                        &current_gs,
                        resources,
                        doc,
                        clip_mask,
                    )?;

                    current_gs.text_matrix.e += advance;
                    total_advance += advance;
                },
                TextElement::Offset(offset) => {
                    // PDF offsets are in 1/1000th of a unit, and positive shifts text to the left
                    let shift = (-offset / 1000.0) * current_gs.font_size;
                    current_gs.text_matrix.e += shift;
                    total_advance += shift;
                },
            }
        }
        Ok(total_advance)
    }

    /// Get font info for a specific font name from resources.
    fn get_font_info(
        &self,
        doc: &mut PdfDocument,
        resources: &Object,
        font_name: &str,
    ) -> Result<crate::fonts::FontInfo> {
        if let Object::Dictionary(res_dict) = resources {
            if let Some(Object::Dictionary(fonts)) = res_dict.get("Font") {
                if let Some(font_ref) = fonts.get(font_name) {
                    let font_obj = doc.resolve_object(font_ref)?;
                    return crate::fonts::FontInfo::from_dict(&font_obj, doc);
                }
            }
        }
        Err(Error::InvalidPdf(format!("Font {} not found", font_name)))
    }

    /// Find and load font data from system.
    fn load_font_data(&self, pdf_font_name: &str) -> Option<(Vec<u8>, u32)> {
        let mut families = Vec::new();

        // 1. Primary choice based on PDF font name
        if pdf_font_name.contains("Times") {
            families.push(fontdb::Family::Serif);
            families.push(fontdb::Family::Name("Times New Roman"));
        } else if pdf_font_name.contains("Courier") {
            families.push(fontdb::Family::Monospace);
            families.push(fontdb::Family::Name("Courier New"));
        } else if pdf_font_name.contains("Helvetica") || pdf_font_name.contains("Arial") {
            families.push(fontdb::Family::SansSerif);
            families.push(fontdb::Family::Name("Arial"));
            families.push(fontdb::Family::Name("Helvetica"));
        } else if pdf_font_name.contains("Noto")
            || pdf_font_name.contains("CJK")
            || pdf_font_name.contains("Adobe")
            || pdf_font_name.contains("STSong")
            || pdf_font_name.contains("SimSun")
            || pdf_font_name.contains("MingLiU")
            || pdf_font_name.contains("MS-Mincho")
            || pdf_font_name.is_empty()
            || pdf_font_name == "F1"
        {
            // High-probability CJK or unknown font — try Noto CJK first
            families.push(fontdb::Family::Name("Noto Sans CJK SC"));
            families.push(fontdb::Family::Name("Noto Serif CJK SC"));
            families.push(fontdb::Family::Name("Noto Sans CJK JP"));
            families.push(fontdb::Family::Name("Noto Serif CJK JP"));
            families.push(fontdb::Family::Name("WenQuanYi Micro Hei"));
            families.push(fontdb::Family::Name("Droid Sans Fallback"));
        }

        // 2. Generic fallbacks
        families.push(fontdb::Family::SansSerif);
        families.push(fontdb::Family::Serif);
        families.push(fontdb::Family::Name("Noto Sans CJK SC"));

        let query = fontdb::Query {
            families: &families,
            weight: fontdb::Weight::NORMAL,
            stretch: fontdb::Stretch::Normal,
            style: fontdb::Style::Normal,
        };

        if let Some(id) = self.font_db().query(&query) {
            let mut data = None;
            self.font_db().with_face_data(id, |face_data, index| {
                log::debug!(
                    "Matched system font for {}: index={}, size={} bytes",
                    pdf_font_name,
                    index,
                    face_data.len()
                );
                data = Some((face_data.to_vec(), index));
            });
            data
        } else {
            log::debug!("No system font match for query: {:?}", query);
            None
        }
    }

    /// Access the font database.
    fn font_db(&self) -> &fontdb::Database {
        &self.fontdb
    }

    /// Render Unicode text using shaped glyphs.
    /// Returns the total horizontal advance in PDF points.
    fn render_unicode_text(
        &self,
        pixmap: &mut Pixmap,
        text: &str,
        font_data: &[u8],
        index: u32,
        paint: &Paint,
        base_transform: Transform,
        gs: &GraphicsState,
        clip_mask: Option<&tiny_skia::Mask>,
    ) -> Result<f32> {
        let font_size = gs.font_size;
        let text_matrix = &gs.text_matrix;

        // 1. Create rustybuzz face and buffer
        let rb_face = rustybuzz::Face::from_slice(font_data, index)
            .ok_or_else(|| Error::InvalidPdf("Failed to create rustybuzz face".to_string()))?;
        let mut buffer = rustybuzz::UnicodeBuffer::new();
        buffer.push_str(text);

        // 2. Shape the text
        let glyphs = rustybuzz::shape(&rb_face, &[], buffer);
        let info = glyphs.glyph_infos();
        let pos = glyphs.glyph_positions();

        // 3. Load ttf-parser face for outlines
        let ttf_face = ttf_parser::Face::parse(font_data, index)
            .map_err(|e| Error::InvalidPdf(format!("Failed to parse font: {}", e)))?;

        let units_per_em = ttf_face.units_per_em() as f32;
        let scale = font_size / units_per_em;

        // 4. Transform setup
        let text_transform = Transform::from_row(
            gs.text_matrix.a,
            gs.text_matrix.b,
            gs.text_matrix.c,
            gs.text_matrix.d,
            0.0,
            0.0,
        );
        let combined_base = base_transform.pre_concat(text_transform);

        let mut x_offset = text_matrix.e;
        let y_offset = text_matrix.f;

        let mut total_advance: f32 = 0.0;

        // 5. Iterate through shaped glyphs
        for i in 0..info.len() {
            let glyph_id = info[i].glyph_id;
            let cluster = info[i].cluster;
            let x_advance = pos[i].x_advance as f32 * scale;

            // Try to get glyph from primary font
            let mut pb = PathBuilder::new();
            let mut builder = SkiaOutlineBuilder(&mut pb);
            let has_primary_outline = ttf_face
                .outline_glyph(ttf_parser::GlyphId(glyph_id as u16), &mut builder)
                .is_some();

            if has_primary_outline {
                if let Some(path) = pb.finish() {
                    let glyph_transform = combined_base
                        .post_translate(x_offset, y_offset)
                        .pre_scale(scale, scale);
                    pixmap.fill_path(
                        &path,
                        paint,
                        tiny_skia::FillRule::Winding,
                        glyph_transform,
                        clip_mask,
                    );
                }
            } else {
                // FALLBACK PATH: If primary font fails, use the cluster index to find the original character
                let char_at_pos = text.chars().nth(cluster as usize).unwrap_or(' ');

                // Skip empty glyphs for spaces
                if char_at_pos.is_whitespace() {
                    x_offset += x_advance + gs.char_space;
                    total_advance += x_advance + gs.char_space;
                    continue;
                }

                if let Some((cjk_data, cjk_index)) = self.load_cjk_fallback() {
                    if let Ok(cjk_face) = ttf_parser::Face::parse(&cjk_data, cjk_index) {
                        if let Some(cjk_glyph_id) = cjk_face.glyph_index(char_at_pos) {
                            let mut cjk_pb = PathBuilder::new();
                            let mut cjk_builder = SkiaOutlineBuilder(&mut cjk_pb);
                            if cjk_face
                                .outline_glyph(cjk_glyph_id, &mut cjk_builder)
                                .is_some()
                            {
                                if let Some(cjk_path) = cjk_pb.finish() {
                                    let cjk_scale = font_size / cjk_face.units_per_em() as f32;
                                    let cjk_transform = combined_base
                                        .post_translate(x_offset, y_offset)
                                        .pre_scale(cjk_scale, cjk_scale);
                                    pixmap.fill_path(
                                        &cjk_path,
                                        paint,
                                        tiny_skia::FillRule::Winding,
                                        cjk_transform,
                                        clip_mask,
                                    );
                                }
                            }
                        }
                    }
                }
            }

            x_offset += x_advance + gs.char_space;
            total_advance += x_advance + gs.char_space;
        }

        Ok(total_advance)
    }

    /// Load a dedicated CJK fallback font.
    fn load_cjk_fallback(&self) -> Option<(Vec<u8>, u32)> {
        let query = fontdb::Query {
            families: &[
                fontdb::Family::Name("Noto Sans CJK SC"),
                fontdb::Family::Name("Noto Serif CJK SC"),
                fontdb::Family::Name("WenQuanYi Micro Hei"),
                fontdb::Family::Name("Droid Sans Fallback"),
            ],
            weight: fontdb::Weight::NORMAL,
            stretch: fontdb::Stretch::Normal,
            style: fontdb::Style::Normal,
        };

        if let Some(id) = self.font_db().query(&query) {
            let mut data = None;
            self.font_db().with_face_data(id, |face_data, index| {
                data = Some((face_data.to_vec(), index));
            });
            data
        } else {
            None
        }
    }

    /// Fallback simple rendering if no font found.
    /// Returns the total horizontal advance in PDF points.
    fn render_text_fallback(
        &self,
        pixmap: &mut Pixmap,
        text: &str,
        paint: &Paint,
        base_transform: Transform,
        gs: &GraphicsState,
        clip_mask: Option<&tiny_skia::Mask>,
    ) -> Result<f32> {
        // Just draw rectangles for now as very last resort
        let font_size = gs.font_size;
        let char_width = font_size * 0.6;
        let mut current_x = gs.text_matrix.e;
        let y = gs.text_matrix.f;
        let mut total_advance: f32 = 0.0;

        let text_transform = Transform::from_row(
            gs.text_matrix.a,
            gs.text_matrix.b,
            gs.text_matrix.c,
            gs.text_matrix.d,
            0.0,
            0.0,
        );
        let transform = base_transform.pre_concat(text_transform);

        for c in text.chars() {
            if c.is_whitespace() {
                current_x += char_width;
                total_advance += char_width + gs.char_space;
                continue;
            }

            let mut pb = PathBuilder::new();
            if let Some(rect) =
                tiny_skia::Rect::from_xywh(current_x, y, char_width * 0.8, font_size * 0.8)
            {
                pb.push_rect(rect);
                if let Some(path) = pb.finish() {
                    pixmap.fill_path(
                        &path,
                        paint,
                        tiny_skia::FillRule::Winding,
                        transform,
                        clip_mask,
                    );
                }
            }
            current_x += char_width + gs.char_space;
            total_advance += char_width + gs.char_space;
        }

        Ok(total_advance)
    }
}

impl Default for TextRasterizer {
    fn default() -> Self {
        Self::new()
    }
}
