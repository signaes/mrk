//! `fmt::Display` impls for the data model.
//!
//! `Display` produces a simple struct-style repr (the same shape as
//! the derived `Debug` output) so it stays cheap and feature-agnostic.
//! For HTML, use [`Renderable::render`](crate::Renderable::render).
//! For the on-the-wire `.mrk` text, use
//! [`Mrk::to_string`](mrk_ir::Mrk::to_string).

use std::fmt;

use crate::attributes::Attribute;
use crate::element::Element;
use crate::node::Node;

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}