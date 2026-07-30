//! Runtime parser backing the [`css!`](crate::css!) macro.
//!
//! The `css!` macro is a thin wrapper: it `stringify!`s its token
//! stream and hands the resulting text to [`parse_stylesheet`].
//! Keeping the grammar in ordinary Rust — instead of a
//! `macro_rules!` token muncher — avoids macro recursion limits
//! entirely and makes every branch unit-testable.
//!
//! `stringify!` preserves source adjacency: tokens written glued
//! stay glued (`background-color`, `.btn`, `&:hover`), and
//! whitespace before punctuation is dropped (`red !important`
//! arrives as `red!important`). The `preprocess` pass only re-joins
//! the few artifacts that can still occur when tokens are written
//! with interior spacing (`. btn`, `# fff`, `- 1`, `! important`),
//! and `normalize_important` restores the whitespace `!important`
//! needs to be recognized. One consequence of token-based input:
//! whitespace is invisible, so `.btn.primary` and `.btn .primary`
//! written *with* interior spaces are indistinguishable. The
//! convention (inherited from v1) is that `:pseudo` / `::pseudo`
//! attach to the previous compound segment and juxtaposed `.` / `#`
//! / type selectors start descendant segments; compound chains are
//! written with a quoted selector (`"&.primary"`), which is passed
//! through as [`Selector::Raw`].

use std::borrow::Cow;

use crate::css::at_rules::{AtRule, Keyframe, RuleOrAtRule};
use crate::css::declaration::Declaration;
use crate::css::properties::{Value, parse_decl_value, split_important};
use crate::css::rule::{NestedBlock, Rule};
use crate::css::selector::{PseudoSelector, Selector, SelectorArg};
use crate::css::values::{CssString, CustomProperty};
use crate::css::StyleSheet;

/// Parse the `stringify!`ed contents of a `css! { … }` invocation
/// into a [`StyleSheet`].
///
/// This is the runtime entry point of the [`css!`](crate::css!)
/// macro; it is not meant to be called directly. Malformed input
/// panics with a message pointing at the offending fragment — since
/// the input always comes from a macro invocation, a panic surfaces
/// the call site in the compiler output.
#[doc(hidden)]
pub fn parse_stylesheet(input: &str) -> StyleSheet {
    let preprocessed = preprocess(input);
    StyleSheet::from_items(parse_sheet_items(&preprocessed))
}

// ── Preprocessing ───────────────────────────────────────────────────

/// Re-join token pairs that `stringify!` may still split when the
/// source was written with interior spacing.
///
/// Applied replacements (only outside string literals):
///
/// | artifact   | rejoined     | example source      |
/// |------------|--------------|---------------------|
/// | `". "` → `"."` | `.btn`   | `. btn { }`         |
/// | `"# "` → `"#"` | `#fff`   | `color: # fff`      |
/// | `"- "` → `"-"` | `-1`     | `margin: - 1`       |
/// | `"! "` → `"!"` | `!important` | `red ! important` |
fn preprocess(input: &str) -> String {
    const PATTERNS: [(&str, &str); 4] = [
        (". ", "."),
        ("# ", "#"),
        ("- ", "-"),
        ("! ", "!"),
    ];
    let mut out = String::with_capacity(input.len());
    let mut quote: Option<char> = None;
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if let Some(q) = quote {
            out.push(c);
            if c == '\\' {
                if let Some(escaped) = chars.next() {
                    out.push(escaped);
                }
            } else if c == q {
                quote = None;
            }
            continue;
        }
        if c == '"' || c == '\'' {
            quote = Some(c);
            out.push(c);
            continue;
        }
        let mut matched = false;
        for (pat, rep) in PATTERNS {
            let mut it = pat.chars();
            if it.next() == Some(c) && chars.peek() == it.next().as_ref() {
                out.push_str(rep);
                chars.next();
                matched = true;
                break;
            }
        }
        if !matched {
            out.push(c);
        }
    }
    out
}

// ── Chunking ────────────────────────────────────────────────────────

/// A top-level item inside a stylesheet, rule, or at-rule body.
enum Chunk {
    /// `prelude { body }` — a style rule or a block at-rule.
    Block { prelude: String, body: String },
    /// `statement;` — a declaration or a statement at-rule.
    Statement(String),
}

/// Split `body` into chunks at top-level `;` and `{ … }` boundaries.
/// Parentheses, brackets, and string literals shield their contents.
fn chunk_body(body: &str) -> Vec<Chunk> {
    let bytes = body.as_bytes();
    let mut chunks = Vec::new();
    let mut start = 0;
    let mut i = 0;
    let mut depth = 0i32;
    let mut quote: Option<u8> = None;
    while i < bytes.len() {
        let b = bytes[i];
        if let Some(q) = quote {
            if b == b'\\' {
                i += 2;
                continue;
            }
            if b == q {
                quote = None;
            }
            i += 1;
            continue;
        }
        match b {
            b'"' | b'\'' => {
                quote = Some(b);
                i += 1;
            }
            b'(' | b'[' => {
                depth += 1;
                i += 1;
            }
            b')' | b']' => {
                depth -= 1;
                i += 1;
            }
            b';' if depth == 0 => {
                let text = body[start..i].trim();
                if !text.is_empty() {
                    chunks.push(Chunk::Statement(text.to_string()));
                }
                i += 1;
                start = i;
            }
            b'{' if depth == 0 => {
                let prelude = body[start..i].trim().to_string();
                let close = matching_delimiter(body, i, b'{', b'}');
                chunks.push(Chunk::Block {
                    prelude,
                    body: body[i + 1..close].to_string(),
                });
                i = close + 1;
                start = i;
            }
            _ => i += 1,
        }
    }
    let tail = body[start..].trim();
    if !tail.is_empty() {
        panic!("css!: expected `;` or `{{ … }}` after `{tail}`");
    }
    chunks
}

