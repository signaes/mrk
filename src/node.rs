use crate::element::Element;
use crate::renderable::Renderable;
use std::borrow::Cow;

pub enum Node {
    Text(Cow<'static, str>),
    Element(Element),
}

impl Renderable for Node {
    fn render(&self) -> String {
        match self {
            Node::Text(s) => s.as_ref().to_string(),
            Node::Element(e) => e.render(),
        }
    }
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

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.render())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attributes::attr;
    use crate::element::el;

    #[test]
    fn render_table() {
        let cases = [
            ("text_simple", "hello".into(), "hello"),
            ("text_empty", "".into(), ""),
            (
                "text_special_chars_unescaped",
                "a < b & c".into(),
                "a < b & c",
            ),
            (
                "text_owned_string",
                String::from("owned").into(),
                "owned",
            ),
            (
                "element_empty",
                Node::Element(el("div")),
                "<div></div>",
            ),
            (
                "element_void",
                Node::Element(el("br")),
                "<br>",
            ),
            (
                "element_with_text_child",
                Node::Element(el("p").children(vec!["hi".into()])),
                "<p>hi</p>",
            ),
            (
                "element_with_attrs",
                Node::Element(el("a").attrs(vec![attr("href").value("/")])),
                "<a href=\"/\"></a>",
            ),
            (
                "nested_element_via_text",
                Node::Element(el("div").children(vec!["x".into()])),
                "<div>x</div>",
            ),
        ];

        for (name, node, expected) in cases {
            assert_eq!(node.render(), expected, "case: {name}");
        }
    }
}
