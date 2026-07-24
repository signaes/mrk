#[derive(Debug)]
pub enum AttributeType {
    KeyValue(&'static str, &'static str),
    Bool(&'static str),
}

#[derive(Debug)]
pub struct Attribute {
    pub key: &'static str,
    pub attr: AttributeType,
}

impl Attribute {
    pub fn new(k: &'static str) -> Self {
        Attribute {
            key: k,
            attr: AttributeType::Bool(k),
        }
    }

    pub fn value(mut self, v: &'static str) -> Self {
        self.attr = AttributeType::KeyValue(self.key, v);
        self
    }
}

pub fn attr(k: &'static str) -> Attribute {
    Attribute::new(k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_creates_keyvalue() {
        let a = attr("href").value("/");
        assert_eq!(a.key, "href");
        assert!(matches!(a.attr, AttributeType::KeyValue("href", "/")));
    }

    #[test]
    fn builder_creates_bool() {
        let a = attr("disabled");
        assert_eq!(a.key, "disabled");
        assert!(matches!(a.attr, AttributeType::Bool("disabled")));
    }

    #[test]
    fn struct_literal_construction() {
        let a = Attribute {
            key: "id",
            attr: AttributeType::KeyValue("id", "main"),
        };
        assert_eq!(a.key, "id");
    }

    #[test]
    fn direct_mutation() {
        let mut a = Attribute::new("class");
        a.attr = AttributeType::KeyValue("class", "container");
        assert!(matches!(a.attr, AttributeType::KeyValue("class", "container")));
    }

    #[test]
    fn debug_format() {
        let a = attr("href").value("/");
        let _ = format!("{:?}", a);
    }
}
