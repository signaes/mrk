//! Internal line-shape types shared by the parser and helpers.
//!
//! The IR is line-oriented, so the parser works on rows of bytes.
//! [`Line`] describes a row of bytes after indent-stripping, and
//! [`PeekedLine`] is the one-off "look-ahead" buffer the parser uses
//! for one-line lookahead.

/// A parsed wire line: indent, kind byte, and the rest-of-line slice.
///
/// `kind = after_indent[0]` — it's a `u8` because the IR uses raw
/// ASCII bytes for tokens (`E`, `C`, `L`, `P`, …) and we want a
/// match without the conversion overhead of `char`.
///
/// `rest` is the *trimmed* slice (it includes the kind byte and
/// everything after it).
#[derive(Debug, Clone, Copy)]
pub(crate) struct Line<'a> {
    /// Number of leading-space bytes the raw line started with.
    pub indent: usize,
    /// First byte of the trimmed line (the IR token).
    pub kind: u8,
    /// Trimmed line bytes (kind byte + payload).
    pub rest: &'a [u8],
}

/// One-ahead peek buffer used by the parser to disambiguate sibling
/// vs. child lines without re-reading from the source.
///
/// Lifetime `'a` ties it to the original source slice.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PeekedLine<'a> {
    /// Raw bytes (no indent-stripping, no trailing newline).
    pub bytes: &'a [u8],
    /// 1-indexed line number where this line was found.
    pub line_no: usize,
}

/// Split a raw byte line into indent/kind/rest.
///
/// Panics in debug builds if called on a blank line — the parser
/// guarantees non-blank inputs reach here.
pub(crate) fn parse_line<'a>(raw: &'a [u8]) -> Line<'a> {
    let trimmed_start = raw.iter().take_while(|&&b| b == b' ').count();
    let after_indent = &raw[trimmed_start..];
    debug_assert!(
        !after_indent.is_empty(),
        "parse_line called on blank line"
    );
    Line {
        indent: trimmed_start,
        kind: after_indent[0],
        rest: after_indent,
    }
}
