//! `.mrk` wire-format encoder (the inverse of [`parser`](super::parser)).
//!
//! Each `encode_*` function appends bytes to a shared `Vec<u8>` so the
//! caller can either stream into a vector or wrap a `Cursor`-style
//! sink.
//!
//! Encoder failures are panics, not `Result`s:
//!
//! ```text
//! payload of 65537 bytes exceeds MAX_PAYLOAD (65536 bytes)
//! ```
//!
//! This is intentional — an encoder failure indicates a programming
//! error (a payload built larger than the wire format allows), not
//! untrusted input. The decoder ([`Mrk::from_bytes`]) is what handles
//! untrusted data and reports failures as [`ParseError`].

use crate::attributes::{Attribute, AttributeType};
use crate::components::{Component, Expr};
use crate::components::WrappedAttribute;
use crate::element::Element;
use crate::node::Node;

use super::error::MAX_PAYLOAD;

/// Write a length-prefixed string into `out`.
///
/// Format: `<ascii-digits>:<bytes>`. Panics if `s` exceeds
/// [`MAX_PAYLOAD`].
pub(crate) fn write_length_prefixed(s: &str, out: &mut Vec<u8>) {
    let bytes = s.as_bytes();
    let len = bytes.len();
    if len > MAX_PAYLOAD {
        panic!(
            "payload of {} bytes exceeds MAX_PAYLOAD ({} bytes)",
            len, MAX_PAYLOAD
        );
    }
    out.extend_from_slice(len.to_string().as_bytes());
    out.push(b':');
    out.extend_from_slice(bytes);
}

/// Indent prefix at a given `depth`. Two spaces per depth level.
pub(crate) fn indent(depth: usize) -> String {
    " ".repeat(depth * 2)
}

// =====================================================================
// Element IR encoder
// =====================================================================

/// Append an `Element` IR block starting at `depth`.
pub(crate) fn encode_element(e: &Element, depth: usize, out: &mut Vec<u8>) {
    out.extend_from_slice(indent(depth).as_bytes());
    out.extend_from_slice(b"E ");
    write_length_prefixed(e.name.as_ref(), out);
    out.push(b'\n');
    for attr in &e.attributes {
        encode_attribute(attr, depth + 1, out);
    }
    for child in &e.children {
        match child {
            Node::Text(t) => {
                out.extend_from_slice(indent(depth + 1).as_bytes());
                out.extend_from_slice(b"T ");
                write_length_prefixed(t.as_ref(), out);
                out.push(b'\n');
            }
            Node::Raw(r) => {
                out.extend_from_slice(indent(depth + 1).as_bytes());
                out.extend_from_slice(b"R ");
                write_length_prefixed(r.as_ref(), out);
                out.push(b'\n');
            }
            Node::Element(c) => encode_element(c, depth + 1, out),
            Node::Expr(_) => {
                unreachable!("Node::Expr must be resolved during Component::render, not during IR encoding")
            }
        }
    }
}

/// Append an `Attribute` IR line (`A` for KeyValue, `B` for Bool).
pub(crate) fn encode_attribute(a: &Attribute, depth: usize, out: &mut Vec<u8>) {
    match &a.attr {
        AttributeType::KeyValue(k, v) => {
            out.extend_from_slice(indent(depth).as_bytes());
            out.extend_from_slice(b"A ");
            write_length_prefixed(k.as_ref(), out);
            out.push(b' ');
            write_length_prefixed(v.as_ref(), out);
            out.push(b'\n');
        }
        AttributeType::Bool(k) => {
            out.extend_from_slice(indent(depth).as_bytes());
            out.extend_from_slice(b"B ");
            write_length_prefixed(k.as_ref(), out);
            out.push(b'\n');
        }
    }
}

// =====================================================================
// Component IR encoder
// =====================================================================

/// Append a `Component` IR block starting at `depth`.
pub(crate) fn encode_component(c: &Component, depth: usize, out: &mut Vec<u8>) {
    out.extend_from_slice(indent(depth).as_bytes());
    out.extend_from_slice(b"C ");
    write_length_prefixed(c.name.as_ref(), out);
    out.push(b'\n');
    encode_expr(&c.expr, depth + 1, out);
}