/// Return the index of the delimiter closing the one at `open`
/// (string-literal aware). Panics on unbalanced input.
fn matching_delimiter(s: &str, open: usize, open_ch: u8, close_ch: u8) -> usize {
    let bytes = s.as_bytes();
    let mut depth = 0i32;
    let mut quote: Option<u8> = None;
    let mut i = open;
    while i < bytes.len() {
        let b = bytes[i];
        if let Some(q) = quote {
            if b == b'\\' {
                i += 2;
                continue;
            }
            if b == q {
                quote = None;
            }
            i += 1;
            continue;
        }
        match b {
            b'"' | b'\'' => quote = Some(b),
            _ if b == open_ch => depth += 1,
            _ if b == close_ch => {
                depth -= 1;
                if depth == 0 {
                    return i;
                }
            }
            _ => {}
        }
        i += 1;
    }
    panic!("css!: unbalanced `{}` in `{s}`", open_ch as char);
}

// ── Stylesheet items ────────────────────────────────────────────────

/// Parse the items of a stylesheet or a block at-rule body.
fn parse_sheet_items(body: &str) -> Vec<RuleOrAtRule> {
    chunk_body(body)
        .into_iter()
        .map(|chunk| match chunk {
            Chunk::Statement(stmt) => parse_at_statement(&stmt),
            Chunk::Block { prelude, body } => {
                if prelude.starts_with('@') {
                    RuleOrAtRule::AtRule(parse_at_rule(&prelude, &body))
                } else {
                    RuleOrAtRule::Rule(parse_rule(&prelude, &body))
                }
            }
        })
        .collect()
}

/// Parse a style rule: selector list + declarations + nested blocks.
fn parse_rule(prelude: &str, body: &str) -> Rule {
    let selectors = parse_selector_list(prelude);
    let mut declarations = Vec::new();
    let mut nested = Vec::new();
    for chunk in chunk_body(body) {
        match chunk {
            Chunk::Statement(stmt) => declarations.push(parse_declaration(&stmt)),
            Chunk::Block { prelude, body } => {
                if prelude.starts_with('@') {
                    nested.push(NestedBlock::AtRule(parse_at_rule(&prelude, &body)));
                } else {
                    nested.push(NestedBlock::Rule(parse_rule(&prelude, &body)));
                }
            }
        }
    }
    Rule {
        selectors,
        declarations,
        nested,
    }
}

/// Parse a body that may only contain declarations (`@font-face`,
/// `@page`, `@keyframes` stops).
fn parse_declarations_only(body: &str, ctx: &str) -> Vec<Declaration> {
    chunk_body(body)
        .into_iter()
        .map(|chunk| match chunk {
            Chunk::Statement(stmt) => parse_declaration(&stmt),
            Chunk::Block { prelude, .. } => {
                panic!("css!: {ctx} bodies only take declarations, got `{prelude} {{ … }}`")
            }
        })
        .collect()
}

// ── Declarations ────────────────────────────────────────────────────

/// Parse a `name: value` statement into a [`Declaration`].
fn parse_declaration(stmt: &str) -> Declaration {
    if stmt.starts_with('@') {
        panic!("css!: statement at-rules (@import, @charset, …) are only allowed at the top level, got `{stmt};`");
    }
    let (name, value) = stmt
        .split_once(':')
        .unwrap_or_else(|| panic!("css!: expected a `name: value;` declaration, got `{stmt};`"));
    let name = name.trim();
    if name.is_empty() {
        panic!("css!: empty property name in `{stmt};`");
    }
    if name.starts_with("--") && CustomProperty::new(Cow::Owned(name.to_string())).is_none() {
        panic!("css!: invalid custom-property name `{name}`");
    }
    let normalized = normalize_important(value.trim());
    let (value, important) = split_important(&normalized);
    let decl = Declaration::new(Cow::Owned(name.to_string()), parse_value_text(value));
    if important { decl.important() } else { decl }
}

/// Restore the whitespace before `!important` that `stringify!`
/// drops (`red !important` arrives as `red!important`), so
/// [`split_important`] can recognize the flag.
fn normalize_important(value: &str) -> Cow<'_, str> {
    let idx = value.to_lowercase().rfind("!important");
    let Some(idx) = idx else {
        return Cow::Borrowed(value);
    };
    let after = idx + "!important".len();
    let boundary = after == value.len()
        || value.as_bytes()[after].is_ascii_whitespace();
    if idx == 0 || !boundary || value.as_bytes()[idx - 1].is_ascii_whitespace() {
        return Cow::Borrowed(value);
    }
    let mut s = String::with_capacity(value.len() + 1);
    s.push_str(&value[..idx]);
    s.push(' ');
    s.push_str(&value[idx..]);
    Cow::Owned(s)
}

/// Parse a declaration value. Quoted strings become
/// [`Value::String`]; everything else goes through the typed value
/// parser.
fn parse_value_text(text: &str) -> Value {
    if let Some(inner) = unquote(text) {
        return Value::String(CssString::new(inner));
    }
    parse_decl_value(text)
}

// ── At-rules ────────────────────────────────────────────────────────

