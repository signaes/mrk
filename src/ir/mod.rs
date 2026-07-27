//! `.mrk` on-the-wire format for `Element` and `Component`.
//!
//! The `.mrk` format is the canonical encoding used to persist
//! markup trees and templates. It's a line-oriented, length-prefixed,
//! binary-safe text format that round-trips losslessly with the
//! in-memory data model.
//!
//! # Feature flag
//!
//! This module requires the `ir` Cargo feature (off by default) and
//! transitively pulls in the `components` feature. With it enabled,
//! the public surface is:
//!
//! - [`Mrk`] — the entry point: encode/decode to `Vec<u8>` or
//!   `String`.
//! - [`ParseError`] — failure variants with line-number context.
//! - [`MAX_PAYLOAD`] — the wire-format size cap.
//!
//! # File map
//!
//! The implementation is split across files in this module for
//! readability:
//!
//! | File              | Responsibility                                          |
//! |-------------------|--------------------------------------------------------|
//! | mrk.rs          | `Mrk` struct + dispatch + `from_bytes_dispatch`         |
//! | error.rs        | `ParseError`, `MAX_PAYLOAD`, `Display`/`Error` impls     |
//! | line.rs         | `Line` + `PeekedLine` line-shape types, `parse_line`     |
//! | parser.rs       | `Parser` + `parse_root` / `parse_component_root` / `parse_expr` |
//! | encoder.rs      | `encode_element` / `encode_component` / `encode_expr`     |
//! | helpers.rs      | `consume_one_field`, `read_lp_value`, `field_payload`, `parse_count`, `validate_header`, `bytes_to_string` |
//! | display.rs      | `Display` for `Element`/`Node`/`Attribute`/`AttributeType` |
//!
//! # Format spec
//!
//! ```text
//! mrk1                       required header, must be the first non-blank line
//! E 3:div                    Element IR; element open; tag name follows as length-prefixed
//!   A 5:class 8:blue box     `A` = KeyValue attribute
//!   B 7:disabled             `B` = Bool attribute
//!   T 5:hello                `T` = Node::Text
//!   R 21:<em>x</em>          `R` = Node::Raw
//!   E 1:p                    nested element; children indented two more spaces
//! C 5:greet                  Component IR; component open
//!   W 1:p 1:0 0:0            `W` = Wrap element; (name, attr-count, body-count)
//! ```
//!
//! All length-prefixed strings use `<ascii-digits>:<bytes>`. The
//! length counts bytes (UTF-8 octets), not characters. Indentation
//! is two spaces per depth level; the parser does not depend on it
//! for correctness.
//!
//! # Examples
//!
//! ```
//! use mrk::*;
//!
//! let tree = el("div").children(nodes!["hello"]);
//! let bytes = Mrk::bytes(&tree);
//! let back = Mrk::from_bytes(&bytes).unwrap();
//! assert_eq!(tree, back);
//! ```

mod display;
mod encoder;
mod error;
mod helpers;
mod line;
mod mrk;
mod parser;

pub use error::{ParseError, MAX_PAYLOAD};
pub use mrk::Mrk;
