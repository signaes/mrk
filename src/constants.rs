//! HTML void elements — elements that don't have a closing tag.
//!
//! Used only by the renderer when the `html` feature is active.

#[cfg(feature = "html")]
pub(crate) const VOID_ELEMENTS: &[&str] = &[
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link",
    "meta", "param", "source", "track", "wbr",
];
