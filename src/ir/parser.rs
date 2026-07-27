//! `.mrk` wire-format parser.
//!
//! The [`Parser`] walks the byte stream line-by-line, dispatched by the
//! first non-blank token. Two entry points correspond to the two IR
//! shapes:
//!
//! - [`parse_root`](Parser::parse_root) — decodes an Element IR
//!   (introduced by `E 3:div` etc.).
//! - [`parse_component_root`](Parser::parse_component_root) — decodes
//!   a Component IR (introduced by `C 5:greet` etc.).
//!
//! The dispatch (in [`mod.rs`](super)) is responsible for picking the
//! right entry point based on the first non-blank line's token byte
//! and for scanning the `mrk1` header. Both entry points take the
//! already-scanned [`PeekedLine`] for the header, so they don't re-read
//! it.
//!
//! `Parser` itself is private (`pub(crate)`) — callers use
//! [`Mrk::from_bytes`], [`Mrk::from_bytes_component`], or the string
//! siblings.

use std::borrow::Cow;

use crate::attributes::{Attribute, AttributeType};
use crate::components::{Component, Expr, MatchArm};
use crate::components::WrappedAttribute;
use crate::element::Element;
use crate::node::Node;

use super::error::ParseError;
use super::helpers::{
    bytes_to_string, field_payload, parse_count, read_lp_value, validate_header,
};
use super::line::{parse_line, Line, PeekedLine};

/// Line-by-line decoder for the `.mrk` wire format.
///
/// Holds a position cursor into `src` and an optional one-ahead peek
/// buffer for `peek_non_blank()`. The peek buffer avoids re-scanning
/// when the parser needs to disambiguate a child vs. sibling line.
pub(crate) struct Parser<'a> {
    src: &'a [u8],
    /// Offset in `src` where the next line begins.
    pos: usize,
    /// Cumulative 1-indexed line number of the next physical line to scan.
    next_line_no: usize,
}

impl<'a> Parser<'a> {
    /// Create a new parser over `src`.
    pub(crate) fn new(src: &'a [u8]) -> Self {
        Parser {
            src,
            pos: 0,
            next_line_no: 1,
        }
    }

    /// Decode an Element IR, given that the header line has already
    /// been scanned.
    ///
    /// Dispatch guarantees a non-blank element line follows the header,
    /// so the `expect` for `scan_line` is safe.
    pub(crate) fn parse_root(&mut self, header: PeekedLine<'a>) -> Result<Element, ParseError> {
        validate_header(header.bytes)?;

        let element_line = self
            .scan_line()
            .expect("dispatch guarantees a non-blank element line");
        let parsed = parse_line(element_line.bytes);
        if parsed.indent != 0 {
            return Err(ParseError::BadNesting {
                line: element_line.line_no,
            });
        }
        self.parse_element(parsed, header.line_no)
    }

    /// Decode a Component IR, given that the header line has already
    /// been scanned.
    ///
    /// The body line after the component definition is required, so a
    /// truncated input here returns [`ParseError::UnexpectedEof`]
    /// (not an `.expect()`-style panic).
    pub(crate) fn parse_component_root(
        &mut self,
        header: PeekedLine<'a>,
    ) -> Result<Component, ParseError> {
        validate_header(header.bytes)?;
        let _ = header.line_no; // line number reserved for future use.

        // First line: `C <name>` at indent 0. Dispatch already filtered
        // by kind byte; we just confirm a name field is present.
        let comp_line = self
            .scan_line()
            .expect("dispatch guarantees a non-blank component line");
        let comp_parsed = parse_line(comp_line.bytes);
        let name_bytes = field_payload(comp_parsed.rest, 0, comp_line.line_no)?;
        let name_str = bytes_to_string(name_bytes, comp_line.line_no)?;

        // Body: a single Expr at indent 2.
        let expr_line = self.scan_line().ok_or(ParseError::UnexpectedEof)?;
        let expr_parsed = parse_line(expr_line.bytes);
        if expr_parsed.indent != 2 {
            return Err(ParseError::BadNesting {
                line: expr_line.line_no,
            });
        }
        let expr = self.parse_expr(expr_parsed, expr_line.line_no)?;

        Ok(Component {
            name: Cow::Owned(name_str),
            expr,
        })
    }

