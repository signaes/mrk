//! HTML-style attributes: `key=value` pairs and bare `key` (boolean)
//! presence attributes.
//!
//! An [`Attribute`] is the unit you put on an
//! [`Element`](crate::Element). Use [`attr`] to start a builder and
//! either `.value(...)` (for `key=value`) or leave it bare (for `key`):
//!
//! ```
//! use mrk::*;
//!
//! let el = el("a")
//!     .attrs(vec![attr("href").value("/")])
//!     .children(nodes!["home"]);
//! ```

use std::borrow::Cow;

/// The two attribute flavors an HTML element can carry.
///
/// See the [`attr`] builder for the conventional way to construct one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeType {
    /// `key="value"` — the attribute carries a string value.
    ///
    /// The first [`Cow`] is the key, the second is the value.
    KeyValue(Cow<'static, str>, Cow<'static, str>),
    /// Boolean attribute — present without a value (e.g. `disabled`,
    /// `checked`, `readonly`).
    ///
    /// Holds only the key. Renders as just the key in the IR.
    Bool(Cow<'static, str>),
}

/// A single attribute on an element: name + flavor.
///
/// Built with [`attr`]:
///
/// ```
/// use mrk::*;
/// let a = attr("href").value("/");
/// assert_eq!(a.key, "href");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    /// The attribute name. `Cow::Borrowed` for literals, `Cow::Owned`
    /// for runtime strings.
    pub key: Cow<'static, str>,
    /// The attribute's value or boolean-flag flavor.
    pub attr: AttributeType,
}

impl Attribute {
    /// Construct an `Attribute` in [`AttributeType::Bool`] flavor from `key`.
    ///
    /// For a `key=value` pair, call `.value(v)` on the result.
    pub fn new(key: Cow<'static, str>) -> Self {
        Attribute {
            key: key.clone(),
            attr: AttributeType::Bool(key),
        }
    }

    /// Convert this attribute into a `key=value` pair, replacing any
    /// previous flavor.
    ///
    /// ```
    /// use mrk::*;
    /// let a = attr("class").value("container");
    /// ```
    pub fn value(mut self, v: impl Into<Cow<'static, str>>) -> Self {
        self.attr = AttributeType::KeyValue(self.key.clone(), v.into());

        self
    }
}

/// Begin building an attribute.
///
/// Returns a boolean-flavored attribute by default; chain `.value(v)`
/// to make it `key="value"`.
pub fn attr(k: &'static str) -> Attribute {
    Attribute::new(Cow::Borrowed(k))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_creates_keyvalue() {
        let a = attr("href").value("/");
        assert_eq!(a.key, "href");
        assert!(matches!(
            a.attr,
            AttributeType::KeyValue(Cow::Borrowed("href"), Cow::Borrowed("/"))
        ));
    }

    #[test]
    fn builder_creates_bool() {
        let a = attr("disabled");
        assert_eq!(a.key, "disabled");
        assert!(matches!(
            a.attr,
            AttributeType::Bool(Cow::Borrowed("disabled"))
        ));
    }

    #[test]
    fn struct_literal_construction() {
        let a = Attribute {
            key: Cow::Borrowed("id"),
            attr: AttributeType::KeyValue(Cow::Borrowed("id"), Cow::Borrowed("main")),
        };
        assert_eq!(a.key, "id");
    }

    #[test]
    fn direct_mutation() {
        let mut a = Attribute::new(Cow::Borrowed("class"));
        a.attr = AttributeType::KeyValue(Cow::Borrowed("class"), Cow::Borrowed("container"));
        assert!(matches!(
            a.attr,
            AttributeType::KeyValue(Cow::Borrowed("class"), Cow::Borrowed("container"))
        ));
    }

    #[test]
    fn debug_format() {
        let a = attr("href").value("/");
        let _ = format!("{:?}", a);
    }
}
