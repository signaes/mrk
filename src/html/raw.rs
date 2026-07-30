//! Namespace for constructing [`Node::Raw`] values.
//!
//! Methods on `Raw` produce raw-HTML [`Node`]s that are rendered as-is
//! without escaping. Use only for trusted input (e.g., pre-built HTML
//! from a markdown library). For untrusted input, use [`text`](crate::text).

use crate::node::Node;
use std::borrow::Cow;

/// Namespace for raw-HTML node constructors.
///
/// `Raw` has no fields; it exists purely to group related constructors
/// under a shared name so that call sites read clearly.
///
/// # Example
///
/// ```
/// use mrk::Renderable;
/// use mrk::html::{div, Raw};
///
/// let html = div()
///     .set_children(mrk::nodes![
///         "safe content",
///         Raw::str("<b>bold</b>"),
///     ])
///     .render();
/// ```
pub struct Raw;

impl Raw {
    /// A raw HTML string with `'static` lifetime.
    ///
    /// Use for string literals or any pre-built HTML with `'static` lifetime.
    /// The content is rendered as-is, without escaping.
    ///
    /// # Example
    ///
    /// ```
    /// use mrk::Renderable;
    /// use mrk::html::Raw;
    ///
    /// let n = Raw::str("<b>bold</b>");
    /// assert_eq!(n.render(), "<b>bold</b>");
    /// ```
    pub fn str(s: &'static str) -> Node {
        Node::Raw(Cow::Borrowed(s))
    }

    /// An owned raw HTML string (for runtime-constructed content).
    ///
    /// Use when the raw HTML is built dynamically (e.g., from a markdown
    /// library at runtime). The content is rendered as-is, without escaping.
    ///
    /// # Example
    ///
    /// ```
    /// use mrk::Renderable;
    /// use mrk::html::Raw;
    ///
    /// let n = Raw::string("<i>italic</i>".to_string());
    /// assert_eq!(n.render(), "<i>italic</i>");
    /// ```
    pub fn string(s: String) -> Node {
        Node::Raw(Cow::Owned(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html::div;
    use crate::renderable::Renderable;

    #[test]
    fn raw_str_renders_without_escape() {
        let n = Raw::str("<b>bold</b>");
        assert_eq!(n.render(), "<b>bold</b>");
    }

    #[test]
    fn raw_string_renders_without_escape() {
        let n = Raw::string("<i>italic</i>".to_string());
        assert_eq!(n.render(), "<i>italic</i>");
    }

    #[test]
    fn raw_str_does_not_escape_special_chars() {
        assert_eq!(Raw::str("a & b < c").render(), "a & b < c");
    }

    #[test]
    fn raw_str_in_tree() {
        let html = div()
            .set_children(vec!["safe".into(), Raw::str("<b>bold</b>")])
            .render();
        assert_eq!(html, "<div>safe<b>bold</b></div>");
    }

    #[test]
    fn raw_string_mixes_with_text() {
        let html = div()
            .set_children(vec![
                Raw::str("<b>"),
                Raw::string("dynamic".to_string()),
                Raw::str("</b>"),
            ])
            .render();
        assert_eq!(html, "<div><b>dynamic</b></div>");
    }
}