/// Split `@keyword rest…` into the keyword and the remaining prelude.
fn at_keyword(text: &str) -> (&str, &str) {
    let rest = text.strip_prefix('@').unwrap_or(text).trim_start();
    let end = rest
        .find(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
        .unwrap_or(rest.len());
    (&rest[..end], rest[end..].trim_start())
}

/// Parse a block at-rule (`@media … { }`, `@keyframes … { }`, …).
fn parse_at_rule(prelude: &str, body: &str) -> AtRule {
    let (keyword, rest) = at_keyword(prelude);
    match keyword {
        "media" => AtRule::Media {
            query: required_prelude(rest, prelude),
            rules: parse_sheet_items(body),
        },
        "supports" => AtRule::Supports {
            condition: required_prelude(rest, prelude),
            rules: parse_sheet_items(body),
        },
        "container" => parse_container(rest, body, prelude),
        "scope" => parse_scope(rest, body),
        "layer" => AtRule::Layer {
            name: optional_name(rest),
            rules: parse_sheet_items(body),
        },
        "keyframes" => parse_keyframes(rest, body, prelude),
        "font-face" => AtRule::FontFace {
            declarations: parse_declarations_only(body, "@font-face"),
        },
        "page" => AtRule::Page {
            pseudo: optional_name(rest),
            declarations: parse_declarations_only(body, "@page"),
        },
        other => panic!("css!: unsupported at-rule `@{other}`"),
    }
}

/// Parse a statement at-rule (`@charset …;`, `@import …;`,
/// `@layer …;`, `@namespace …;`) at the top level.
fn parse_at_statement(stmt: &str) -> RuleOrAtRule {
    if !stmt.starts_with('@') {
        panic!("css!: declarations need a rule around them — got a top-level `{stmt};`");
    }
    let (keyword, rest) = at_keyword(stmt);
    let at = match keyword {
        "charset" => AtRule::Charset {
            encoding: Cow::Owned(unquote(rest).unwrap_or_else(|| rest.to_string())),
        },
        "import" => parse_import(rest),
        "layer" => AtRule::Layer {
            name: optional_name(rest),
            rules: Vec::new(),
        },
        "namespace" => parse_namespace(rest),
        other => panic!("css!: `@{other}` needs a `{{ … }}` block, got the statement `{stmt};`"),
    };
    RuleOrAtRule::AtRule(at)
}

/// A non-empty at-rule prelude, unquoted if it was written as a
/// string literal.
fn required_prelude(rest: &str, prelude: &str) -> Cow<'static, str> {
    if rest.is_empty() {
        panic!("css!: `{prelude}` needs a prelude");
    }
    Cow::Owned(unquote(rest).unwrap_or_else(|| rest.to_string()))
}

/// An optional at-rule name / pseudo (empty → `None`).
fn optional_name(rest: &str) -> Option<Cow<'static, str>> {
    if rest.is_empty() {
        None
    } else {
        Some(Cow::Owned(rest.to_string()))
    }
}

/// `@container [name] (condition) { … }`.
fn parse_container(rest: &str, body: &str, prelude: &str) -> AtRule {
    let Some(open) = rest.find('(') else {
        panic!("css!: @container needs a `(condition)` prelude, got `{prelude}`");
    };
    let name = rest[..open].trim();
    AtRule::Container {
        name: if name.is_empty() {
            None
        } else {
            Some(Cow::Owned(name.to_string()))
        },
        query: Cow::Owned(strip_parens(&rest[open..], prelude)),
        rules: parse_sheet_items(body),
    }
}

/// `@scope [(root)] [to (limit)] { … }`.
fn parse_scope(rest: &str, body: &str) -> AtRule {
    let mut s = rest.trim();
    let mut root = None;
    let mut limit = None;
    if s.starts_with('(') {
        let close = matching_delimiter(s, 0, b'(', b')');
        root = Some(Cow::Owned(s[1..close].trim().to_string()));
        s = s[close + 1..].trim();
    }
    if let Some(tail) = s.strip_prefix("to") {
        limit = Some(Cow::Owned(strip_parens(tail.trim_start(), "@scope")));
    } else if !s.is_empty() {
        panic!("css!: unexpected `{s}` in @scope prelude (expected `to (…)`)");
    }
    AtRule::Scope {
        root,
        limit,
        rules: parse_sheet_items(body),
    }
}

/// `@keyframes name { stop { … } … }`.
fn parse_keyframes(name: &str, body: &str, prelude: &str) -> AtRule {
    if name.is_empty() {
        panic!("css!: @keyframes needs a name, got `{prelude}`");
    }
    let name = Cow::Owned(unquote(name).unwrap_or_else(|| name.to_string()));
    let mut keyframes = Vec::new();
    for chunk in chunk_body(body) {
        match chunk {
            Chunk::Block { prelude, body } => {
                let selectors = prelude
                    .split(',')
                    .map(|s| Cow::Owned(s.trim().to_string()))
                    .collect();
                keyframes.push(Keyframe {
                    selectors,
                    declarations: parse_declarations_only(&body, "@keyframes"),
                });
            }
            Chunk::Statement(stmt) => {
                panic!("css!: @keyframes stops look like `from {{ … }}` or `50% {{ … }}`, got `{stmt};`")
            }
        }
    }
    AtRule::Keyframes { name, keyframes }
}

/// `@import url [supports(…)] [media];`.
fn parse_import(rest: &str) -> AtRule {
    let (url_tok, tail) = split_first_token(rest);
    if url_tok.is_empty() {
        panic!("css!: @import needs a url");
    }
    let url = Cow::Owned(url_text(url_tok));
    let mut supports = None;
    let mut media = None;
    if let Some(t) = tail.strip_prefix("supports") {
        let t = t.trim_start();
        if !t.starts_with('(') {
            panic!("css!: expected `supports(…)` in @import, got `{tail}`");
        }
        let close = matching_delimiter(t, 0, b'(', b')');
        supports = Some(Cow::Owned(t[1..close].trim().to_string()));
        let after = t[close + 1..].trim();
        if !after.is_empty() {
            media = Some(Cow::Owned(after.to_string()));
        }
    } else if !tail.is_empty() {
        media = Some(Cow::Owned(tail.to_string()));
    }
    AtRule::Import {
        url,
        supports,
        media,
    }
}

