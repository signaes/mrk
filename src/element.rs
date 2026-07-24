use crate::attributes::Attribute;
use crate::constants;
use crate::node::Node;
use crate::renderable::Renderable;

pub struct Element {
    name: &'static str,
    attributes: Vec<Attribute>,
    children: Vec<Node>,
}

impl Renderable for Element {
    fn render(&self) -> String {
        enum Context {
            Void,
            VoidWithAttrs,
            WithoutAttrs,
            WithAttrs,
        }

        fn join(items: Vec<String>, separator: &'static str) -> String {
            items.join(separator)
        }

        let attributes = join(self.attributes.iter().map(|a| a.render()).collect(), " ");
        let children = join(self.children.iter().map(|a| a.render()).collect(), "");
        let is_void = constants::VOID_ELEMENTS.contains(&self.name);
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

impl Element {
    fn new(name: &'static str) -> Self {
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
    /// let html = el("a").attrs(vec![attr("href").value("/")]).render();
    /// assert_eq!(html, "<a href=\"/\"></a>");
    /// ```
    pub fn attrs(mut self, attributes: Vec<Attribute>) -> Self {
        self.attributes = attributes;

        self
    }

    /// Sets the element's children, replacing any previously set.
    ///
    /// To mix element and text children, wrap each element in [`node`].
    ///
    /// # Example
    ///
    /// ```
    /// use mrk::*;
    ///
    /// let html = el("p").children(vec![text("hi")]).render();
    /// assert_eq!(html, "<p>hi</p>");
    /// ```
    pub fn children(mut self, children: Vec<Node>) -> Self {
        self.children = children;
        self
    }
}

/// Creates an element with the given tag name.
///
/// For common tags, prefer the factory functions like [`div`](crate::div).
///
/// # Example
///
/// ```
/// use mrk::*;
///
/// assert_eq!(el("span").children(vec![text("hi")]).render(), "<span>hi</span>");
/// ```
pub fn el(name: &'static str) -> Element {
    Element::new(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attributes::attr;
    use crate::node::{Node, text};
    use crate::renderable::render;

    #[test]
    fn render_table() {
        let cases = [
            ("void_no_attrs", el("br"), "<br>"),
            (
                "void_keyvalue_attr",
                el("img").attrs(vec![attr("src").value("x.png")]),
                "<img src=\"x.png\">",
            ),
            (
                "void_bool_attr",
                el("input").attrs(vec![attr("disabled")]),
                "<input disabled>",
            ),
            (
                "void_mixed_attrs_order",
                el("input").attrs(vec![attr("type").value("text"), attr("disabled")]),
                "<input type=\"text\" disabled>",
            ),
            ("non_void_empty", el("div"), "<div></div>"),
            (
                "non_void_text_child",
                el("p").children(vec![text("hello")]),
                "<p>hello</p>",
            ),
            (
                "non_void_attrs_no_children",
                el("a").attrs(vec![attr("href").value("/")]),
                "<a href=\"/\"></a>",
            ),
            (
                "non_void_attrs_text_child",
                el("a")
                    .attrs(vec![attr("href").value("/")])
                    .children(vec![text("Home")]),
                "<a href=\"/\">Home</a>",
            ),
            (
                "non_void_multi_children",
                el("ul").children(vec![
                    Node::Element(el("li").children(vec![text("a")])),
                    Node::Element(el("li").children(vec![text("b")])),
                ]),
                "<ul><li>a</li><li>b</li></ul>",
            ),
            (
                "nested_one_level",
                el("div").children(vec![Node::Element(el("span").children(vec![text("x")]))]),
                "<div><span>x</span></div>",
            ),
            (
                "nested_three_levels",
                el("div").children(vec![Node::Element(
                    el("section").children(vec![Node::Element(el("p").children(vec![text("x")]))]),
                )]),
                "<div><section><p>x</p></section></div>",
            ),
            (
                "text_special_chars_unescaped",
                el("p").children(vec![text("a < b & c")]),
                "<p>a < b & c</p>",
            ),
        ];

        for (name, element, expected) in cases {
            assert_eq!(render(element), expected, "case: {name}");
        }
    }
}