/// Append an [`Expr`] IR line, recursing into child expressions.
pub(crate) fn encode_expr(e: &Expr, depth: usize, out: &mut Vec<u8>) {
    out.extend_from_slice(indent(depth).as_bytes());
    match e {
        Expr::Literal(el) => {
            out.extend_from_slice(b"L\n");
            encode_element(el, depth + 1, out);
        }
        Expr::Prop(key) => {
            out.extend_from_slice(b"P ");
            write_length_prefixed(key.as_ref(), out);
            out.push(b'\n');
        }
        Expr::List(items) => {
            out.extend_from_slice(b"S ");
            write_length_prefixed(items.len().to_string().as_str(), out);
            out.push(b'\n');
            for item in items {
                encode_expr(item, depth + 1, out);
            }
        }
        Expr::Match { key, arms, default } => {
            out.extend_from_slice(b"M ");
            write_length_prefixed(key.as_ref(), out);
            out.push(b' ');
            write_length_prefixed(arms.len().to_string().as_str(), out);
            out.push(b'\n');
            for arm in arms {
                out.extend_from_slice(indent(depth + 1).as_bytes());
                write_length_prefixed(arm.value.as_ref(), out);
                out.push(b'\n');
                encode_expr(&arm.result, depth + 1, out);
            }
            encode_expr(default, depth + 1, out);
        }
        Expr::Either { condition, then, otherwise } => {
            out.extend_from_slice(b"I ");
            write_length_prefixed(condition.as_ref(), out);
            out.push(b'\n');
            encode_expr(then, depth + 1, out);
            encode_expr(otherwise, depth + 1, out);
        }
        Expr::Maybe { condition, then } => {
            out.extend_from_slice(b"O ");
            write_length_prefixed(condition.as_ref(), out);
            out.push(b'\n');
            encode_expr(then, depth + 1, out);
        }
        Expr::Map { input, body } => {
            out.extend_from_slice(b"F ");
            write_length_prefixed(input.as_ref(), out);
            out.push(b'\n');
            encode_expr(body, depth + 1, out);
        }
        Expr::Wrap { name, attrs, body } => {
            out.extend_from_slice(b"W ");
            write_length_prefixed(name.as_ref(), out);
            out.push(b' ');
            write_length_prefixed(attrs.len().to_string().as_str(), out);
            out.push(b' ');
            write_length_prefixed(body.len().to_string().as_str(), out);
            out.push(b'\n');
            for wa in attrs {
                match wa {
                    WrappedAttribute::Static(attr) => {
                        encode_attribute(attr, depth + 1, out);
                    }
                    WrappedAttribute::Dynamic(key, expr) => {
                        out.extend_from_slice(indent(depth + 1).as_bytes());
                        out.extend_from_slice(b"D ");
                        write_length_prefixed(key.as_ref(), out);
                        out.push(b'\n');
                        encode_expr(expr, depth + 2, out);
                    }
                }
            }
            for b in body {
                encode_expr(b, depth + 1, out);
            }
        }
        Expr::LiteralChildren(nodes) => {
            out.extend_from_slice(b"N ");
            write_length_prefixed(nodes.len().to_string().as_str(), out);
            out.push(b'\n');
            for n in nodes {
                encode_node(n, depth + 1, out);
            }
        }
    }
}

/// Append a single Node line (`T` for Text, `R` for Raw, `E` for Element).
pub(crate) fn encode_node(n: &Node, depth: usize, out: &mut Vec<u8>) {
    match n {
        Node::Text(t) => {
            out.extend_from_slice(indent(depth).as_bytes());
            out.extend_from_slice(b"T ");
            write_length_prefixed(t.as_ref(), out);
            out.push(b'\n');
        }
        Node::Raw(r) => {
            out.extend_from_slice(indent(depth).as_bytes());
            out.extend_from_slice(b"R ");
            write_length_prefixed(r.as_ref(), out);
            out.push(b'\n');
        }
        Node::Element(el) => {
            // `encode_element` writes its own indent at `depth`, so
            // we do not emit one here.
            encode_element(el, depth, out);
        }
        Node::Expr(_) => {
            unreachable!("Node::Expr must be resolved during Component::render, not during IR encoding")
        }
    }
}
