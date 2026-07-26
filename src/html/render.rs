//! HTML rendering impls for [`Element`], [`Node`], and [`Attribute`].

use crate::attributes::{Attribute, AttributeType};
use crate::element::Element;
use crate::node::Node;
use crate::renderable::Renderable;

use super::constants::VOID_ELEMENTS;

enum Context {
    Void,
    VoidWithAttrs,
    WithoutAttrs,
    WithAttrs,
}

fn join(items: Vec<String>, separator: &'static str) -> String {
    items.join(separator)
}

/// Escape HTML special characters in text content or attribute values.
/// Replaces `&`, `<`, `>`, `"` with their entity equivalents.
fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    s.chars().for_each(|c| match c {
        '&' => out.push_str("&amp;"),
        '<' => out.push_str("&lt;"),
        '>' => out.push_str("&gt;"),
        '"' => out.push_str("&quot;"),
        _ => out.push(c),
    });
    out
}

impl Renderable for Element {
    fn render(&self) -> String {
        let attributes = join(self.attributes.iter().map(|a| a.render()).collect(), " ");
        let children = join(self.children.iter().map(|c| c.render()).collect(), "");
        let is_void = VOID_ELEMENTS.contains(&self.name);
        let has_attrs = !attributes.is_empty();
        let context = match (is_void, has_attrs) {
            (true, true) => Context::VoidWithAttrs,
            (true, false) => Context::Void,
            (false, true) => Context::WithAttrs,
            (false, false) => Context::WithoutAttrs,
        };

        match context {
            Context::Void => format!("<{}>", self.name),
            Context::VoidWithAttrs => format!("<{} {}>", self.name, attributes),
            Context::WithoutAttrs => {
                format!("<{}>{}</{}>", self.name, children, self.name)
            }
            Context::WithAttrs => {
                format!("<{} {}>{}</{}>", self.name, attributes, children, self.name)
            }
        }
    }
}

impl Renderable for Node {
    fn render(&self) -> String {
        match self {
            Node::Text(s) => escape(s.as_ref()),
            Node::Element(e) => e.render(),
            Node::Raw(s) => s.as_ref().to_string(),
        }
    }
}

impl Renderable for Attribute {
    fn render(&self) -> String {
        match &self.attr {
            AttributeType::KeyValue(k, v) => format!("{}=\"{}\"", k, escape(v.as_ref())),
            AttributeType::Bool(k) => k.to_string(),
        }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.render())
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.render())
    }
}

impl std::fmt::Display for Attribute {
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
    fn attribute_render_table() {
        let cases = [
            (attr("class").value("container"), "class=\"container\""),
            (attr("id").value("main"), "id=\"main\""),
            (attr("data-x").value("a&b"), "data-x=\"a&amp;b\""),
            (attr("disabled"), "disabled"),
            (attr("checked"), "checked"),
        ];

        for (input, expected) in cases {
            assert_eq!(input.render(), expected);
        }
    }

    #[test]
    fn void_element_no_attrs() {
        assert_eq!(el("br").render(), "<br>");
    }

    #[test]
    fn void_element_with_attrs() {
        let html = el("img")
            .attrs(vec![attr("src").value("x.png")])
            .render();
        assert_eq!(html, "<img src=\"x.png\">");
    }

    #[test]
    fn non_void_empty() {
        assert_eq!(el("div").render(), "<div></div>");
    }

    #[test]
    fn non_void_with_text_child() {
        let html = el("p").children(vec!["Hello".into()]).render();
        assert_eq!(html, "<p>Hello</p>");
    }

    #[test]
    fn non_void_with_attrs_and_children() {
        let html = el("a")
            .attrs(vec![attr("href").value("/")])
            .children(vec!["Home".into()])
            .render();
        assert_eq!(html, r#"<a href="/">Home</a>"#);
    }

    #[test]
    fn nested_elements() {
        let html = el("div")
            .children(vec![el("strong").children(vec!["bold".into()]).into()])
            .render();
        assert_eq!(html, "<div><strong>bold</strong></div>");
    }

    #[test]
    fn display_impl() {
        let e = el("div");
        assert_eq!(format!("{}", e), "<div></div>");

        let n: Node = el("div").into();
        assert_eq!(format!("{}", n), "<div></div>");

        let a = attr("href").value("/");
        assert_eq!(format!("{}", a), "href=\"/\"");
    }

    #[test]
    fn escapes_text_content() {
        let html = el("p").children(vec!["a < b & c".into()]).render();
        assert_eq!(html, "<p>a &lt; b &amp; c</p>");
    }

    #[test]
    fn escapes_attribute_value_ampersand() {
        let a = attr("title").value("Tom & Jerry");
        assert_eq!(a.render(), "title=\"Tom &amp; Jerry\"");
    }

    #[test]
    fn escapes_attribute_value_quotes() {
        let a = attr("title").value("she said \"hi\"");
        assert_eq!(a.render(), "title=\"she said &quot;hi&quot;\"");
    }

    #[test]
    fn escapes_angle_brackets_in_text() {
        let n: Node = "<script>alert(1)</script>".into();
        assert_eq!(
            n.render(),
            "&lt;script&gt;alert(1)&lt;/script&gt;"
        );
    }

    #[test]
    fn does_not_double_escape() {
        let n: Node = "&lt;".into();
        assert_eq!(n.render(), "&amp;lt;");
    }

    #[test]
    fn preserves_safe_text() {
        let n: Node = "hello world".into();
        assert_eq!(n.render(), "hello world");
    }
}
