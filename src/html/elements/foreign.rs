//! Foreign content elements (`<math>`, `<svg>`).

use super::macros::{define_html_element, factory};

define_html_element!(HtmlMath, "math");
define_html_element!(HtmlSvg, "svg",
    width("Width of the SVG viewport."),
    height("Height of the SVG viewport."),
    view_box("SVG viewBox attribute for coordinate system."),
    preserve_aspect_ratio("How to scale within the viewport."),
    xmlns("XML namespace for SVG."),
    version("SVG version number."));

// Create a new [`HtmlMath`] element (`<math>`).
factory!(math, HtmlMath);
// Create a new [`HtmlSvg`] element (`<svg>`).
factory!(svg, HtmlSvg);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn math_element() {
        assert_eq!(math().render(), "<math></math>");
    }

    #[test]
    fn svg_element() {
        assert_eq!(svg().render(), "<svg></svg>");
    }

    #[test]
    fn svg_attrs() {
        assert_eq!(svg().width("100").render(), r#"<svg width="100"></svg>"#);
        assert_eq!(svg().height("200").render(), r#"<svg height="200"></svg>"#);
        assert_eq!(svg().view_box("0 0 100 100").render(), r#"<svg view-box="0 0 100 100"></svg>"#);
        assert_eq!(svg().preserve_aspect_ratio("xMidYMid meet").render(), r#"<svg preserve-aspect-ratio="xMidYMid meet"></svg>"#);
        assert_eq!(svg().xmlns("http://www.w3.org/2000/svg").render(), r#"<svg xmlns="http://www.w3.org/2000/svg"></svg>"#);
        assert_eq!(svg().version("1.1").render(), r#"<svg version="1.1"></svg>"#);
    }
}
