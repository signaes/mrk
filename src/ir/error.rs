//! Decode-error variants and the wire-format size cap.
//!
//! All errors here are produced by [`crate::ir::Mrk::from_bytes`] (and
//! the `_component` / `_string` siblings). Encoder failures
//! (oversize payload) result in a panic with a clear message:
//!
//! ```text
//! payload of 65537 bytes exceeds MAX_PAYLOAD (65536 bytes)
//! ```
//!
//! — there is no `Result` returned from the encoder, on the
//! principle that a misconfigured input should fail loudly at
//! construction time rather than silently corrupt the wire.

use std::fmt;

/// Maximum payload size (in bytes) for any single length-prefixed
/// string in the `.mrk` wire format.
///
/// 64 KiB. Larger payloads are rejected on both encode (panic) and
/// decode (`ParseError::LengthExceedsCap`).
pub const MAX_PAYLOAD: usize = 64 * 1024;

/// Errors produced by the IR decoder.
///
/// Each variant carries enough context (a `line` number, a bad byte,
/// etc.) for `format!("{}", err)` to produce a useful diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// The input did not start with a `mrk…` header line. Most often
    /// the input is empty or its first non-blank line has no token.
    MissingVersion,
    /// Header was `mrk<N>` for `N != 1`. The encoder always emits
    /// `mrk1`; a different integer suggests a future (or paste-error)
    /// version.
    UnknownVersion(u32),
    /// A token byte was neither `E`/`A`/`B`/`T`/`R` (Element IR) nor
    /// `L`/`P`/`S`/`M`/`I`/`O`/`F`/`W`/`N`/`C`/`D` (Component IR).
    UnknownToken {
        /// Line number where the unknown token appeared.
        line: usize,
        /// The token byte (cast through `as char` for display).
        got: u8,
    },
    /// Length prefix isn't all-ASCII-digits or has a stray trailing
    /// byte after the payload.
    BadLengthPrefix {
        /// Line number where the bad prefix appeared.
        line: usize,
    },
    /// A length prefix declared a payload bigger than
    /// [`MAX_PAYLOAD`].
    LengthExceedsCap {
        /// Line number.
        line: usize,
        /// The oversized length that was declared.
        length: usize,
    },
    /// A length prefix declared N bytes but only `< N` bytes were
    /// available.
    TruncatedPayload {
        /// Line number.
        line: usize,
        /// Declared payload size.
        expected: usize,
        /// Bytes actually present.
        got: usize,
    },
    /// An attribute (`A`/`B`) appeared after a child (`T`/`R`/`E`)
    /// in the same element. The wire format requires all attributes
    /// before any children.
    AttributeAfterChild {
        /// Line number of the offending attribute.
        line: usize,
    },
    /// The input ended before a complete line could be read. Most
    /// often: missing the `mrk1` header, missing the element line
    /// after a `W` body count, etc.
    UnexpectedEof,
    /// Element nesting inconsistent with indentation (e.g. a child at
    /// the same indent as its parent, or child indent ≠
    /// `parent_indent + 2`).
    BadNesting {
        /// Line number of the offending child line.
        line: usize,
    },
    /// A token line (`W`, `M`, `S`) declared more length-prefixed
    /// fields than the line actually contained.
    MissingField {
        /// Line number.
        line: usize,
        /// The token that needed a field (`b'W'`, `b'M'`, `b'S'`, …).
        expected: u8,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::MissingVersion => f.write_str("missing or invalid `mrk1` header"),
            ParseError::UnknownVersion(v) => write!(
                f,
                "unsupported IR version `mrk{}` (only `mrk1` is supported)",
                v
            ),
            ParseError::UnknownToken { line, got } => write!(
                f,
                "line {}: unknown token byte `{}`",
                line, *got as char
            ),
            ParseError::BadLengthPrefix { line } => {
                write!(f, "line {}: malformed length prefix", line)
            }
            ParseError::LengthExceedsCap { line, length } => write!(
                f,
                "line {}: payload of {} bytes exceeds MAX_PAYLOAD ({} bytes)",
                line, length, MAX_PAYLOAD
            ),
            ParseError::TruncatedPayload {
                line,
                expected,
                got,
            } => write!(
                f,
                "line {}: truncated payload, expected {} bytes, got {}",
                line, expected, got
            ),
            ParseError::AttributeAfterChild { line } => write!(
                f,
                "line {}: attribute line after child line within the same element",
                line
            ),
            ParseError::UnexpectedEof => f.write_str("unexpected end of input"),
            ParseError::BadNesting { line } => write!(
                f,
                "line {}: element nesting inconsistent with indentation",
                line
            ),
            ParseError::MissingField { line, expected } => write!(
                f,
                "line {}: missing length-prefixed field for `{}` token",
                line, *expected as char
            ),
        }
    }
}

impl std::error::Error for ParseError {}
