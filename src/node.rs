//! The three runtime flavors of node content: escaped text, nested
//! element, or unescaped raw HTML.
//!
//! A [`Node`] is what lives inside an
//! [`Element::children`](crate::Element::children). The `nodes!` macro
//! accepts strings (→ `Node::Text`), nested [`Element`]s (→
//! `Node::Element`), [`Raw`](crate::html::Raw) HTML (→ `Node::Raw`),
//! and existing [`Node`]s (pass-through).
//!
//! With the `html` feature, `Raw::str("<b>bold</b>")` produces a
//! `Node::Raw` for trusted prebuilt HTML.

use crate::element::Element;
use std::borrow::Cow;

/// A single child of an element.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    /// Escaped text content. Special HTML chars (`<`, `>`, `&`, `"`) are
    /// replaced with their entity equivalents when rendered.
    Text(Cow<'static, str>),
    /// A child element.
    Element(Element),
    /// Raw, unescaped HTML content — rendered as-is. Use only for trusted
    /// input (e.g., pre-built HTML from a markdown library). Construct via
    /// `Raw::str(...)` or `Raw::string(...)` (in the `html` module).
    Raw(Cow<'static, str>),
}

impl From<&'static str> for Node {
    fn from(s: &'static str) -> Node {
        Node::Text(Cow::Borrowed(s))
    }
}

impl From<String> for Node {
    fn from(s: String) -> Node {
        Node::Text(Cow::Owned(s))
    }
}

impl From<Element> for Node {
    fn from(e: Element) -> Self {
        Node::Element(e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::el;

    #[test]
    fn from_str_creates_borrowed_text() {
        let n: Node = "hello".into();
        let debug = format!("{:?}", n);
        assert!(debug.contains("Text"));
        assert!(debug.contains("hello"));
    }

    #[test]
    fn from_string_creates_owned_text() {
        let owned = String::from("dynamic");
        let n: Node = owned.into();
        let debug = format!("{:?}", n);
        assert!(debug.contains("Text"));
        assert!(debug.contains("dynamic"));
    }

    #[test]
    fn from_element() {
        let e = el("div");
        let n: Node = e.into();
        let debug = format!("{:?}", n);
        assert!(debug.contains("Element"));
    }

    #[test]
    fn debug_format() {
        let n: Node = "hello".into();
        let _ = format!("{:?}", n);
    }

    /// `Display`-based tests for `Node`: the `Display` impl lives in
    /// `crate::ir::display`, gated on the `ir` Cargo feature.
    #[cfg(feature = "ir")]
    mod display_tests {
        use super::*;

        #[test]
        fn to_string_text() {
            let n: Node = "hello".into();
            assert_eq!(format!("{}", n), "hello");
        }

        #[test]
        fn to_string_raw() {
            let n = Node::Raw("<br/>".into());
            assert_eq!(format!("{}", n), "<br/>");
        }

        #[test]
        fn to_string_element() {
            let n: Node = el("div").into();
            let s = format!("{}", n);
            assert!(s.starts_with("mrk1\n"));
        }
    }
}