/// `@namespace [prefix] url;`.
fn parse_namespace(rest: &str) -> AtRule {
    let (first, tail) = split_first_token(rest);
    if first.is_empty() {
        panic!("css!: @namespace needs a url");
    }
    if tail.is_empty() {
        AtRule::Namespace {
            prefix: None,
            url: Cow::Owned(url_text(first)),
        }
    } else {
        let (url_tok, extra) = split_first_token(tail);
        if !extra.is_empty() {
            panic!("css!: unexpected `{extra}` in @namespace");
        }
        AtRule::Namespace {
            prefix: Some(Cow::Owned(first.to_string())),
            url: Cow::Owned(url_text(url_tok)),
        }
    }
}

/// Split a string at its first top-level whitespace, respecting
/// parentheses and quotes. Returns `(head, tail)`.
fn split_first_token(s: &str) -> (&str, &str) {
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut depth = 0i32;
    let mut quote: Option<u8> = None;
    while i < bytes.len() {
        let b = bytes[i];
        if let Some(q) = quote {
            if b == q {
                quote = None;
            }
            i += 1;
            continue;
        }
        match b {
            b'"' | b'\'' => quote = Some(b),
            b'(' => depth += 1,
            b')' => depth -= 1,
            _ if b.is_ascii_whitespace() && depth == 0 => break,
            _ => {}
        }
        i += 1;
    }
    (&s[..i], s[i..].trim_start())
}

/// Extract the URL from a `"…"`, `'…'`, `url(…)`, or bare token.
fn url_text(tok: &str) -> String {
    if let Some(inner) = unquote(tok) {
        return inner;
    }
    if let Some(rest) = tok.strip_prefix("url") {
        let rest = rest.trim_start();
        if rest.starts_with('(') && rest.ends_with(')') && rest.len() >= 2 {
            let inner = rest[1..rest.len() - 1].trim();
            return unquote(inner).unwrap_or_else(|| inner.to_string());
        }
    }
    tok.to_string()
}

/// Strip the outer parentheses of a `( … )` group.
fn strip_parens(s: &str, ctx: &str) -> String {
    let s = s.trim();
    if s.starts_with('(') && s.ends_with(')') && s.len() >= 2 {
        s[1..s.len() - 1].trim().to_string()
    } else {
        panic!("css!: expected `( … )` in `{ctx}`");
    }
}

/// Strip surrounding matching quotes from a string literal.
fn unquote(s: &str) -> Option<String> {
    let s = s.trim();
    let bytes = s.as_bytes();
    if bytes.len() >= 2 {
        let first = bytes[0];
        if (first == b'"' || first == b'\'') && bytes[bytes.len() - 1] == first {
            return Some(s[1..s.len() - 1].to_string());
        }
    }
    None
}

// ── Selectors ───────────────────────────────────────────────────────

/// Parse a rule prelude into a selector list.
fn parse_selector_list(prelude: &str) -> Vec<Selector> {
    let prelude = prelude.trim();
    if prelude.is_empty() {
        panic!("css!: rule with an empty selector");
    }
    if let Some(raw) = unquote(prelude) {
        return vec![Selector::Raw(Cow::Owned(raw))];
    }
    split_top_level_commas(prelude)
        .iter()
        .map(|s| parse_complex_selector(s))
        .collect()
}

/// Split a selector list at top-level commas.
fn split_top_level_commas(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut depth = 0i32;
    let mut quote: Option<char> = None;
    for c in text.chars() {
        if let Some(q) = quote {
            cur.push(c);
            if c == q {
                quote = None;
            }
            continue;
        }
        match c {
            '"' | '\'' => {
                quote = Some(c);
                cur.push(c);
            }
            '(' | '[' => {
                depth += 1;
                cur.push(c);
            }
            ')' | ']' => {
                depth -= 1;
                cur.push(c);
            }
            ',' if depth == 0 => {
                out.push(cur.trim().to_string());
                cur.clear();
            }
            _ => cur.push(c),
        }
    }
    out.push(cur.trim().to_string());
    out
}

/// Combinator joining two compound selector segments.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Comb {
    Descendant,
    Child,
    Sibling,
    GeneralSibling,
}

/// Parse one complex selector (no top-level commas).
fn parse_complex_selector(text: &str) -> Selector {
    let mut chain: Option<Selector> = None;
    for (comb, segment) in selector_segments(text) {
        let seg = parse_compound(&segment);
        chain = Some(match (chain, comb) {
            (None, _) => seg,
            (Some(a), Comb::Descendant) => Selector::Descendant(Box::new(a), Box::new(seg)),
            (Some(a), Comb::Child) => Selector::Child(Box::new(a), Box::new(seg)),
            (Some(a), Comb::Sibling) => Selector::Sibling(Box::new(a), Box::new(seg)),
            (Some(a), Comb::GeneralSibling) => {
                Selector::GeneralSibling(Box::new(a), Box::new(seg))
            }
        });
    }
    chain.unwrap_or_else(|| panic!("css!: empty selector in `{text}`"))
}

/// Split a complex selector into `(combinator, segment)` pairs.
/// Whitespace separates descendant segments; `>`, `+`, `~` are
/// explicit combinators.
fn selector_segments(text: &str) -> Vec<(Comb, String)> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut pending = Comb::Descendant;
    let mut depth = 0i32;
    for c in text.chars() {
        match c {
            '(' | '[' => {
                depth += 1;
                cur.push(c);
            }
            ')' | ']' => {
                depth -= 1;
                cur.push(c);
            }
            '>' | '+' | '~' if depth == 0 => {
                flush_segment(&mut out, &mut cur, &mut pending);
                pending = match c {
                    '>' => Comb::Child,
                    '+' => Comb::Sibling,
                    _ => Comb::GeneralSibling,
                };
            }
            c if c.is_whitespace() && depth == 0 => {
                flush_segment(&mut out, &mut cur, &mut pending);
            }
            _ => cur.push(c),
        }
    }
    flush_segment(&mut out, &mut cur, &mut pending);
    if pending != Comb::Descendant {
        panic!("css!: dangling combinator in selector `{text}`");
    }
    out
}

