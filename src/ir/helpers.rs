//! Low-level helpers for parsing the `.mrk` wire format.
//!
//! Each helper operates on a slice of bytes for one line (or one
//! length-prefixed fragment within a line). They all return
//! `Result<…, ParseError>` and lean on the caller for line context.
//!
//! The helpers fall into three categories:
//!
//! - **Length-prefixed field readers**: [`consume_one_field`],
//!   [`read_lp_value`], [`field_payload`].
//! - **Counter helpers**: [`parse_count`] (accepts the empty string
//!   as zero for compact IR).
//! - **Header / string conversion**: [`validate_header`],
//!   [`bytes_to_string`].
//!
//! All `pub(crate)` — these are internal to the `ir` module.

use super::error::ParseError;
use super::MAX_PAYLOAD;

/// Try to consume one `<len>:<bytes>` field starting at index `start`
/// within `buf`.
///
/// Returns:
/// - `Ok(Some((start, end, next)))` — fields are valid; `end` is
///   the index right after the payload, `next` is one byte past any
///   trailing space (or `end` itself if no space follows).
/// - `Ok(None)` — `start >= buf.len()`: nothing left to read.
/// - `Err` on malformed length prefix, oversize payload, truncated
///   payload, or a non-space trailing byte.
///
/// `start` and `next` are useful when you want to chain field reads
/// (see [`field_payload`]).
pub(crate) fn consume_one_field(
    buf: &[u8],
    start: usize,
    line_no: usize,
) -> Result<Option<(usize, usize, usize)>, ParseError> {
    if start >= buf.len() {
        return Ok(None);
    }
    let colon_rel = buf[start..]
        .iter()
        .position(|&b| b == b':')
        .ok_or(ParseError::BadLengthPrefix { line: line_no })?;
    let colon = start + colon_rel;
    let len_bytes = &buf[start..colon];
    if len_bytes.is_empty() || !len_bytes.iter().all(|b| b.is_ascii_digit()) {
        return Err(ParseError::BadLengthPrefix { line: line_no });
    }
    // ASCII digits are valid UTF-8 by construction; parse directly.
    let len: usize = std::str::from_utf8(len_bytes)
        .expect("ASCII digits are valid UTF-8")
        .parse()
        .map_err(|_| ParseError::BadLengthPrefix { line: line_no })?;
    if len > MAX_PAYLOAD {
        return Err(ParseError::LengthExceedsCap {
            line: line_no,
            length: len,
        });
    }
    let payload_start = colon + 1;
    let payload_end = payload_start + len;
    if payload_end > buf.len() {
        return Err(ParseError::TruncatedPayload {
            line: line_no,
            expected: len,
            got: buf.len() - payload_start,
        });
    }
    // Next byte (if any) must be space or end of buffer.
    if payload_end < buf.len() && buf[payload_end] != b' ' {
        return Err(ParseError::BadLengthPrefix { line: line_no });
    }
    let next = if payload_end < buf.len() && buf[payload_end] == b' ' {
        payload_end + 1
    } else {
        payload_end
    };
    Ok(Some((start, payload_end, next)))
}

/// Read a length-prefixed value from a line that does NOT start with a
/// kind byte. Format: `<len>:<bytes>`. The entire `line` (after indent
/// stripping) is the value.
///
/// Used by the `M` arm-value field on the Component IR.
pub(crate) fn read_lp_value(line: &[u8], line_no: usize) -> Result<&[u8], ParseError> {
    let colon = line
        .iter()
        .position(|&b| b == b':')
        .ok_or(ParseError::BadLengthPrefix { line: line_no })?;
    let len_bytes = &line[..colon];
    if len_bytes.is_empty() || !len_bytes.iter().all(|b| b.is_ascii_digit()) {
        return Err(ParseError::BadLengthPrefix { line: line_no });
    }
    let len: usize = std::str::from_utf8(len_bytes)
        .expect("ASCII digits are valid UTF-8")
        .parse()
        .map_err(|_| ParseError::BadLengthPrefix { line: line_no })?;
    let payload_start = colon + 1;
    let payload_end = payload_start + len;
    if payload_end > line.len() {
        return Err(ParseError::TruncatedPayload {
            line: line_no,
            expected: len,
            got: line.len() - payload_start,
        });
    }
    Ok(&line[payload_start..payload_end])
}

