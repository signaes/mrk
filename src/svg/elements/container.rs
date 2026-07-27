//! Container elements: `<svg>`, `<g>`, `<defs>`, `<symbol>`, `<use>`,
//! `<a>`, `<switch>`, `<marker>`.

use super::macros::{define_svg_element, svg_factory};

define_svg_element!(SvgSvg, "svg",
    x(r#"X coordinate of the upper-left corner of the rendered region.

Used when this SVG is itself nested inside another SVG."#),
    y(r#"Y coordinate of the upper-left corner of the rendered region."#),
    width(r#"Width of the rendered region (a `<length>` or `auto`)."#),
    height(r#"Height of the rendered region (a `<length>` or `auto`)."#),
    view_box(r#"Position and size of the SVG viewport in user space.

Four space-separated numbers: `min-x min-y width height` (e.g. `0 0 100 100`).
The SVG's internal coordinate system is mapped onto this box."#),
    preserve_aspect_ratio(r#"How to scale the SVG content when the aspect
ratio of `viewBox` differs from the aspect ratio of the rendered region.

A pair of alignment + meet/slice tokens (e.g. `xMidYMid meet`)."#),
    xmlns(r#"XML namespace declaration. For standalone SVG: `http://www.w3.org/2000/svg`."#),
    version(r#"SVG language version (legacy; the only meaningful value is `1.1`)."#));

define_svg_element!(SvgG, "g");

define_svg_element!(SvgDefs, "defs");

define_svg_element!(SvgSymbol, "symbol",
    x(r#"X coordinate of the symbol's reference box."#),
    y(r#"Y coordinate of the symbol's reference box."#),
    width(r#"Width of the symbol's reference box."#),
    height(r#"Height of the symbol's reference box."#),
    ref_x(r#"Reference X within the symbol — used by `<use>` to anchor the
`x` attribute of the `<use>` element to a point inside the symbol."#),
    ref_y(r#"Reference Y within the symbol — counterpart to `ref_x`."#),
    ref_width(r#"Override the symbol's intrinsic width when used."#),
    ref_height(r#"Override the symbol's intrinsic height when used."#),
    preserve_aspect_ratio(r#"Aspect-ratio handling for the symbol."#),
    view_box(r#"Internal coordinate system of the symbol."#));

define_svg_element!(SvgUse, "use",
    href(r#"URL fragment or external reference to the symbol/element being reused.

Local references use a `#fragment` selector."#),
    xlink_href(r#"Legacy XLink href for the same purpose as `href`."#),
    x(r#"X offset applied to the cloned element."#),
    y(r#"Y offset applied to the cloned element."#),
    width(r#"Override width of the cloned element."#),
    height(r#"Override height of the cloned element."#));

define_svg_element!(SvgA, "a",
    href(r#"URL the link points to.

Within an SVG document, this can be a fragment identifier (`#id`) to create
internal navigation."#),
    xlink_href(r#"Legacy XLink href for the same purpose as `href`."#),
    target(r#"Name of the browsing context for the linked resource (HTML-style)."#),
    download(r#"Present if the link is a download prompt."#));

define_svg_element!(SvgSwitch, "switch",
    required_features(r#"Whitespace-separated list of feature strings that
must be supported by the user agent for the matching child to render."#),
    required_extensions(r#"Whitespace-separated list of extension names that
must be supported by the user agent."#),
    system_language(r#"BCP 47 language tag; matching child is rendered only
when the user agent's language matches."#));

define_svg_element!(SvgMarker, "marker",
    ref_x(r#"Reference X within the marker — the marker's coordinate space
is anchored at `(ref_x, ref_y)` when placed."#),
    ref_y(r#"Reference Y within the marker."#),
    marker_units(r#"Units used to size the marker.

One of `strokeWidth` (scales with the host path's stroke width) or
`userSpaceOnUse` (uses absolute user units)."#),
    marker_width(r#"Width of the marker's viewport (default `3`)."#),
    marker_height(r#"Height of the marker's viewport (default `3`)."#),
    orient(r#"Orientation of the marker.

Either `auto` (rotate to follow path direction) or `auto-start-reverse`
(same, but flipped on reversed segments), or an angle like `90`."#),
    preserve_aspect_ratio(r#"Aspect-ratio handling for the marker."#),
    view_box(r#"Internal viewBox of the marker."#));

svg_factory!(
    /// Create a new [`SvgSvg`] element (`<svg>`).
    svg, SvgSvg
);
svg_factory!(
    /// Create a new [`SvgG`] element (`<g>`).
    g, SvgG
);
svg_factory!(
    /// Create a new [`SvgDefs`] element (`<defs>`).
    defs, SvgDefs
);
svg_factory!(
    /// Create a new [`SvgSymbol`] element (`<symbol>`).
    symbol, SvgSymbol
);
svg_factory!(
    /// Create a new [`SvgUse`] element (`<use>`).
    ///
    /// Note: `use` is a Rust keyword; the factory is renamed to `use_`.
    use_, SvgUse
);
svg_factory!(
    /// Create a new [`SvgA`] element (`<a>`).
    a, SvgA
);
svg_factory!(
    /// Create a new [`SvgSwitch`] element (`<switch>`).
    switch, SvgSwitch
);
svg_factory!(
    /// Create a new [`SvgMarker`] element (`<marker>`).
    marker, SvgMarker
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderable::Renderable;

    #[test]
    fn svg_root_attrs() {
        // The last setter in a chain wins (replace semantics); so
        // we set multiple attrs through the `attrs()` API.
        use crate::attributes::attr;
        let s = svg().attrs(vec![
            attr("xmlns").value("http://www.w3.org/2000/svg"),
            attr("viewBox").value("0 0 100 100"),
        ]).render();
        assert!(s.contains("xmlns=\"http://www.w3.org/2000/svg\""));
        assert!(s.contains("viewBox=\"0 0 100 100\""));
    }

    #[test]
    fn g_and_defs_have_no_attrs() {
        assert_eq!(g().render(), "<g></g>");
        assert_eq!(defs().render(), "<defs></defs>");
    }

    #[test]
    fn use_attrs() {
        use crate::attributes::attr;
        let a = use_().attrs(vec![
            attr("href").value("#icon"),
            attr("x").value("10"),
            attr("y").value("10"),
        ]).render();
        assert_eq!(a, r##"<use href="#icon" x="10" y="10"></use>"##);
    }

    #[test]
    fn switch_attrs() {
        switch().system_language("en").render();
    }

    #[test]
    fn marker_attrs() {
        use crate::attributes::attr;
        marker().attrs(vec![
            attr("markerWidth").value("8"),
            attr("markerHeight").value("8"),
            attr("refX").value("4"),
            attr("refY").value("4"),
            attr("markerUnits").value("strokeWidth"),
            attr("orient").value("auto"),
        ]).render();
    }

    #[test]
    fn symbol_attrs() {
        symbol().view_box("0 0 24 24").render();
    }
}