    /// Decode an `Element` body: name + attribute block + child block.
    ///
    /// Stops reading when it encounters a sibling line (same or lower
    /// indent) or runs out of input.
    fn parse_element(
        &mut self,
        header: Line<'a>,
        header_line_no: usize,
    ) -> Result<Element, ParseError> {
        let elem_indent = header.indent;

        let name_bytes = field_payload(header.rest, 0, header_line_no)?;
        let name_str = bytes_to_string(name_bytes, header_line_no)?;

        let mut element = Element {
            name: Cow::Owned(name_str),
            attributes: Vec::new(),
            children: Vec::new(),
        };
        let mut child_seen = false;

        loop {
            let next = self.peek_non_blank();
            let Some(peeked) = next else {
                return Ok(element);
            };
            let line_no = peeked.line_no;
            let parsed = parse_line(peeked.bytes);
            // Sibling or shallower line ends this element. `peek_non_blank`
            // already rewound, so the next `scan_line` will return
            // this same line for the parent parser.
            if parsed.indent <= elem_indent {
                return Ok(element);
            }
            if parsed.indent != elem_indent + 2 {
                return Err(ParseError::BadNesting { line: line_no });
            }
            // Consume the line.
            let _ = self.scan_line();
            match parsed.kind {
                b'A' => {
                    if child_seen {
                        return Err(ParseError::AttributeAfterChild { line: line_no });
                    }
                    let k = field_payload(parsed.rest, 0, line_no)?;
                    let v = field_payload(parsed.rest, 1, line_no)?;
                    let k_str = bytes_to_string(k, line_no)?;
                    let v_str = bytes_to_string(v, line_no)?;
                    element.attributes.push(Attribute {
                        key: Cow::Owned(k_str.clone()),
                        attr: AttributeType::KeyValue(Cow::Owned(k_str), Cow::Owned(v_str)),
                    });
                }
                b'B' => {
                    if child_seen {
                        return Err(ParseError::AttributeAfterChild { line: line_no });
                    }
                    let k = field_payload(parsed.rest, 0, line_no)?;
                    let k_str = bytes_to_string(k, line_no)?;
                    element.attributes.push(Attribute {
                        key: Cow::Owned(k_str.clone()),
                        attr: AttributeType::Bool(Cow::Owned(k_str)),
                    });
                }
                b'T' => {
                    child_seen = true;
                    let t = field_payload(parsed.rest, 0, line_no)?;
                    let t_str = bytes_to_string(t, line_no)?;
                    element.children.push(Node::Text(Cow::Owned(t_str)));
                }
                b'R' => {
                    child_seen = true;
                    let r = field_payload(parsed.rest, 0, line_no)?;
                    let r_str = bytes_to_string(r, line_no)?;
                    element.children.push(Node::Raw(Cow::Owned(r_str)));
                }
                b'E' => {
                    child_seen = true;
                    let child = self.parse_element(parsed, line_no)?;
                    element.children.push(Node::Element(child));
                }
                other => {
                    return Err(ParseError::UnknownToken {
                        line: line_no,
                        got: other,
                    });
                }
            }
        }
    }

