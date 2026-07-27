//! Masking elements: `<mask>`, `<clipPath>`.

use super::macros::{define_svg_element, svg_factory};

define_svg_element!(SvgMask, "mask",
    x(r#"X coordinate of the mask region (default `-10%`)."#),
    y(r#"Y coordinate of the mask region (default `-10%`)."#),
    width(r#"Width of the mask region (default `120%`)."#),
    height(r#"Height of the mask region (default `120%`)."#),
    mask_units(r#"Units for `x`/`y`/`width`/`height` of the mask region.

One of `userSpaceOnUse` (default) or `objectBoundingBox`."#),
    mask_content_units(r#"Units for the contents of the mask.

One of `userSpaceOnUse` (default) or `objectBoundingBox`."#));

define_svg_element!(SvgClipPath, "clipPath",
    clip_path_units(r#"Coordinate system for the clip path's contents.

One of `userSpaceOnUse` (default) or `objectBoundingBox`."#));

// Create factories.
svg_factory!(mask, SvgMask);
svg_factory!(clip_path, SvgClipPath);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderable::Renderable;

    #[test]
    fn mask_no_attrs() {
        assert_eq!(mask().render(), "<mask></mask>");
    }

    #[test]
    fn clip_path_attrs() {
        assert_eq!(
            clip_path().clip_path_units("userSpaceOnUse").render(),
            r#"<clipPath clipPathUnits="userSpaceOnUse"></clipPath>"#
        );
    }
}
