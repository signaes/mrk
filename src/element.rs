use crate::attributes::Attribute;
use crate::node::Node;

#[derive(Debug)]
pub struct Element {
    pub name: &'static str,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Node>,
}

impl Element {
    pub fn new(name: &'static str) -> Self {
        Element {
            name,
            attributes: vec![],
            children: vec![],
        }
    }

    /// Sets the element's attributes, replacing any previously set.
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
        self.attributes = attributes;
        self
    }

    /// Sets the element's children, replacing any previously set.
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

pub fn el(name: &'static str) -> Element {
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
            name: "custom",
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
}
