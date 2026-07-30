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
///
/// This enum has three variants (`Text`, `Element`, `Raw`).
/// Unevaluated template expressions live in the companion
/// `mrk-components` crate, which builds on this type.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(unused)]
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

impl From<Cow<'static, str>> for Node {
    fn from(s: Cow<'static, str>) -> Node {
        Node::Text(s)
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
    fn from_cow_borrowed_text() {
        let borrowed: Cow<'static, str> = Cow::Borrowed("static");
        let n: Node = borrowed.into();
        let debug = format!("{:?}", n);
        assert!(debug.contains("Text"));
        assert!(debug.contains("static"));
    }

    #[test]
    fn from_cow_owned_text() {
        let owned: Cow<'static, str> = Cow::Owned(String::from("owned-cow"));
        let n: Node = owned.into();
        let debug = format!("{:?}", n);
        assert!(debug.contains("Text"));
        assert!(debug.contains("owned-cow"));
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

    /// `Display`-based tests for `Node`: `Display` produces the
    /// derived `Debug` repr.
    mod display_tests {
        use super::*;

        #[test]
        fn to_string_text() {
            let n: Node = "hello".into();
            let s = format!("{}", n);
            assert!(s.contains("Text"));
            assert!(s.contains("hello"));
        }

        #[test]
        fn to_string_raw() {
            let n = Node::Raw("<br/>".into());
            let s = format!("{}", n);
            assert!(s.contains("Raw"));
            assert!(s.contains("<br/>"));
        }

        #[test]
        fn to_string_element() {
            let n: Node = el("div").into();
            let s = format!("{}", n);
            assert!(s.contains("Element"));
            assert!(s.contains("div"));
        }
    }
}
