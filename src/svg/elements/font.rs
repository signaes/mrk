//! SVG font elements. Used only when authoring complete SVG fonts;
//! most usage of text in SVG reverts to the `<text>` element with
//! system fonts.

use super::macros::{define_svg_element, svg_factory};

define_svg_element!(SvgFont, "font",
    horiz_adv_x(r#"Default horizontal advance for glyphs without an explicit value."#),
    horiz_origin_x(r#"X-coordinate of the horizontal origin in user units."#),
    horiz_origin_y(r#"Y-coordinate of the horizontal origin in user units."#),
    vert_adv_y(r#"Default vertical advance for vertical text."#),
    vert_origin_x(r#"X-coordinate of the vertical origin in user units."#),
    vert_origin_y(r#"Y-coordinate of the vertical origin in user units."#));

define_svg_element!(SvgFontFace, "font-face",
    font_family(r#"Font family name (e.g. `Verdana` or custom identifier)."#),
    font_size(r#"Default glyph size (a `<length>`)."#),
    font_style(r#"Style: `normal`, `italic`, `oblique`."#),
    font_weight(r#"Weight: `normal`, `bold`, or a numeric value 100..1000."#),
    units_per_em(r#"Number of units per em-square (commonly `1000` or `2048`)."#),
    underline_position(r#"Y-offset of the underline line below the baseline."#),
    underline_thickness(r#"Thickness of the underline stroke."#),
    overline_position(r#"Y-offset of the overline line above the baseline."#),
    overline_thickness(r#"Thickness of the overline stroke."#),
    strike_through_position(r#"Y-offset of the strike-through line."#),
    strike_through_thickness(r#"Thickness of the strike-through stroke."#),
    ascent(r#"Distance from the baseline to the top of the em-square."#),
    descent(r#"Distance from the baseline to the bottom of the em-square."#));

define_svg_element!(SvgFontFaceSrc, "font-face-src");

define_svg_element!(SvgFontFaceUri, "font-face-uri",
    href(r#"URL of the referenced font."#));

define_svg_element!(SvgFontFaceFormat, "font-face-format");

define_svg_element!(SvgFontFaceName, "font-face-name",
    name(r#"Name referencing a local font installed on the user agent."#));

define_svg_element!(SvgMissingGlyph, "missing-glyph",
    d(r#"Path data for the missing-glyph shape."#),
    horiz_adv_x(r#"Horizontal advance for the missing glyph."#),
    vert_adv_y(r#"Vertical advance for the missing glyph."#));

define_svg_element!(SvgGlyph, "glyph",
    unicode(r#"Unicode code point(s) this glyph represents."#),
    glyph_name(r#"Name of the glyph for `<use>` references."#),
    d(r#"Path data for the glyph's outline."#),
    horiz_adv_x(r#"Horizontal advance for this glyph (overrides the font default)."#),
    vert_adv_y(r#"Vertical advance for this glyph."#),
    arabic_form(r#"Arabic form: `initial`, `medial`, `terminal`, `isolated`."#),
    orientation(r#"Orientation: `h` (horizontal) or `v` (vertical)."#),
    lang(r#"Language tag for the glyph (used for hinting sources)."#));

define_svg_element!(SvgHKern, "hkern",
    k1(r#"First glyph (`<glyph-name>` or `Unicode-range`) for kerning pair."#),
    k2(r#"Second glyph for kerning pair."#),
    k3(r#"Adjustment value added to k2's advance (kerning amount)."#),
    k4(r#"Optional second adjustment for orthogonal axis."#),
    u1(r#"Starting Unicode range for the first glyph."#),
    u2(r#"Starting Unicode range for the second glyph."#),
    g1(r#"Starting glyph range for the first glyph."#),
    g2(r#"Starting glyph range for the second glyph."#));

define_svg_element!(SvgVKern, "vkern",
    k1(r#"First glyph for vertical kerning pair."#),
    k2(r#"Second glyph for vertical kerning pair."#),
    k3(r#"Vertical kerning amount."#),
    k4(r#"Optional horizontal adjustment."#),
    u1(r#"Starting Unicode range for the first glyph."#),
    u2(r#"Starting Unicode range for the second glyph."#),
    g1(r#"Starting glyph range for the first glyph."#),
    g2(r#"Starting glyph range for the second glyph."#));

// Create factories.
svg_factory!(font, SvgFont);
svg_factory!(font_face, SvgFontFace);
svg_factory!(font_face_src, SvgFontFaceSrc);
svg_factory!(font_face_uri, SvgFontFaceUri);
svg_factory!(font_face_format, SvgFontFaceFormat);
svg_factory!(font_face_name, SvgFontFaceName);
svg_factory!(missing_glyph, SvgMissingGlyph);
svg_factory!(glyph, SvgGlyph);
svg_factory!(hkern, SvgHKern);
svg_factory!(vkern, SvgVKern);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderable::Renderable;

    #[test]
    fn font_face_no_attrs() {
        assert_eq!(font_face().render(), "<font-face></font-face>");
    }

    #[test]
    fn glyph_attrs_smoke() {
        glyph().unicode("A").horiz_adv_x("500").render();
    }
}
