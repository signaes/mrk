use crate::element::Element;
use std::borrow::Cow;

#[derive(Debug)]
pub enum Node {
    Text(Cow<'static, str>),
    Element(Element),
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
}
