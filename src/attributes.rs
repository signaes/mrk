use crate::renderable::Renderable;

enum Attr {
    KeyValue(&'static str, &'static str),
    Bool(&'static str),
}

impl Renderable for Attr {
    fn render(&self) -> String {
        match self {
            Attr::KeyValue(k, v) => format!("{}=\"{}\"", k, v),
            Attr::Bool(n) => n.to_string(),
        }
    }
}

pub struct Attribute {
    key: &'static str,
    attr: Attr,
}

impl Attribute {
    fn new(k: &'static str) -> Self {
        Attribute {
            key: k,
            attr: Attr::Bool(k),
        }
    }

    pub fn value(mut self, v: &'static str) -> Self {
        self.attr = Attr::KeyValue(self.key, v);

        self
    }
}

/// Creates an HTML attribute by name.
///
/// Boolean attribute by default (renders as just the name). Call
/// `.value(...)` to set a key/value pair.
///
/// # Example
///
/// ```
/// use mrk::*;
///
/// assert_eq!(attr("disabled").render(), "disabled");
/// assert_eq!(attr("href").value("/").render(), "href=\"/\"");
/// ```
pub fn attr(k: &'static str) -> Attribute {
    Attribute::new(k)
}

impl Renderable for Attribute {
    fn render(&self) -> String {
        self.attr.render()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_table() {
        let cases = [
            (attr("class").value("container"), "class=\"container\""),
            (attr("id").value("main"), "id=\"main\""),
            (attr("data-x").value("a&b"), "data-x=\"a&b\""),
            (attr("disabled"), "disabled"),
            (attr("checked"), "checked"),
        ];

        for (input, expected) in cases {
            assert_eq!(input.render(), expected);
        }
    }
}