    /// Consume the next non-blank line, returning its raw bytes and
    /// 1-indexed line number. Returns `None` at end-of-input.
    pub(crate) fn scan_line(&mut self) -> Option<PeekedLine<'a>> {
        let mut p = self.pos;
        loop {
            if p >= self.src.len() {
                return None;
            }
            let line_no = self.next_line_no;
            self.next_line_no += 1;
            let start = p;
            while p < self.src.len() && self.src[p] != b'\n' {
                p += 1;
            }
            let end = p;
            let eof = p == self.src.len();
            // Advance past the newline (or stay at EOF).
            p = if !eof { p + 1 } else { p };
            self.pos = p;
            let raw = &self.src[start..end];
            let raw = if raw.last() == Some(&b'\r') {
                &raw[..raw.len() - 1]
            } else {
                raw
            };
            let trimmed = raw.iter().take_while(|&&b| b == b' ').count();
            let after_indent = &raw[trimmed..];
            if after_indent.is_empty() {
                // blank line; if the last line was blank, the next
                // iteration's `p >= src.len()` returns None.
                continue;
            }
            return Some(PeekedLine {
                bytes: raw,
                line_no,
            });
        }
    }

    /// Peek the next non-blank line without consuming it. The parser
    /// state is rewound to the start of the line so the caller can
    /// later consume the same line via `scan_line`.
    fn peek_non_blank(&mut self) -> Option<PeekedLine<'a>> {
        let saved_pos = self.pos;
        let saved_line_no = self.next_line_no;
        let p = self.scan_line();
        self.pos = saved_pos;
        self.next_line_no = saved_line_no;
        p
    }

    /// Parse a single `Expr`. The first line is provided (already
    /// scanned). Returns the parsed Expr.
    fn parse_expr(
        &mut self,
        first: Line<'a>,
        line_no: usize,
    ) -> Result<Expr, ParseError> {
        match first.kind {
            b'L' => {
                // L <indent+2>: E block follows.
                let el_line = self.scan_line().ok_or(ParseError::UnexpectedEof)?;
                let el_parsed = parse_line(el_line.bytes);
                if el_parsed.indent != first.indent + 2 {
                    return Err(ParseError::BadNesting {
                        line: el_line.line_no,
                    });
                }
                if el_parsed.kind != b'E' {
                    return Err(ParseError::UnknownToken {
                        line: el_line.line_no,
                        got: el_parsed.kind,
                    });
                }
                let el = self.parse_element(el_parsed, el_line.line_no)?;
                Ok(Expr::Literal(el))
            }
            b'P' => {
                let key_bytes = field_payload(first.rest, 0, line_no)?;
                let key_str = bytes_to_string(key_bytes, line_no)?;
                Ok(Expr::Prop(Cow::Owned(key_str)))
            }
            b'S' => {
                // S <count> at indent N. <count> Exprs follow at indent N+2.
                let count_bytes = field_payload(first.rest, 0, line_no)?;
                let count_str = bytes_to_string(count_bytes, line_no)?;
                let count: usize = parse_count(&count_str, line_no)?;
                let mut items = Vec::with_capacity(count);
                for _ in 0..count {
                    let item_line = self.scan_line().ok_or(ParseError::UnexpectedEof)?;
                    let item_parsed = parse_line(item_line.bytes);
                    if item_parsed.indent != first.indent + 2 {
                        return Err(ParseError::BadNesting {
                            line: item_line.line_no,
                        });
                    }
                    let item = self.parse_expr(item_parsed, item_line.line_no)?;
                    items.push(Box::new(item));
                }
                Ok(Expr::List(items))
            }
            b'M' => {
                // M <key> <arm-count> at indent N.
                // <arm-count> pairs of (value, arm-expr) follow at indent N+2.
                // Then a default expr at indent N+2.
                let key_bytes = field_payload(first.rest, 0, line_no)?;
                let key_str = bytes_to_string(key_bytes, line_no)?;

                let count_bytes = field_payload(first.rest, 1, line_no)?;
                let count_str = bytes_to_string(count_bytes, line_no)?;
                let count: usize = parse_count(&count_str, line_no)?;

                let mut arms = Vec::with_capacity(count);
                for _ in 0..count {
                    let value_line = self.scan_line().ok_or(ParseError::UnexpectedEof)?;
                    let value_parsed = parse_line(value_line.bytes);
                    if value_parsed.indent != first.indent + 2 {
                        return Err(ParseError::BadNesting {
                            line: value_line.line_no,
                        });
                    }
                    let value_bytes = read_lp_value(value_parsed.rest, value_line.line_no)?;
                    let value_str = bytes_to_string(value_bytes, value_line.line_no)?;

                    let arm_line = self.scan_line().ok_or(ParseError::UnexpectedEof)?;
                    let arm_parsed = parse_line(arm_line.bytes);
                    if arm_parsed.indent != first.indent + 2 {
                        return Err(ParseError::BadNesting {
                            line: arm_line.line_no,
                        });
                    }
                    let arm_result = self.parse_expr(arm_parsed, arm_line.line_no)?;
                    arms.push(MatchArm {
                        value: Cow::Owned(value_str),
                        result: Box::new(arm_result),
                    });
                }

                // Default expr.
                let default_line = self.scan_line().ok_or(ParseError::UnexpectedEof)?;
                let default_parsed = parse_line(default_line.bytes);
                if default_parsed.indent != first.indent + 2 {
                    return Err(ParseError::BadNesting {
                        line: default_line.line_no,
                    });
                }
                let default = self.parse_expr(default_parsed, default_line.line_no)?;

                Ok(Expr::Match {
                    key: Cow::Owned(key_str),
                    arms,
                    default: Box::new(default),
                })
            }
            b'I' => {
                // I <condition> at indent N. then, otherwise follow at indent N+2.
                let cond_bytes = field_payload(first.rest, 0, line_no)?;
                let cond_str = bytes_to_string(cond_bytes, line_no)?;

                let then_line = self.scan_line().ok_or(ParseError::UnexpectedEof)?;
                let then_parsed = parse_line(then_line.bytes);
                if then_parsed.indent != first.indent + 2 {
                    return Err(ParseError::BadNesting {
                        line: then_line.line_no,
                    });
                }
                let then_expr = self.parse_expr(then_parsed, then_line.line_no)?;

                let else_line = self.scan_line().ok_or(ParseError::UnexpectedEof)?;
                let else_parsed = parse_line(else_line.bytes);
                if else_parsed.indent != first.indent + 2 {
                    return Err(ParseError::BadNesting {
                        line: else_line.line_no,
                    });
                }
                let else_expr = self.parse_expr(else_parsed, else_line.line_no)?;

                Ok(Expr::Either {
                    condition: Cow::Owned(cond_str),
                    then: Box::new(then_expr),
                    otherwise: Box::new(else_expr),
                })
            }
            b'O' => {
                // O <condition> at indent N. then follows at indent N+2.
                let cond_bytes = field_payload(first.rest, 0, line_no)?;
                let cond_str = bytes_to_string(cond_bytes, line_no)?;

                let then_line = self.scan_line().ok_or(ParseError::UnexpectedEof)?;
                let then_parsed = parse_line(then_line.bytes);
                if then_parsed.indent != first.indent + 2 {
                    return Err(ParseError::BadNesting {
                        line: then_line.line_no,
                    });
                }
                let then_expr = self.parse_expr(then_parsed, then_line.line_no)?;

                Ok(Expr::Maybe {
                    condition: Cow::Owned(cond_str),
                    then: Box::new(then_expr),
                })
            }
            b'F' => {
                // F <input> at indent N. body follows at indent N+2.
                let input_bytes = field_payload(first.rest, 0, line_no)?;
                let input_str = bytes_to_string(input_bytes, line_no)?;

                let body_line = self.scan_line().ok_or(ParseError::UnexpectedEof)?;
                let body_parsed = parse_line(body_line.bytes);
                if body_parsed.indent != first.indent + 2 {
                    return Err(ParseError::BadNesting {
                        line: body_line.line_no,
                    });
                }
                let body = self.parse_expr(body_parsed, body_line.line_no)?;

                Ok(Expr::Map {
                    input: Cow::Owned(input_str),
                    body: Box::new(body),
                })
            }
            b'W' => {
                // W <name> <attr-count> <body-count> at indent N.
                let name_bytes = field_payload(first.rest, 0, line_no)?;
                let name_str = bytes_to_string(name_bytes, line_no)?;

                let attr_count_bytes = field_payload(first.rest, 1, line_no)?;
                let attr_count_str = bytes_to_string(attr_count_bytes, line_no)?;
                let attr_count: usize = parse_count(&attr_count_str, line_no)?;

                let body_count_bytes = {
                    // Field 0 is name; field 1 is attr-count; field 2 is
                    // body-count.
                    field_payload(first.rest, 2, line_no)?
                };
                let body_count_str = bytes_to_string(body_count_bytes, line_no)?;
                let body_count: usize = parse_count(&body_count_str, line_no)?;

                let mut attributes = Vec::with_capacity(attr_count);
                for _ in 0..attr_count {
                    let attr_line = self.scan_line().ok_or(ParseError::UnexpectedEof)?;
                    let attr_parsed = parse_line(attr_line.bytes);
                    if attr_parsed.indent != first.indent + 2 {
                        return Err(ParseError::BadNesting {
                            line: attr_line.line_no,
                        });
                    }
                    let wa = self.parse_wrapped_attr(attr_parsed, attr_line.line_no)?;
                    attributes.push(wa);
                }

                let mut body = Vec::with_capacity(body_count);
                for _ in 0..body_count {
                    let body_line = self.scan_line().ok_or(ParseError::UnexpectedEof)?;
                    let body_parsed = parse_line(body_line.bytes);
                    if body_parsed.indent != first.indent + 2 {
                        return Err(ParseError::BadNesting {
                            line: body_line.line_no,
                        });
                    }
                    let body_expr = self.parse_expr(body_parsed, body_line.line_no)?;
                    body.push(Box::new(body_expr));
                }

                Ok(Expr::Wrap {
                    name: Cow::Owned(name_str),
                    attrs: attributes,
                    body,
                })
            }
            b'N' => {
                // N <count> at indent N. <count> Node lines follow at indent N+2.
                let count_bytes = field_payload(first.rest, 0, line_no)?;
                let count_str = bytes_to_string(count_bytes, line_no)?;
                let count: usize = parse_count(&count_str, line_no)?;

                let mut nodes = Vec::with_capacity(count);
                for _ in 0..count {
                    let node_line = self.scan_line().ok_or(ParseError::UnexpectedEof)?;
                    let node_parsed = parse_line(node_line.bytes);
                    if node_parsed.indent != first.indent + 2 {
                        return Err(ParseError::BadNesting {
                            line: node_line.line_no,
                        });
                    }
                    let node = self.parse_node(node_parsed, node_line.line_no)?;
                    nodes.push(node);
                }

                Ok(Expr::LiteralChildren(nodes))
            }
            other => Err(ParseError::UnknownToken {
                line: line_no,
                got: other,
            }),
        }
    }

    /// Parse a single attribute line (`A` or `B`).
    pub(crate) fn parse_attr(
        &mut self,
        first: Line<'a>,
        line_no: usize,
    ) -> Result<Attribute, ParseError> {
        match first.kind {
            b'A' => {
                let k = field_payload(first.rest, 0, line_no)?;
                let v = field_payload(first.rest, 1, line_no)?;
                let k_str = bytes_to_string(k, line_no)?;
                let v_str = bytes_to_string(v, line_no)?;
                Ok(Attribute {
                    key: Cow::Owned(k_str.clone()),
                    attr: AttributeType::KeyValue(Cow::Owned(k_str), Cow::Owned(v_str)),
                })
            }
            b'B' => {
                let k = field_payload(first.rest, 0, line_no)?;
                let k_str = bytes_to_string(k, line_no)?;
                Ok(Attribute {
                    key: Cow::Owned(k_str.clone()),
                    attr: AttributeType::Bool(Cow::Owned(k_str)),
                })
            }
            other => Err(ParseError::UnknownToken {
                line: line_no,
                got: other,
            }),
        }
    }

    /// Parse a wrapped attribute line for `Expr::Wrap`: `A`, `B`, or `D`.
    ///
    /// `A`/`B` produce [`WrappedAttribute::Static`]; `D` produces
    /// [`WrappedAttribute::Dynamic`] with the following expression at
    /// `indent + 2`.
    fn parse_wrapped_attr(
        &mut self,
        first: Line<'a>,
        line_no: usize,
    ) -> Result<WrappedAttribute, ParseError> {
        match first.kind {
            b'A' | b'B' => {
                let attr = self.parse_attr(first, line_no)?;
                Ok(WrappedAttribute::Static(attr))
            }
            b'D' => {
                let k = field_payload(first.rest, 0, line_no)?;
                let k_str = bytes_to_string(k, line_no)?;

                let expr_line = self.scan_line().ok_or(ParseError::UnexpectedEof)?;
                let expr_parsed = parse_line(expr_line.bytes);
                if expr_parsed.indent != first.indent + 2 {
                    return Err(ParseError::BadNesting {
                        line: expr_line.line_no,
                    });
                }
                let expr = self.parse_expr(expr_parsed, expr_line.line_no)?;
                Ok(WrappedAttribute::Dynamic(Cow::Owned(k_str), expr))
            }
            other => Err(ParseError::UnknownToken {
                line: line_no,
                got: other,
            }),
        }
    }

    /// Parse a single node line inside an `N` body. `T`/`R`/`E`.
    fn parse_node(
        &mut self,
        first: Line<'a>,
        line_no: usize,
    ) -> Result<Node, ParseError> {
        match first.kind {
            b'T' => {
                let t = field_payload(first.rest, 0, line_no)?;
                let t_str = bytes_to_string(t, line_no)?;
                Ok(Node::Text(Cow::Owned(t_str)))
            }
            b'R' => {
                let r = field_payload(first.rest, 0, line_no)?;
                let r_str = bytes_to_string(r, line_no)?;
                Ok(Node::Raw(Cow::Owned(r_str)))
            }
            b'E' => {
                let el = self.parse_element(first, line_no)?;
                Ok(Node::Element(el))
            }
            other => Err(ParseError::UnknownToken {
                line: line_no,
                got: other,
            }),
        }
    }
}
