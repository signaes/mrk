//! Shape elements: `<circle>`, `<ellipse>`, `<line>`, `<polyline>`,
//! `<polygon>`, `<rect>`, `<path>`.

use super::macros::{define_svg_element, svg_factory};

define_svg_element!(SvgCircle, "circle",
    cx(r#"X coordinate of the circle's center in user coordinates."#),
    cy(r#"Y coordinate of the circle's center in user coordinates."#),
    r(r#"Radius of the circle (a non-negative `<length>`)."#),
    path_length(r#"Author-specified total circumference of the circle."#));

define_svg_element!(SvgEllipse, "ellipse",
    cx(r#"X coordinate of the ellipse's center."#),
    cy(r#"Y coordinate of the ellipse's center."#),
    rx(r#"X-radius of the ellipse (a non-negative `<length>`)."#),
    ry(r#"Y-radius of the ellipse (a non-negative `<length>`)."#),
    path_length(r#"Author-specified circumference of the ellipse."#));

define_svg_element!(SvgLine, "line",
    x1(r#"X coordinate of the line's start point."#),
    y1(r#"Y coordinate of the line's start point."#),
    x2(r#"X coordinate of the line's end point."#),
    y2(r#"Y coordinate of the line's end point."#),
    path_length(r#"Author-specified total length of the line."#));

define_svg_element!(SvgPolyline, "polyline",
    points(r#"Space- or comma-separated list of points making up the polyline."#),
    path_length(r#"Author-specified total length of the polyline."#));

define_svg_element!(SvgPolygon, "polygon",
    points(r#"Space- or comma-separated list of points making up the polygon."#),
    path_length(r#"Author-specified total length of the polygon's perimeter."#));

define_svg_element!(SvgRect, "rect",
    x(r#"X coordinate of the rectangle's left edge."#),
    y(r#"Y coordinate of the rectangle's top edge."#),
    width(r#"Width of the rectangle (a non-negative `<length>`)."#),
    height(r#"Height of the rectangle (a non-negative `<length>`)."#),
    rx(r#"X-radius of the ellipse used to round corners (default: 0)."#),
    ry(r#"Y-radius of the ellipse used to round corners (default: 0)."#),
    path_length(r#"Author-specified perimeter length of the rectangle."#));

define_svg_element!(SvgPath, "path",
    d(r#"Path data defining the shape's outline."#),
    path_length(r#"Author-specified total length of the path."#));

svg_factory!(
    /// Create a new [`SvgCircle`] element (`<circle>`).
    circle, SvgCircle
);
svg_factory!(
    /// Create a new [`SvgEllipse`] element (`<ellipse>`).
    ellipse, SvgEllipse
);
svg_factory!(
    /// Create a new [`SvgLine`] element (`<line>`).
    line, SvgLine
);
svg_factory!(
    /// Create a new [`SvgPolyline`] element (`<polyline>`).
    polyline, SvgPolyline
);
svg_factory!(
    /// Create a new [`SvgPolygon`] element (`<polygon>`).
    polygon, SvgPolygon
);
svg_factory!(
    /// Create a new [`SvgRect`] element (`<rect>`).
    rect, SvgRect
);
svg_factory!(
    /// Create a new [`SvgPath`] element (`<path>`).
    path, SvgPath
);

#[cfg(test)]
mod tests {
    use super::*;

    fn attr_list(pairs: &[(&'static str, &'static str)]) -> Vec<crate::attributes::Attribute> {
        use crate::attributes::attr;
        pairs.iter().map(|(k, v)| attr(k).value(*v)).collect()
    }

    #[test]
    fn circle_attrs() {
        let el = attr_list(&[("cx", "50"), ("cy", "50"), ("r", "40")]);
        assert_eq!(
            circle().attrs(el.clone()).render(),
            r#"<circle cx="50" cy="50" r="40"></circle>"#
        );
    }

    #[test]
    fn ellipse_attrs() {
        let el = attr_list(&[("cx", "50"), ("cy", "50"), ("rx", "40"), ("ry", "20")]);
        assert_eq!(
            ellipse().attrs(el.clone()).render(),
            r#"<ellipse cx="50" cy="50" rx="40" ry="20"></ellipse>"#
        );
    }

    #[test]
    fn line_attrs() {
        let el = attr_list(&[("x1", "0"), ("y1", "0"), ("x2", "100"), ("y2", "100")]);
        assert_eq!(
            line().attrs(el.clone()).render(),
            r#"<line x1="0" y1="0" x2="100" y2="100"></line>"#
        );
    }

    #[test]
    fn polyline_attrs() {
        let el = attr_list(&[("points", "0,0 50,50 100,0")]);
        assert_eq!(
            polyline().attrs(el.clone()).render(),
            r#"<polyline points="0,0 50,50 100,0"></polyline>"#
        );
    }

    #[test]
    fn polygon_attrs() {
        let el = attr_list(&[("points", "0,0 100,0 50,100")]);
        assert_eq!(
            polygon().attrs(el.clone()).render(),
            r#"<polygon points="0,0 100,0 50,100"></polygon>"#
        );
    }

    #[test]
    fn rect_attrs() {
        let el = attr_list(&[("x", "10"), ("y", "10"), ("width", "100"), ("height", "50"), ("rx", "5")]);
        assert_eq!(
            rect().attrs(el.clone()).render(),
            r#"<rect x="10" y="10" width="100" height="50" rx="5"></rect>"#
        );
    }

    #[test]
    fn path_attrs() {
        let el = attr_list(&[("d", "M 0 0 L 100 100")]);
        assert_eq!(
            path().attrs(el.clone()).render(),
            r#"<path d="M 0 0 L 100 100"></path>"#
        );
    }

    #[test]
    fn shapes_render_with_closing_tag_when_no_attrs() {
        assert_eq!(rect().render(), "<rect></rect>");
        assert_eq!(path().render(), "<path></path>");
    }

    #[test]
    fn shapes_render_single_attr() {
        assert_eq!(circle().cx("50").render(), r#"<circle cx="50"></circle>"#);
        assert_eq!(rect().width("100").render(), r#"<rect width="100"></rect>"#);
    }
}
