# `mrk::ir`

The `.mrk` on-the-wire format — encode/decode for `Element` and
`Component`.

This directory contains:

| File          | Responsibility                                               |
|---------------|--------------------------------------------------------------|
| mod.rs        | Module doc, public re-exports ([`Mrk`], [`ParseError`], [`MAX_PAYLOAD`]). |
| mrk.rs        | [`Mrk`] struct + methods, `Dispatch` enum, `from_bytes_dispatch`. |
| error.rs      | [`ParseError`], [`MAX_PAYLOAD`], `Display`/`Error` impls.    |
| line.rs       | `Line`, `PeekedLine` line-shape types, `parse_line`.        |
| parser.rs     | `Parser` + `parse_root` / `parse_component_root` / `parse_expr` / `parse_attr` / `parse_node` + `scan_line` / `peek_non_blank`. |
| encoder.rs    | `encode_element` / `encode_component` / `encode_expr` / `encode_node` + `write_length_prefixed` + `indent`. |
| helpers.rs    | `consume_one_field` / `read_lp_value` / `field_payload` / `parse_count` / `validate_header` / `bytes_to_string`. |
| display.rs    | `Display` impls for `Element` / `Node` / `Attribute` / `AttributeType`. |
| tests.rs      | Test module (compiled only under the `ir` feature).          |

Each file name above is the Rust module path *without* the
`mod.rs`. They are file-path references, not Rustdoc links, so they
are written as plain text rather than `[`…`]` (which would be parsed
as Rustdoc links).

## Feature flag

Everything here is gated behind the **`ir`** Cargo feature (off by
default) — which **depends on** `components`. Enable it like:

```toml
mrk = { version = "0.8.0", features = ["ir"] }
```

`ir` brings in the `components` module transitively.

## Format spec (version 1, header `mrk1`)

```text
mrk1                       required header, must be the first non-blank line
E 3:div                    Element IR; tag name follows as length-prefixed
  A 5:class 8:blue box     `A` = KeyValue attribute
  B 7:disabled             `B` = Bool attribute
  T 5:hello                `T` = Node::Text
  R 21:<em>x</em>          `R` = Node::Raw
  E 1:p                    nested element; children indented two more spaces
C 5:greet                  Component IR; component open
  W 1:p 1:0 0:0            `W` = Wrap element; (name, attr-count, body-count)
```

### Rules

1. The first non-blank line must be `mrk1`. Future versions increment
   the integer (the decoder returns `UnknownVersion(n)`).
2. Recognized tokens (one per line, after optional leading spaces):
   - **Element IR:** `E`, `A`, `B`, `T`, `R`.
   - **Component IR:** `L`, `P`, `S`, `M`, `I`, `O`, `F`, `W`, `N`,
     plus `C` (component open) and the inherited `E`/`A`/`B`/`T`/`R`.
   Anything else is [`ParseError::UnknownToken`].
3. All strings use the `<ascii-digits>:<bytes>` form. The decoder
   reads a greedy digit run terminated by `:` and then takes
   exactly that many bytes verbatim. The length counts bytes (UTF-8
   octets), not characters. No padding, no escapes — the format is
   binary-safe.
4. Indentation is two spaces per depth level; the parser does not
   depend on it for correctness, only for stable round-trip output.
5. Within an element block, attribute lines (`A`/`B`) come before
   child lines (`T`/`R`/`E`). Mixing is rejected as
   [`ParseError::AttributeAfterChild`].
6. Blank lines and trailing whitespace are ignored.
7. Payloads larger than [`MAX_PAYLOAD`] (`64 KiB`) are rejected on
   both encode (panic) and decode
   ([`ParseError::LengthExceedsCap`]).

## Public API

- [`Mrk::bytes`] / [`Mrk::from_bytes`] — Element IR.
- [`Mrk::bytes_component`] / [`Mrk::from_bytes_component`] —
  Component IR.
- [`Mrk::to_string`] / [`Mrk::from_string`] (and the `_component`
  counterparts) — UTF-8 string form.

A `Mrk::from_bytes` call on a Component IR returns
`ParseError::BadLengthPrefix { line: 1 }` (and vice-versa) — the
caller is expected to use the right `*_component` variant.

### `ParseError`

Ten variants. All carry enough context for `format!("{}", err)` to
be a useful diagnostic — line numbers, the bad token byte, expected
vs. found types, etc.

