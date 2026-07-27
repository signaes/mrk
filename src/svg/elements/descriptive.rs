//! Descriptive elements: `<title>`, `<desc>`, `<metadata>`, `<foreignObject>`.

use super::macros::{define_svg_element, svg_factory};

define_svg_element!(SvgTitle, "title");

define_svg_element!(SvgDesc, "desc");

define_svg_element!(SvgMetadata, "metadata");

define_svg_element!(SvgForeignObject, "foreignObject",
    x(r#"X coordinate of the foreign object's location."#),
    y(r#"Y coordinate of the foreign object's location."#),
    width(r#"Width of the foreign object (must be a `<length>`). Required."#),
    height(r#"Height of the foreign object (must be a `<length>`). Required."#));

svg_factory!(
    /// Create a new [`SvgTitle`] element (`<title>`).
    title, SvgTitle
);
svg_factory!(
    /// Create a new [`SvgDesc`] element (`<desc>`).
    desc, SvgDesc
);
svg_factory!(
    /// Create a new [`SvgMetadata`] element (`<metadata>`).
    metadata, SvgMetadata
);
svg_factory!(
    /// Create a new [`SvgForeignObject`] element (`<foreignObject>`).
    foreign_object, SvgForeignObject
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderable::Renderable;

    #[test]
    fn descriptive_no_attrs() {
        assert_eq!(title().render(), "<title></title>");
        assert_eq!(desc().render(), "<desc></desc>");
        assert_eq!(metadata().render(), "<metadata></metadata>");
    }

    #[test]
    fn foreign_object_attrs() {
        // Verify `width` and `height` are valid method names (no compiler clash).
        foreign_object().x("0").render();
    }
}
