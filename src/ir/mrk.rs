//! The `.mrk` wire-format encoder/decoder entry point.
//!
//! [`Mrk`] is a zero-sized struct that groups the encode / decode
//! methods for the wire format. Each method takes a type-appropriate
//! input and returns either a `Vec<u8>` / `String` (encode) or a
//! typed result (decode, with [`ParseError`] on failure).
//!
//! # Format versions
//!
//! All methods read or write version `mrk1`. A future format change
//! should bump to `mrk2` and dispatch through the header parsing at
//! [`super::helpers::validate_header`].
//!
//! # Two IR shapes
//!
//! The `.mrk` header is shared by both IR shapes:
//!
//! | Wire shape    | First content line | Decoder            |
//! |---------------|--------------------|--------------------|
//! | **Element IR** | `E 3:div`          | [`Mrk::from_bytes`]    |
//! | **Component IR** | `C 5:greet`        | [`Mrk::from_bytes_component`] |
//!
//! Calling the wrong decoder for a shape returns
//! [`ParseError::BadLengthPrefix`].

use crate::components::Component;
use crate::element::Element;

use super::encoder::{encode_component, encode_element};
use super::error::ParseError;
use super::parser::Parser;

/// Encoder/decoder for the `.mrk` intermediate representation.
///
/// This type holds no state. It's a namespace for the encode/decode
/// methods so call sites look like `Mrk::bytes(&tree)` rather than
/// `bytes(&tree)`.
pub struct Mrk;

impl Mrk {
    /// Encode an [`Element`] tree to its IR bytes form.
    ///
    /// # Panics
    ///
    /// Panics if any payload exceeds [`super::MAX_PAYLOAD`].
    pub fn bytes(e: &Element) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"mrk1\n");
        encode_element(e, 0, &mut out);
        out
    }

    /// Decode an [`Element`] tree from its IR bytes form.
    ///
    /// Returns [`ParseError::BadLengthPrefix`] if the input is a
    /// Component IR (the `C` token); use
    /// [`Mrk::from_bytes_component`] for those.
    pub fn from_bytes(b: &[u8]) -> Result<Element, ParseError> {
        from_bytes_dispatch(b).and_then(|d| match d {
            Dispatch::Element(e) => Ok(e),
            Dispatch::Component(_) => Err(ParseError::BadLengthPrefix { line: 1 }),
        })
    }

    /// Encode an [`Element`] tree to its IR string form (UTF-8).
    pub fn to_string(e: &Element) -> String {
        String::from_utf8(Self::bytes(e)).expect("IR encoder emits valid UTF-8")
    }

    /// Decode an [`Element`] tree from its IR string form.
    pub fn from_string(s: &str) -> Result<Element, ParseError> {
        Self::from_bytes(s.as_bytes())
    }

    /// Encode a [`Component`] to its IR bytes form.
    ///
    /// # Panics
    ///
    /// Panics if any payload exceeds [`super::MAX_PAYLOAD`].
    pub fn bytes_component(c: &Component) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"mrk1\n");
        encode_component(c, 0, &mut out);
        out
    }

    /// Decode a [`Component`] from its IR bytes form.
    ///
    /// Returns [`ParseError::BadLengthPrefix`] if the input is an
    /// Element IR; use [`Mrk::from_bytes`] for those.
    pub fn from_bytes_component(b: &[u8]) -> Result<Component, ParseError> {
        from_bytes_dispatch(b).and_then(|d| match d {
            Dispatch::Component(c) => Ok(c),
            Dispatch::Element(_) => Err(ParseError::BadLengthPrefix { line: 1 }),
        })
    }

    /// Encode a [`Component`] to its IR string form (UTF-8).
    pub fn to_string_component(c: &Component) -> String {
        String::from_utf8(Self::bytes_component(c)).expect("IR encoder emits valid UTF-8")
    }

    /// Decode a [`Component`] from its IR string form.
    pub fn from_string_component(s: &str) -> Result<Component, ParseError> {
        Self::from_bytes_component(s.as_bytes())
    }
}

/// Result of dispatching on the first non-blank line of an `.mrk`
/// document. Internal — callers pick the right outcome via
/// [`Mrk::from_bytes`] or [`Mrk::from_bytes_component`].
enum Dispatch {
    Element(Element),
    Component(Component),
}

/// Dispatch to either [`Parser::parse_root`] or
/// [`Parser::parse_component_root`] based on the first non-blank line's
/// kind byte.
///
/// Both parsers expect their pre-validated `mrk1` header to already
/// be in `Parser::peeked`; we read it once here and hand it off.
fn from_bytes_dispatch(b: &[u8]) -> Result<Dispatch, ParseError> {
    if b.is_empty() {
        return Err(ParseError::MissingVersion);
    }
    let mut header_seen = false;
    for line in b.split(|&b| b == b'\n') {
        if !header_seen {
            header_seen = true;
            continue;
        }
        if line.is_empty() {
            continue;
        }
        // Skip leading spaces.
        let trimmed = line
            .iter()
            .skip_while(|&&c| c == b' ')
            .copied()
            .collect::<Vec<u8>>();
        if trimmed.is_empty() {
            continue;
        }
        let mut p = Parser::new(b);
        // Read the header once, up front; both parsers receive the
        // already-scanned header.
        let header = p
            .scan_line()
            .expect("dispatch guarantees a non-blank header line");
        let result = match trimmed[0] {
            b'E' => p.parse_root(header).map(Dispatch::Element),
            b'C' => p.parse_component_root(header).map(Dispatch::Component),
            other => Err(ParseError::UnknownToken { line: 1, got: other }),
        };
        return result;
    }
    // No content lines after the header.
    Err(ParseError::UnexpectedEof)
}
