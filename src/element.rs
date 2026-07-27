//! The core in-memory element: tag name + attributes + children.
//!
//! Use [`el`] (or [`Element::new`]) to construct, then chain
//! `.attrs(...)` and `.children(...)`. For pure-HTML trees, the
//! `html` feature adds 114 tag-specific factories (e.g. `div()`, `p()`).
//!
//! ```
//! use mrk::*;
//!
//! let link = el("a")
//!     .attrs(vec![attr("href").value("/home")])
//!     .children(nodes!["Home"]);
//! ```

use crate::attributes::Attribute;
use crate::node::Node;
use std::borrow::Cow;

/// A markup element: name, attributes, children.
///
/// Plain struct with three fields so it composes cleanly with
/// [`Node::Element`](crate::Node::Element). The conventional
/// builders live on [`Element`] itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Element {
    /// Tag name (e.g. `"div"`, `"custom-tag"`). `Cow::Borrowed` for
    /// literals, `Cow::Owned` for runtime strings.
    pub name: Cow<'static, str>,
    /// Attributes in source order.
    pub attributes: Vec<Attribute>,
    /// Child nodes (text, raw HTML, or nested elements).
    pub children: Vec<Node>,
}

impl Element {
    /// Construct an empty element with `name` and no attributes or
    /// children.
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Element {
            name: name.into(),
            attributes: vec![],
            children: vec![],
        }
    }

    /// Append attributes to the element.
    ///
    /// # Example
    ///
    /// ```
    /// use mrk::*;
    ///
    /// let e = el("a").attrs(vec![attr("href").value("/")]);
    /// assert_eq!(e.attributes.len(), 1);
    /// ```
    pub fn attrs(mut self, attributes: Vec<Attribute>) -> Self {
        self.attributes.extend(attributes);
        self
    }

    /// Append a single attribute to the element.
    pub fn push_attr(mut self, attr: Attribute) -> Self {
        self.attributes.push(attr);
        self
    }

    /// Set the element's children, replacing any previously set.
    ///
    /// Use the `nodes!` macro to build the children list with mixed
    /// strings and elements:
    ///
    /// # Example
    ///
    /// ```
    /// use mrk::*;
    ///
    /// let e = el("p").children(nodes!["hi"]);
    /// assert_eq!(e.children.len(), 1);
    /// ```
    pub fn children(mut self, children: Vec<Node>) -> Self {
        self.children = children;
        self
    }
}

/// Construct an empty element with `name`. Convenience wrapper for
/// [`Element::new`].
pub fn el(name: impl Into<Cow<'static, str>>) -> Element {
    Element::new(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attributes::attr;
    use crate::nodes;

    #[test]
    fn el_creates_empty_element() {
        let e = el("div");
        assert_eq!(e.name, "div");
        assert!(e.attributes.is_empty());
        assert!(e.children.is_empty());
    }

    #[test]
    fn builder_chains() {
        let e = el("a")
            .attrs(vec![attr("href").value("/")])
            .children(nodes!["Home"]);

        assert_eq!(e.name, "a");
        assert_eq!(e.attributes.len(), 1);
        assert_eq!(e.children.len(), 1);
    }

    #[test]
    fn struct_literal_construction() {
        let e = Element {
            name: Cow::Borrowed("custom"),
            attributes: vec![],
            children: vec![],
        };
        assert_eq!(e.name, "custom");
    }

    #[test]
    fn direct_field_mutation() {
        let mut e = el("div");
        e.attributes.push(attr("class").value("container"));
        e.children.push("Hello".into());
        assert_eq!(e.attributes.len(), 1);
        assert_eq!(e.children.len(), 1);
    }

    #[test]
    fn debug_format() {
        let e = el("div");
        let _ = format!("{:?}", e);
    }

    #[test]
    fn push_attr_appends_single() {
        let e = el("div")
            .push_attr(attr("class").value("a"))
            .push_attr(attr("id").value("b"));
        assert_eq!(e.attributes.len(), 2);
        assert_eq!(e.attributes[0].key, "class");
        assert_eq!(e.attributes[1].key, "id");
    }

    #[test]
    fn attrs_appends_multiple() {
        let e = el("div")
            .attrs(vec![attr("class").value("a")])
            .attrs(vec![attr("id").value("b")]);
        assert_eq!(e.attributes.len(), 2);
    }

    #[test]
    fn el_accepts_owned_string() {
        let dynamic = String::from("dyn");
        let e = el(dynamic);
        assert_eq!(e.name, "dyn");
    }
}