/// Push the accumulated segment with its pending combinator, if any.
fn flush_segment(out: &mut Vec<(Comb, String)>, cur: &mut String, pending: &mut Comb) {
    if !cur.is_empty() {
        out.push((*pending, std::mem::take(cur)));
        *pending = Comb::Descendant;
    }
}

/// Parse one compound segment (`.btn:hover`, `a`, `&`, …) into a
/// selector. A single simple selector is returned as-is; multiple
/// parts become [`Selector::Compound`].
fn parse_compound(seg: &str) -> Selector {
    let mut parts: Vec<Selector> = Vec::new();
    let mut chars = seg.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '.' => parts.push(Selector::Class(Cow::Owned(required_ident(&mut chars, seg)))),
            '#' => parts.push(Selector::Id(Cow::Owned(required_ident(&mut chars, seg)))),
            '&' => parts.push(Selector::NestingRef),
            '*' => parts.push(Selector::Universal),
            ':' => {
                let element = chars.next_if_eq(&':').is_some();
                let name = required_ident(&mut chars, seg);
                if chars.next_if_eq(&'(').is_some() {
                    let args = take_paren_args(&mut chars, seg);
                    parts.push(Selector::Pseudo(PseudoSelector::Function {
                        name: Cow::Owned(name),
                        args: vec![SelectorArg::AnPlusB(Cow::Owned(args))],
                    }));
                } else if element {
                    parts.push(Selector::pseudo_element(Cow::Owned(name)));
                } else {
                    parts.push(Selector::pseudo_class(Cow::Owned(name)));
                }
            }
            c if is_ident_char(c) => {
                let mut name = String::from(c);
                name.push_str(&take_ident(&mut chars));
                parts.push(Selector::Type(Cow::Owned(name)));
            }
            other => panic!("css!: unexpected `{other}` in selector `{seg}`"),
        }
    }
    if parts.len() == 1 {
        parts.remove(0)
    } else {
        Selector::Compound(parts)
    }
}

/// Consume identifier characters (alphanumeric, `-`, `_`); may
/// return an empty string.
fn take_ident(chars: &mut std::iter::Peekable<std::str::Chars>) -> String {
    let mut name = String::new();
    while let Some(&c) = chars.peek() {
        if !is_ident_char(c) {
            break;
        }
        name.push(c);
        chars.next();
    }
    name
}

/// Like [`take_ident`], but panics when no identifier follows.
fn required_ident(chars: &mut std::iter::Peekable<std::str::Chars>, seg: &str) -> String {
    let name = take_ident(chars);
    if name.is_empty() {
        panic!("css!: expected an identifier in selector `{seg}`");
    }
    name
}

/// Consume up to and including the matching `)` of a just-opened
/// parenthesis; returns the inner text.
fn take_paren_args(chars: &mut std::iter::Peekable<std::str::Chars>, seg: &str) -> String {
    let mut depth = 1i32;
    let mut out = String::new();
    for c in chars.by_ref() {
        match c {
            '(' => {
                depth += 1;
                out.push(c);
            }
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return out;
                }
                out.push(c);
            }
            _ => out.push(c),
        }
    }
    panic!("css!: unbalanced `(` in selector `{seg}`");
}

