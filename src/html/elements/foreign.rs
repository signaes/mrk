//! Foreign content elements (`<math>`, `<svg>`).

use super::macros::{define_html_element, factory};

define_html_element!(HtmlMath, "math", all);
define_html_element!(HtmlSvg, "svg", no_aria,
    width(r#"Width of the rendered region of the SVG.

A length, either a number (interpreted as user-space units) or a CSS length string with a unit (e.g. `100`, `100px`, `50%`)."#),
    height(r#"Height of the rendered region of the SVG.

A length, either a number (interpreted as user-space units) or a CSS length string with a unit (e.g. `100`, `100px`, `50%`)."#),
    view_box(r#"Position and size of the SVG viewport in user space, as four space-separated numbers: `min-x min-y width height` (e.g. `0 0 100 100`).

Sets the coordinate system the SVG content is drawn against. Emitted as the spec-correct camelCase `viewBox` (the macro's `attr_name` table provides an explicit override for this SVG attribute)."#),
    preserve_aspect_ratio(r#"How to scale the SVG content if the aspect ratio of `viewBox` differs from the aspect ratio of the rendered region.

Emitted as the spec-correct camelCase `preserveAspectRatio` (the macro's `attr_name` table provides an explicit override for this SVG attribute).

A space-separated combination of two values:
- An alignment: `none`, `xMinYMin`, `xMidYMin`, `xMaxYMin`, `xMinYMid`, `xMidYMid` (default), `xMaxYMid`, `xMinYMax`, `xMidYMax`, `xMaxYMax`
- A meet-or-slice keyword: `meet` (default; preserve aspect ratio, fit inside viewport) or `slice` (preserve aspect ratio, cover viewport)"#),
    xmlns(r#"XML namespace declaration for SVG.

The standard value is `http://www.w3.org/2000/svg`. Required when the SVG is serialized as standalone XML; not needed when inlined in HTML, which infers the SVG namespace."#),
    version(r#"SVG language version.

Legacy attribute; was required in SVG 1.1 to identify the version of the SVG language in use. SVG 2 dropped the requirement and the attribute is no longer needed. If present, the standard value is `1.1`."#));

factory!(
    /// Create a new [`HtmlMath`] element (`<math>`).
    math, HtmlMath
);
factory!(
    /// Create a new [`HtmlSvg`] element (`<svg>`).
    svg, HtmlSvg
);

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
        assert_eq!(svg().view_box("0 0 100 100").render(), r#"<svg viewBox="0 0 100 100"></svg>"#);
        assert_eq!(svg().preserve_aspect_ratio("xMidYMid meet").render(), r#"<svg preserveAspectRatio="xMidYMid meet"></svg>"#);
        assert_eq!(svg().xmlns("http://www.w3.org/2000/svg").render(), r#"<svg xmlns="http://www.w3.org/2000/svg"></svg>"#);
        assert_eq!(svg().version("1.1").render(), r#"<svg version="1.1"></svg>"#);
    }
}
