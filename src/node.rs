use crate::element::Element;
use crate::renderable::Renderable;

pub enum Node {
    Text(&'static str),
    Element(Element),
}

impl Renderable for Node {
    fn render(&self) -> String {
        match self {
            Node::Text(s) => s.to_string(),
            Node::Element(e) => e.render(),
        }
    }
}

impl From<Element> for Node {
    fn from(e: Element) -> Self {
        Node::Element(e)
    }
}

/// Creates a text node.
///
/// # Example
///
/// ```
/// use mrk::*;
///
/// assert_eq!(el("p").children(vec![text("Hello!")]).render(), "<p>Hello!</p>");
/// ```
pub fn text(s: &'static str) -> Node {
    Node::Text(s)
}

/// Wraps an element as a node so it can be used as a child inside
/// `.children(vec![...])`, since the vector must be homogeneous.
///
/// # Example
///
/// ```
/// use mrk::*;
///
/// let html = div().children(vec![
///     text("Hello, "),
///     node(el("strong").children(vec![text("world")])),
/// ]).render();
///
/// assert_eq!(html, "<div>Hello, <strong>world</strong></div>");
/// ```
pub fn node(e: Element) -> Node {
    Node::Element(e)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attributes::attr;
    use crate::element::el;

    #[test]
    fn render_table() {
        let cases = [
            (
                "text_simple",
                Node::Text("hello"),
                "hello",
            ),
            (
                "text_empty",
                Node::Text(""),
                "",
            ),
            (
                "text_special_chars_unescaped",
                Node::Text("a < b & c"),
                "a < b & c",
            ),
            (
                "element_empty",
                node(el("div")),
                "<div></div>",
            ),
            (
                "element_void",
                node(el("br")),
                "<br>",
            ),
            (
                "element_with_text_child",
                node(el("p").children(vec![text("hi")])),
                "<p>hi</p>",
            ),
            (
                "element_with_attrs",
                node(el("a").attrs(vec![attr("href").value("/")])),
                "<a href=\"/\"></a>",
            ),
            (
                "nested_element_via_text",
                node(el("div").children(vec![text("x")])),
                "<div>x</div>",
            ),
        ];

        for (name, node, expected) in cases {
            assert_eq!(node.render(), expected, "case: {name}");
        }
    }
}
