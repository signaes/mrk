//! The [`css!`] macro: CSS-like syntax compiled to the typed
//! [`StyleSheet`](crate::css::StyleSheet) API.
//!
//! The macro is intentionally thin: it `stringify!`s its tokens and
//! hands the text to a runtime parser
//! ([`parse_stylesheet`](crate::css::parse::parse_stylesheet)).
//! There is no token munching, so there is no recursion limit to
//! tune, no procedural macro, and no dependencies.

/// Build a [`StyleSheet`](crate::css::StyleSheet) from CSS-like syntax.
///
/// Available when the `css` Cargo feature is enabled. Exported at the
/// crate root (and importable as `use mrk::css;`, which brings both the
/// `css` module and the `css!` macro into scope).
///
/// Values are parsed into typed [`Value`](crate::css::Value)s:
/// dimensions (`8px`, `1.5rem`), percentages (`100%`), angles
/// (`45deg`), times, hex colors (`#fff`), color functions
/// (`rgb(255, 0, 0)`), the 148 named colors (`rebeccapurple`),
/// `url(…)`, `var(--name[, fallback])`, plain numbers, and
/// space-separated lists (`margin: 8px 16px`). Quoted strings pass
/// through as [`Value::String`](crate::css::Value). `!important` is
/// recognized. Malformed input panics with a message pointing at the
/// offending fragment.
///
/// # Example
///
/// ```
/// use mrk::{css, Renderable};
///
/// let sheet = css! {
///     .btn {
///         background-color: rgba(0,0,0);
///         color: blue;
///         width: 8px;
///         &:hover { color: red; }
///         & .text { font-weight: bold; }
///         "&.primary" { color: green; }
///         @media "(min-width: 800px)" {
///             & { width: 100%; }
///         }
///     }
///     @media (prefers-color-scheme: dark) {
///         .btn { color: white; }
///     }
/// };
///
/// let css_text = sheet.render();
/// assert!(css_text.contains(".btn"));
/// assert!(css_text.contains("background-color: rgba(0, 0, 0);"));
/// assert!(css_text.contains("color: rgb(0, 0, 255);"));
/// assert!(css_text.contains("width: 8px;"));
/// assert!(css_text.contains("&:hover"));
/// assert!(css_text.contains("& .text"));
/// assert!(css_text.contains("@media (min-width: 800px)"));
/// ```
///
/// # Supported syntax
///
/// - Style rules with selectors: `.class`, `#id`, `type`, `*`, `&`,
///   `:pseudo-class`, `::pseudo-element`, functional pseudo-classes
///   (`:nth-child(2n + 1)`), descendant (juxtaposition), child `>`,
///   sibling `+` / `~`, and comma-separated lists (`.a, .b { }`).
/// - Nested rules at any depth, with or without `&` (CSS nesting).
/// - Every at-rule the [`AtRule`](crate::css::AtRule) AST supports:
///   `@media`, `@supports`, `@container`, `@scope`, `@layer` (block
///   and statement forms), `@keyframes`, `@font-face`, `@page`,
///   `@import`, `@charset`, and `@namespace`. At-rule preludes may be
///   bare tokens or a quoted string.
/// - Custom properties: `--brand: rebeccapurple;` declares and
///   `var(--brand)` / `var(--brand, blue)` references them.
/// - Declarations as `name: value;` (see the value forms above).
///
/// # Limitations (inherent to `macro_rules!`)
///
/// - **Whitespace is invisible to macros.** `.btn.primary` (compound)
///   and `.btn .primary` (descendant) look identical to the parser,
///   so juxtaposed classes/ids/types are always read as descendants
///   and `:pseudo` always attaches to the previous selector. Write
///   compound chains with a quoted selector: `"&.primary" { }` —
///   it is rendered verbatim (see
///   [`selector()`](crate::css::selector::selector)).
/// - **`1.5em` and `1.5ex` fail at lex time.** Rust's lexer reads the
///   `e` as a float exponent. Write `1.5 em` (split) or quote the
///   value instead. Other glued dimensions (`8px`, `1.5rem`, `100%`,
///   `45deg`, `0.5s`) work as-is.
/// - **Attribute selectors and `calc()` need quoting.** `[data-x]`
///   is not part of the token grammar; `calc(100% - 8px)` loses the
///   operator spacing during tokenization. Quote such values
///   (`"[data-x]"`, `"calc(100% - 8px)"`) or use the typed
///   [`Selector`](crate::css::Selector) / [`Value`](crate::css::Value)
///   APIs.
#[macro_export]
macro_rules! css {
    ($($t:tt)*) => {
        $crate::css::parse::parse_stylesheet(::std::stringify!($($t)*))
    };
}