/// Identifier characters accepted in selectors.
fn is_ident_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-' || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderable::Renderable;

    fn render(input: &str) -> String {
        parse_stylesheet(input).render()
    }

    // ── preprocess ─────────────────────────────────────────────

    #[test]
    fn preprocess_joins_dot() {
        assert_eq!(preprocess(". btn . text"), ".btn .text");
    }

    #[test]
    fn preprocess_joins_hash() {
        assert_eq!(preprocess("color: # fff"), "color: #fff");
    }

    #[test]
    fn preprocess_joins_dash() {
        assert_eq!(preprocess("margin: - 1.5"), "margin: -1.5");
    }

    #[test]
    fn preprocess_joins_important() {
        assert_eq!(preprocess("red ! important"), "red !important");
    }

    #[test]
    fn preprocess_leaves_strings_alone() {
        assert_eq!(
            preprocess("content: \". a # b - c ! d\""),
            "content: \". a # b - c ! d\""
        );
    }

    #[test]
    fn preprocess_escaped_quote_in_string() {
        assert_eq!(preprocess("\"a\\\"b . c\" . d"), "\"a\\\"b . c\" .d");
    }

    #[test]
    fn preprocess_single_quoted_string() {
        assert_eq!(preprocess("'a . b' . c"), "'a . b' .c");
    }

    #[test]
    fn preprocess_backslash_at_end_of_input() {
        // A `\` as the last character inside a string literal: there
        // is no escaped character left to copy.
        assert_eq!(preprocess("\"a\\"), "\"a\\");
    }

    // ── chunk_body ─────────────────────────────────────────────

    #[test]
    fn chunk_statements_and_blocks() {
        let chunks = chunk_body("color: red; .a { width: 8px; }");
        assert_eq!(chunks.len(), 2);
        assert!(matches!(&chunks[0], Chunk::Statement(s) if s == "color: red"));
        assert!(matches!(&chunks[1], Chunk::Block { prelude, .. } if prelude == ".a"));
    }

    #[test]
    fn chunk_semicolon_inside_parens_is_shielded() {
        let chunks = chunk_body("background: url(data:a;b);");
        assert_eq!(chunks.len(), 1);
    }

    #[test]
    fn chunk_brace_inside_string_is_shielded() {
        let chunks = chunk_body("content: \"{\"; color: red;");
        assert_eq!(chunks.len(), 2);
    }

    #[test]
    fn chunk_nested_blocks() {
        let chunks = chunk_body(".a { .b { color: red; } }");
        assert_eq!(chunks.len(), 1);
        assert!(matches!(&chunks[0], Chunk::Block { body, .. } if body.contains(".b")));
    }

    #[test]
    fn chunk_escaped_quote_before_close_brace() {
        // Backslash-escaped quote inside a string while scanning for
        // the matching `}`.
        let chunks = chunk_body(".a { content: \"x\\\"}y\"; }");
        assert_eq!(chunks.len(), 1);
    }

    #[test]
    fn chunk_empty_input() {
        assert!(chunk_body("  ").is_empty());
    }

    #[test]
    fn chunk_empty_statement_is_skipped() {
        // A stray `;;` yields an empty statement, which is dropped.
        let chunks = chunk_body("color: red;;");
        assert_eq!(chunks.len(), 1);
        assert!(matches!(&chunks[0], Chunk::Statement(s) if s == "color: red"));
    }

    #[test]
    #[should_panic(expected = "unbalanced `{`")]
    fn chunk_unbalanced_brace_panics() {
        chunk_body(".a { color: red;");
    }

    #[test]
    #[should_panic(expected = "expected `;` or `{ … }` after")]
    fn chunk_trailing_junk_panics() {
        chunk_body("color: red");
    }

    #[test]
    fn chunk_escaped_quote_inside_string() {
        let chunks = chunk_body("content: \"a\\\";b\"; color: red;");
        assert_eq!(chunks.len(), 2);
    }

    // ── selectors ──────────────────────────────────────────────

    #[test]
    fn selector_simple_forms() {
        assert!(render(".a { color: red; }").contains(".a"));
        assert!(render("#i { color: red; }").contains("#i"));
        assert!(render("div { color: red; }").contains("div"));
        assert!(render("* { color: red; }").contains('*'));
        assert!(render("& { color: red; }").contains('&'));
    }

    #[test]
    fn selector_pseudo_attach() {
        let css = render(".a:hover { color: red; }");
        assert!(css.contains(".a:hover"));
    }

    #[test]
    fn selector_pseudo_element() {
        let css = render(".a::before { color: red; }");
        assert!(css.contains(".a::before"));
    }

    #[test]
    fn selector_functional_pseudo() {
        let css = render(".a:nth-child(2n + 1) { color: red; }");
        assert!(css.contains(".a:nth-child(2n + 1)"));
    }

    #[test]
    fn selector_functional_pseudo_nested_parens() {
        let css = render(".a:has(:not(.b)) { color: red; }");
        assert!(css.contains(".a:has(:not(.b))"));
    }

    #[test]
    fn selector_combinators() {
        assert!(render(".a .b { color: red; }").contains(".a .b"));
        assert!(render(".a > .b { color: red; }").contains(".a > .b"));
        assert!(render(".a + .b { color: red; }").contains(".a + .b"));
        assert!(render(".a ~ .b { color: red; }").contains(".a ~ .b"));
    }

    #[test]
    fn selector_comma_list() {
        let css = render(".a, .b { color: red; }");
        assert!(css.contains(".a, .b"));
    }

    #[test]
    fn selector_quoted_raw() {
        let css = render("\"&.primary\" { color: red; }");
        assert!(css.contains("&.primary"));
    }

    #[test]
    fn selector_nesting_descendant() {
        let css = render("& .text { color: red; }");
        assert!(css.contains("& .text"));
    }

    #[test]
    fn selector_compound_type_class() {
        let css = render("a.btn { color: red; }");
        assert!(css.contains("a.btn"));
    }

    #[test]
    #[should_panic(expected = "dangling combinator")]
    fn selector_dangling_combinator_panics() {
        parse_stylesheet(".a > { color: red; }");
    }

    #[test]
    #[should_panic(expected = "empty selector")]
    fn selector_empty_after_comma_panics() {
        parse_stylesheet(".a, { color: red; }");
    }

    #[test]
    #[should_panic(expected = "rule with an empty selector")]
    fn selector_empty_prelude_panics() {
        parse_stylesheet("{ color: red; }");
    }

    #[test]
    #[should_panic(expected = "unexpected `[`")]
    fn selector_attribute_panics() {
        parse_stylesheet("[data-x] { color: red; }");
    }

    #[test]
    #[should_panic(expected = "expected an identifier")]
    fn selector_dot_without_ident_panics() {
        parse_stylesheet(".:hover { color: red; }");
    }

    #[test]
    #[should_panic(expected = "unbalanced `(` in selector")]
    fn selector_unbalanced_paren_panics() {
        // Unbalanced parens in a rule prelude are caught by chunking
        // first; exercise take_paren_args directly for its own guard.
        take_paren_args(&mut "2n".chars().peekable(), ".a:nth-child(2n");
    }

    // ── declarations ───────────────────────────────────────────

    #[test]
    fn declaration_typed_value() {
        assert!(render(".a { width: 8px; }").contains("width: 8px;"));
    }

    #[test]
    fn declaration_important() {
        assert!(render(".a { color: red !important; }").contains("color: rgb(255, 0, 0) !important;"));
    }

    #[test]
    fn declaration_quoted_value() {
        assert!(render(".a { font-family: \"My Font\"; }").contains("font-family: \"My Font\";"));
    }

    #[test]
    fn declaration_single_quoted_value() {
        assert!(render(".a { font-family: 'My Font'; }").contains("font-family: \"My Font\";"));
    }

    #[test]
    fn declaration_custom_property() {
        assert!(render(":root { --brand: rebeccapurple; }").contains("--brand: rgb(102, 51, 153);"));
    }

    #[test]
    fn declaration_var_reference() {
        assert!(render(".a { color: var(--brand); }").contains("color: var(--brand);"));
    }

    #[test]
    #[should_panic(expected = "invalid custom-property name `--`")]
    fn declaration_invalid_custom_property_panics() {
        parse_stylesheet(".a { --: red; }");
    }

    #[test]
    #[should_panic(expected = "expected a `name: value;` declaration")]
    fn declaration_missing_colon_panics() {
        parse_stylesheet(".a { color red; }");
    }

    #[test]
    #[should_panic(expected = "empty property name")]
    fn declaration_empty_name_panics() {
        parse_stylesheet(".a { : red; }");
    }

    #[test]
    #[should_panic(expected = "only allowed at the top level")]
    fn declaration_at_statement_inside_rule_panics() {
        parse_stylesheet(".a { @import \"x.css\"; }");
    }

    // ── at-rules ───────────────────────────────────────────────

    #[test]
    fn at_media_token_prelude() {
        let css = render("@media (max-width: 600px) { .a { color: red; } }");
        assert!(css.contains("@media (max-width: 600px)"));
        assert!(css.contains(".a"));
    }

    #[test]
    fn at_media_quoted_prelude() {
        let css = render("@media \"(min-width: 800px)\" { .a { color: red; } }");
        assert!(css.contains("@media (min-width: 800px)"));
    }

    #[test]
    #[should_panic(expected = "needs a prelude")]
    fn at_media_empty_prelude_panics() {
        parse_stylesheet("@media { .a { color: red; } }");
    }

    #[test]
    fn at_supports() {
        let css = render("@supports (display: grid) { .a { display: grid; } }");
        assert!(css.contains("@supports (display: grid)"));
    }

    #[test]
    fn at_container_unnamed() {
        let css = render("@container (min-width: 800px) { .a { color: red; } }");
        assert!(css.contains("@container (min-width: 800px)"));
    }

    #[test]
    fn at_container_named() {
        let css = render("@container card (inline-size > 30ch) { .a { color: red; } }");
        assert!(css.contains("@container card (inline-size > 30ch)"));
    }

    #[test]
    #[should_panic(expected = "@container needs a `(condition)` prelude")]
    fn at_container_missing_parens_panics() {
        parse_stylesheet("@container card { .a { color: red; } }");
    }

    #[test]
    fn at_scope_root_only() {
        let css = render("@scope (.card) { h1 { color: red; } }");
        assert!(css.contains("@scope (.card)"));
    }

    #[test]
    fn at_scope_limit_only() {
        let css = render("@scope to (.inner) { h1 { color: red; } }");
        assert!(css.contains("@scope to (.inner)"));
    }

    #[test]
    fn at_scope_root_and_limit() {
        let css = render("@scope (.card) to (.inner) { h1 { color: red; } }");
        assert!(css.contains("@scope (.card) to (.inner)"));
    }

    #[test]
    #[should_panic(expected = "expected `to (…)`")]
    fn at_scope_garbage_panics() {
        parse_stylesheet("@scope (.card) junk { h1 { color: red; } }");
    }

    #[test]
    #[should_panic(expected = "expected `( … )`")]
    fn strip_parens_panics() {
        strip_parens("nope", "@test");
    }

    #[test]
    fn at_layer_block_named() {
        let css = render("@layer base { .a { color: red; } }");
        assert!(css.contains("@layer base {"));
    }

    #[test]
    fn at_layer_block_unnamed() {
        let css = render("@layer { .a { color: red; } }");
        assert!(css.contains("@layer {"));
    }

    #[test]
    fn at_layer_statement() {
        assert!(render("@layer utilities;").contains("@layer utilities;"));
    }

    #[test]
    fn at_layer_bare_statement() {
        assert!(render("@layer;").contains("@layer;"));
    }

    #[test]
    fn at_keyframes_from_to() {
        let css = render("@keyframes fade { from { opacity: 0; } to { opacity: 1; } }");
        assert!(css.contains("@keyframes fade"));
        assert!(css.contains("from"));
        assert!(css.contains("to"));
    }

    #[test]
    fn at_keyframes_percentages() {
        let css = render("@keyframes bounce { 0% { opacity: 0; } 50%, 100% { opacity: 1; } }");
        assert!(css.contains("0%"));
        assert!(css.contains("50%, 100%"));
    }

    #[test]
    #[should_panic(expected = "@keyframes needs a name")]
    fn at_keyframes_no_name_panics() {
        parse_stylesheet("@keyframes { from { opacity: 0; } }");
    }

    #[test]
    #[should_panic(expected = "@keyframes stops look like")]
    fn at_keyframes_statement_panics() {
        parse_stylesheet("@keyframes fade { opacity: 0; }");
    }

    #[test]
    #[should_panic(expected = "@keyframes bodies only take declarations")]
    fn at_keyframes_nested_rule_panics() {
        parse_stylesheet("@keyframes fade { from { .a { color: red; } } }");
    }

    #[test]
    fn at_font_face() {
        let css = render("@font-face { font-family: \"My Font\"; src: url(\"font.woff2\"); }");
        assert!(css.contains("@font-face"));
        assert!(css.contains("font-family: \"My Font\";"));
        assert!(css.contains("src: url(\"font.woff2\");"));
    }

    #[test]
    fn at_page_plain() {
        assert!(render("@page { margin: 1cm; }").contains("@page {"));
    }

    #[test]
    fn at_page_pseudo() {
        assert!(render("@page :first { margin-top: 2cm; }").contains("@page :first"));
    }

    #[test]
    fn at_import_plain() {
        assert!(render("@import \"foo.css\";").contains("@import \"foo.css\";"));
    }

    #[test]
    fn at_import_with_media() {
        // `stringify!` drops the space before `(`, so the media query
        // arrives (and renders) as `and(...)`.
        let css = render("@import \"foo.css\" screen and(max-width: 600px);");
        assert!(css.contains("@import \"foo.css\" screen and(max-width: 600px);"));
    }

    #[test]
    fn at_import_with_supports() {
        let css = render("@import \"foo.css\" supports(display: flex);");
        assert!(css.contains("supports(display: flex)"));
    }

    #[test]
    fn at_import_with_supports_and_media() {
        let css = render("@import \"foo.css\" supports(display: flex) screen;");
        assert!(css.contains("supports(display: flex) screen;"));
    }

    #[test]
    fn at_import_url_function() {
        assert!(render("@import url(foo.css);").contains("@import \"foo.css\";"));
    }

    #[test]
    fn at_import_url_function_quoted() {
        assert!(render("@import url(\"foo.css\");").contains("@import \"foo.css\";"));
    }

    #[test]
    fn at_import_url_prefix_without_parens() {
        assert!(render("@import urlx;").contains("@import \"urlx\";"));
    }

    #[test]
    fn at_import_bare_url() {
        assert!(render("@import foo.css;").contains("@import \"foo.css\";"));
    }

    #[test]
    #[should_panic(expected = "@import needs a url")]
    fn at_import_no_url_panics() {
        parse_stylesheet("@import;");
    }

    #[test]
    #[should_panic(expected = "expected `supports(…)`")]
    fn at_import_bad_supports_panics() {
        parse_stylesheet("@import \"foo.css\" supports-flex;");
    }

    #[test]
    fn at_charset() {
        assert!(render("@charset \"utf-8\";").contains("@charset \"utf-8\";"));
    }

    #[test]
    fn at_charset_unquoted() {
        // An unquoted encoding falls back to the raw text.
        assert!(render("@charset utf-8;").contains("utf-8"));
    }

    #[test]
    fn at_namespace_url_only() {
        let css = render("@namespace \"http://www.w3.org/2000/svg\";");
        assert!(css.contains("@namespace \"http://www.w3.org/2000/svg\";"));
    }

    #[test]
    fn at_namespace_with_prefix() {
        let css = render("@namespace svg \"http://www.w3.org/2000/svg\";");
        assert!(css.contains("svg"));
    }

    #[test]
    #[should_panic(expected = "@namespace needs a url")]
    fn at_namespace_no_url_panics() {
        parse_stylesheet("@namespace;");
    }

    #[test]
    #[should_panic(expected = "unexpected")]
    fn at_namespace_extra_panics() {
        parse_stylesheet("@namespace svg \"u\" extra;");
    }

    #[test]
    #[should_panic(expected = "unsupported at-rule `@unknown`")]
    fn at_rule_unknown_block_panics() {
        parse_stylesheet("@unknown x { .a { color: red; } }");
    }

    #[test]
    #[should_panic(expected = "needs a `{ … }` block")]
    fn at_rule_unknown_statement_panics() {
        parse_stylesheet("@unknown x;");
    }

    #[test]
    #[should_panic(expected = "declarations need a rule around them")]
    fn top_level_declaration_panics() {
        parse_stylesheet("color: red;");
    }

    // ── nesting & end-to-end ───────────────────────────────────

    #[test]
    fn nested_rule_in_rule() {
        let css = render(".card { padding: 16px; & .text { font-weight: bold; } }");
        assert!(css.contains("padding: 16px;"));
        assert!(css.contains("& .text"));
        assert!(css.contains("font-weight: bold;"));
    }

    #[test]
    fn nested_at_rule_in_rule() {
        let css = render(".a { color: red; @media(max-width: 600px) { & { color: blue; } } }");
        assert!(css.contains("@media (max-width: 600px)"));
        assert!(css.contains("color: rgb(0, 0, 255);"));
    }

    #[test]
    fn deeply_nested_rules() {
        let css = render(".a { .b { .c { color: red; } } }");
        assert!(css.contains(".c"));
    }

    #[test]
    fn empty_stylesheet() {
        assert_eq!(render(""), "");
    }

    #[test]
    fn unquote_non_matching() {
        assert_eq!(unquote("\"mixed'"), None);
        assert_eq!(unquote("x"), None);
        assert_eq!(unquote("\"\""), Some(String::new()));
    }

    #[test]
    fn normalize_important_glued() {
        assert_eq!(normalize_important("red!important"), Cow::<str>::Owned("red !important".to_string()));
    }

    #[test]
    fn normalize_important_passthrough() {
        assert_eq!(normalize_important("red !important"), Cow::<str>::Borrowed("red !important"));
        assert_eq!(normalize_important("red"), Cow::<str>::Borrowed("red"));
        // Not at a word boundary — no normalization.
        assert_eq!(
            normalize_important("url(img!important.png)"),
            Cow::<str>::Borrowed("url(img!important.png)")
        );
    }

    #[test]
    fn split_first_token_basic() {
        assert_eq!(split_first_token("head tail"), ("head", "tail"));
        assert_eq!(split_first_token("only"), ("only", ""));
    }

    #[test]
    fn split_commas_quote_shielded() {
        let parts = split_top_level_commas("\"a,b\", .c");
        assert_eq!(parts, vec!["\"a,b\"".to_string(), ".c".to_string()]);
        let parts = split_top_level_commas("'a,b'");
        assert_eq!(parts, vec!["'a,b'".to_string()]);
    }
}