### `MAX_PAYLOAD`

64 KiB. The encoder panics on payloads over this cap; the decoder
returns `ParseError::LengthExceedsCap`.

## Implementation notes

### Dispatch (in `mrk.rs`)

`from_bytes_dispatch` picks the right decoder by reading the first
non-blank line after the header, switching on its kind byte
(`E` → `parse_root`, `C` → `parse_component_root`). The header line
is scanned once and passed in as a `PeekedLine` so the chosen entry
point doesn't re-read it.

### Line model (in `line.rs`)

Both the parser and the encoder work on raw byte slices broken into
lines by `\n` (with `\r\n` tolerated). `Line<'a>` describes
indent-stripped rows; `PeekedLine<'a>` is the one-ahead buffer the
parser uses to disambiguate child vs. sibling lines without re-scanning.

### Parser (in `parser.rs`)

`Parser<'a>` holds a position cursor and the peek buffer. The public
entry points are `parse_root` and `parse_component_root`; the rest
of the file is the recursive walk through the AST:

- `parse_element` — name + attrs + children; stops on sibling indent.
- `parse_expr` — the big `match` on the `Expr` kind byte. Each arm
  has the same structure: read fields with `field_payload(_, N, _)`,
  recurse into the body.
- `parse_attr` / `parse_node` — small matches for `A`/`B` and
  `T`/`R`/`E`.

### Encoder (in `encoder.rs`)

Each `encode_*` function appends bytes to a shared `Vec<u8>`. All
`encode_*` functions follow the same indent convention: they write
their own indent at the given `depth`, except `encode_element` which
is the only function expected to be called recursively from
non-encoder code (i.e. inside `Node::Element` in the wire format).

Encoders panic, not return `Result`. The input is presumed safe: a
64 KiB payload limit on the encoder is a programming-error guard,
not an untrusted-input guard. Untrusted input goes through
`Mrk::from_bytes`.

### Helpers (in `helpers.rs`)

Low-level length-prefixed helpers, all `pub(crate)`:

- `consume_one_field(buf, start, line_no)` — parse one `<len>:<bytes>`
  field starting at `start`.
- `read_lp_value(line, line_no)` — same, on a line without a kind
  byte (used by the `M` arm-value).
- `field_payload(after_kind, field_idx, line_no)` — extract the
  payload of field `field_idx` after a kind byte. Supports indices
  0, 1, 2 (the only positions used in the format).
- `parse_count(s, line_no)` — counts field with empty-as-zero
  leniency (so compact hand-crafted IR with `0:` works).
- `validate_header(bytes)` — accept exactly `mrk1`, otherwise emit
  `UnknownVersion(n)` or `MissingVersion`.
- `bytes_to_string(b, line_no)` — UTF-8 decode, mapped to
  `BadLengthPrefix` on failure.

### Display (in `display.rs`)

`fmt::Display` for `Element` / `Node` / `Attribute` /
`AttributeType`. The first two delegate to the encoder; attributes
use a stable `key=value` / `key`-only form.

## Examples

```rust,ignore
use mrk::*;

// Round-trip an Element.
let el = el("p").children(nodes!["hi"]);
let bytes = Mrk::bytes(&el);
let back = Mrk::from_bytes(&bytes).unwrap();
assert_eq!(el, back);

// Round-trip a Component.
use mrk::components::*;
let c = component(
    "greet",
    wrap(
        el("div").attrs(vec![attr("class").value("card")]),
        list![prop("name")],
    ),
);
let bytes = Mrk::bytes_component(&c);
let back_c = Mrk::from_bytes_component(&bytes).unwrap();
assert_eq!(c, back_c);
```

## Layout rationale

Eight files in the `ir/` directory to keep each focused:

- mod.rs is just doc + re-exports.
- error.rs is the failure catalog — easy to scan when debugging.
- line.rs is 30 lines and gets a file to itself because every
  other module uses `Line<'a>` and `PeekedLine<'a>`.
- mrk.rs is the public entry point.
- parser.rs is the bulk — ~430 lines of recursive descent.
- encoder.rs mirrors parser.rs, ~180 lines.
- helpers.rs is 130 lines of "small, standalone, called from many
  places" functions.
- display.rs is 30 lines of `Display` impls that depend on
  `Mrk::to_string`.

The split makes each file understandable on its own. The
tests.rs next to the production code keeps test and impl side by
side, matching the convention used in `components/`.
