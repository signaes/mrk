//! `Display` implementations that delegate to the `.mrk` encoder.
//!
//! [`Element`], [`Node`], [`Attribute`], and [`AttributeType`] each
//! have a `Display` impl that produces the same text as
//! [`Mrk::to_string`] (for the structured types) or a stable
//! `key=value` / `key`-only form (for attribute and attribute-type).
//!
//! Implemented here rather than in the core data-type modules so the
//! encoder dependency stays in the `ir` module.

use std::fmt;

use crate::attributes::{Attribute, AttributeType};
use crate::element::Element;
use crate::node::Node;

use super::mrk::Mrk;

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&Mrk::to_string(self))
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Text(t) => f.write_str(t),
            Node::Raw(r) => f.write_str(r),
            Node::Element(e) => f.write_str(&Mrk::to_string(e)),
        }
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.attr {
            AttributeType::KeyValue(k, v) => write!(f, "{}={}", k, v),
            AttributeType::Bool(k) => f.write_str(k),
        }
    }
}

impl fmt::Display for AttributeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttributeType::KeyValue(k, v) => write!(f, "{}={}", k, v),
            AttributeType::Bool(k) => f.write_str(k),
        }
    }
}