/// Extract the payload bytes of the length-prefixed field at
/// `field_idx` within `after_kind` (which starts with the kind byte).
///
/// Supports `field_idx` 0, 1, or 2 — the only positions used by the IR.
///
/// Note: `field_idx = 0` reads the *first* field (right after the kind
/// byte), `1` reads the second, `2` the third. The function chains
/// [`consume_one_field`] reads until the requested index is reached.
pub(crate) fn field_payload(
    after_kind: &[u8],
    field_idx: usize,
    line_no: usize,
) -> Result<&[u8], ParseError> {
    let mut i = 1; // skip kind byte
    while i < after_kind.len() && after_kind[i] == b' ' {
        i += 1;
    }

    let (start, end) = {
        let mut result: Option<(usize, usize)> = None;
        for current_idx in 0..=field_idx {
            let (s, e, n) = match consume_one_field(after_kind, i, line_no)? {
                Some(v) => v,
                None => {
                    return Err(ParseError::MissingField {
                        line: line_no,
                        expected: after_kind.first().copied().unwrap_or(b'?'),
                    });
                }
            };
            if current_idx == field_idx {
                result = Some((s, e));
                break;
            }
            i = n;
        }
        result.expect("loop terminates by either returning or breaking")
    };
    let colon_rel = after_kind[start..]
        .iter()
        .position(|&b| b == b':')
        .expect("consume_one_field validated a colon");
    Ok(&after_kind[start + colon_rel + 1..end])
}

/// Parse a count field with the empty-string-as-zero leniency.
///
/// The wire format always emits counts as `1:N` (one byte payload, the
/// digit `N`). The parser also accepts an empty payload for compact
/// hand-crafted IR where `0:` legitimately means "zero". A non-empty
/// payload that isn't a valid `usize` returns [`ParseError::BadLengthPrefix`]
/// at the supplied `line_no`.
pub(crate) fn parse_count(s: &str, line_no: usize) -> Result<usize, ParseError> {
    if s.is_empty() {
        return Ok(0);
    }
    s.parse()
        .map_err(|_| ParseError::BadLengthPrefix { line: line_no })
}

/// Validate a header byte slice. Accepts exactly `mrk1`; rejects
/// anything else with either [`ParseError::MissingVersion`] (no
/// `mrk` prefix, or `mrk` followed by garbage) or
/// [`ParseError::UnknownVersion(n)`] (`mrk<n>` with a parseable
/// integer).
pub(crate) fn validate_header(bytes: &[u8]) -> Result<(), ParseError> {
    if bytes == b"mrk1" {
        return Ok(());
    }
    let parsed: Option<u32> = bytes
        .strip_prefix(b"mrk")
        .and_then(|rest| std::str::from_utf8(rest).ok())
        .and_then(|s| s.parse::<u32>().ok());
    match parsed {
        Some(v) => Err(ParseError::UnknownVersion(v)),
        None => Err(ParseError::MissingVersion),
    }
}

/// Convert a length-prefixed payload to a `String`.
///
/// UTF-8 failures are mapped to [`ParseError::BadLengthPrefix`] — the
/// string-form decoder returns that variant for any payload byte
/// failure (length prefix, UTF-8, etc.) so callers don't need to
/// distinguish.
pub(crate) fn bytes_to_string(b: &[u8], line_no: usize) -> Result<String, ParseError> {
    std::str::from_utf8(b)
        .map(|s| s.to_string())
        .map_err(|_| ParseError::BadLengthPrefix { line: line_no })
}
