//! Text elements: `<text>`, `<tspan>`, `<textPath>`.

use super::macros::{define_svg_element, svg_factory};

define_svg_element!(SvgText, "text",
    x(r#"X coordinate at which to place the text (default `0`)."#),
    y(r#"Y coordinate at which to place the text (default `0`)."#),
    dx(r#"Horizontal offset from the previous text element (default `0`)."#),
    dy(r#"Vertical offset from the previous text element (default `0`)."#),
    rotate(r#"Comma-separated rotation angles applied to each glyph (default `0`)."#),
    text_length(r#"Author-specified total length of the text.

A `<length>` the text must occupy; the renderer scales letter-spacing to fit."#),
    length_adjust(r#"How `text_length` is enforced.

One of `spacing` (default; only adjusts spacing) or `spacingAndGlyphs`
(adjusts both glyph widths and spacing)."#));

define_svg_element!(SvgTspan, "tspan",
    x(r#"X coordinate at which to place the `<tspan>` text (absolute)."#),
    y(r#"Y coordinate at which to place the `<tspan>` text (absolute)."#),
    dx(r#"Horizontal offset from the previous element."#),
    dy(r#"Vertical offset from the previous element."#),
    rotate(r#"Comma-separated rotation angles applied to each glyph."#),
    text_length(r#"Author-specified total length of the `<tspan>` text."#),
    length_adjust(r#"How `text_length` is enforced.

One of `spacing` or `spacingAndGlyphs`."#));

define_svg_element!(SvgTextPath, "textPath",
    href(r#"URL of the reference path that the text follows.

A `#id` reference to a `<path>` or other element whose geometry is used
as the baseline for the text."#),
    start_offset(r#"Distance into the path at which the text begins.

A `<length>` or `<percentage>`; the percentage form is relative to the path's
total length."#),
    text_length(r#"Author-specified total length of the rendered text along the path."#),
    length_adjust(r#"How `text_length` is enforced."#),
    side(r#"Side of the path on which to render the text.

One of `left` (default) or `right`."#));

svg_factory!(
    /// Create a new [`SvgText`] element (`<text>`).
    text, SvgText
);
svg_factory!(
    /// Create a new [`SvgTspan`] element (`<tspan>`).
    tspan, SvgTspan
);
svg_factory!(
    /// Create a new [`SvgTextPath`] element (`<textPath>`).
    text_path, SvgTextPath
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderable::Renderable;

    #[test]
    fn text_attrs() {
        text().x("10").render();
    }

    #[test]
    fn tspan_attrs() {
        tspan().dx("2").render();
    }

    #[test]
    fn text_path_attrs() {
        text_path().start_offset("10%").render();
    }
}
