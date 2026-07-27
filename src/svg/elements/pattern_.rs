//! Pattern element: `<pattern>`.

use super::macros::{define_svg_element, svg_factory};

define_svg_element!(SvgPattern, "pattern",
    x(r#"X coordinate of the pattern's reference rectangle."#),
    y(r#"Y coordinate of the pattern's reference rectangle."#),
    width(r#"Width of the pattern tile (default `0`)."#),
    height(r#"Height of the pattern tile (default `0`)."#),
    pattern_units(r#"Units for `x`/`y`/`width`/`height` of the pattern itself.

One of `userSpaceOnUse` (default) or `objectBoundingBox`."#),
    pattern_content_units(r#"Units for the pattern's contents.

One of `userSpaceOnUse` (default) or `objectBoundingBox`."#),
    pattern_transform(r#"Transform applied to the pattern."#),
    href(r#"Reference to another pattern from which to inherit attributes."#),
    preserve_aspect_ratio(r#"Aspect-ratio handling for the pattern."#),
    view_box(r#"Internal viewBox of the pattern."#));

// Create a new [`SvgPattern`] element (`<pattern>`).
svg_factory!(pattern, SvgPattern);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderable::Renderable;

    #[test]
    fn pattern_attrs() {
        pattern().width("20").render();
    }
}
