//! The [`css!`] macro: CSS-like syntax compiled to the typed
//! [`StyleSheet`](crate::css::StyleSheet) builder API.
//!
//! The macro is a pure `macro_rules!` token muncher — no procedural
//! macros, no dependencies. It expands to `StyleSheetBuilder` /
//! `RuleBuilder` / `NestedBuilder` / `AtRule` calls.

/// Build a [`StyleSheet`](crate::css::StyleSheet) from CSS-like syntax.
///
/// Available when the `css` Cargo feature is enabled. Exported at the
/// crate root (and importable as `use mrk::css;`, which brings both the
/// `css` module and the `css!` macro into scope).
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
///         width: "8px";
///         &:hover { color: red; }
///         & .text { font-weight: bold; }
///         "&.primary" { color: green; }
///         @media "(min-width: 800px)" {
///             & { width: "100%"; }
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
/// assert!(css_text.contains("& .text"));
/// assert!(css_text.contains("@media (min-width: 800px)"));
/// ```
///
/// # Supported syntax
///
/// - Style rules with selectors: `.class`, `#id`, `type`, `*`, `&`,
///   `:pseudo-class`, `::pseudo-element`, descendant (juxtaposition),
///   child `>`, sibling `+` / `~`, and comma-separated lists
///   (`.a, .b { }`).
/// - Nested rules at any depth, with or without `&` (CSS nesting).
/// - `@media` and `@supports`, top-level and nested, with the prelude
///   as bare tokens (`(prefers-color-scheme: dark)`) or a quoted
///   string (`"(min-width: 800px)"`).
/// - Declarations as `name: value;`. Values may be bare token
///   sequences (`blue`, `bold`, `rgba(0,0,0)`, `#fff`, `1 / -1`,
///   `red !important`) or quoted strings / numbers (`"8px 16px"`,
///   `0`, `1.5`).
///
/// # Limitations (inherent to `macro_rules!`)
///
/// - **Dimensions and percentages must be quoted.** `8px`, `1.5rem`,
///   `50%` are invalid Rust literals and fail at lex time. Write
///   `width: "8px";` instead.
/// - **Whitespace is invisible to macros.** `.btn.primary` (compound)
///   and `.btn .primary` (descendant) look identical to the parser,
///   so juxtaposed classes/ids/types are always read as descendants
///   and `:pseudo` always attaches to the previous selector. Write
///   compound chains with a quoted selector: `"&.primary" { }` —
///   it is rendered verbatim (see
///   [`selector()`](crate::css::selector::selector)).
/// - **Quote values the tokenizer can't rejoin.** Leading-dash
///   idents (`"-apple-system"`) and `calc()` expressions should be
///   quoted; `#fff` and negative numbers (`-1`) work bare.
/// - Other at-rules (`@keyframes`, `@layer`, …) are not parsed; use
///   the fluent [`AtRule`](crate::css::AtRule) API for those.
/// - Very large sheets may need `#![recursion_limit = "256"]` (or
///   higher) at the crate root, since the token muncher uses one
///   expansion step per token.
#[macro_export]
macro_rules! css {
    // ── @sheet: top-level items (StyleSheetBuilder or AtRuleBuilder) ──
    (@sheet $b:tt,) => { $b .build() };
    (@sheet $b:tt, $($t:tt)+) => {
        $crate::css!(@scan sheet, $b, (), $($t)+)
    };

    // ── @body: rule-body items (RuleBuilder or NestedBuilder) ──
    (@body $b:tt,) => { $b };
    (@body $b:tt, $($t:tt)+) => {
        $crate::css!(@scan rule, $b, (), $($t)+)
    };

    // ── @cont: continue scanning after an emitted item ──
    (@cont sheet, $b:tt, $($rest:tt)*) => { $crate::css!(@sheet $b, $($rest)*) };
    (@cont rule, $b:tt, $($rest:tt)*) => { $crate::css!(@body $b, $($rest)*) };

    // ── @scan: accumulate tokens until an item boundary (`;` or `{…}`) ──
    (@scan $mode:ident, $b:tt, ($($acc:tt)*), { $($inner:tt)* } $($rest:tt)*) => {
        $crate::css!(@cont $mode,
            { $crate::css!(@block $mode, $b, ($($acc)*), { $($inner)* }) },
            $($rest)*)
    };
    (@scan $mode:ident, $b:tt, ($($acc:tt)*), ; $($rest:tt)*) => {
        $crate::css!(@cont $mode,
            { $crate::css!(@decl $b, (), $($acc)*) },
            $($rest)*)
    };
    (@scan $mode:ident, $b:tt, ($($acc:tt)*), $t:tt $($rest:tt)*) => {
        $crate::css!(@scan $mode, $b, ($($acc)* $t), $($rest)*)
    };

    // ── @block: selector / at-rule + `{ … }` ──
    // At-rules first: the generic selector arm would swallow them.
    (@block sheet, $b:tt, (@ media $q:literal), { $($inner:tt)* }) => {
        $b .at_rule($crate::css!(@sheet { $crate::css::AtRule::media($q) }, $($inner)*))
    };
    (@block sheet, $b:tt, (@ media $($q:tt)+), { $($inner:tt)* }) => {
        $b .at_rule($crate::css!(@sheet { $crate::css::AtRule::media(::std::stringify!($($q)+)) }, $($inner)*))
    };
    (@block sheet, $b:tt, (@ supports $q:literal), { $($inner:tt)* }) => {
        $b .at_rule($crate::css!(@sheet { $crate::css::AtRule::supports($q) }, $($inner)*))
    };
    (@block sheet, $b:tt, (@ supports $($q:tt)+), { $($inner:tt)* }) => {
        $b .at_rule($crate::css!(@sheet { $crate::css::AtRule::supports(::std::stringify!($($q)+)) }, $($inner)*))
    };
    (@block rule, $b:tt, (@ media $q:literal), { $($inner:tt)* }) => {
        $b .nest_at_rule($crate::css!(@sheet { $crate::css::AtRule::media($q) }, $($inner)*))
    };
    (@block rule, $b:tt, (@ media $($q:tt)+), { $($inner:tt)* }) => {
        $b .nest_at_rule($crate::css!(@sheet { $crate::css::AtRule::media(::std::stringify!($($q)+)) }, $($inner)*))
    };
    (@block rule, $b:tt, (@ supports $q:literal), { $($inner:tt)* }) => {
        $b .nest_at_rule($crate::css!(@sheet { $crate::css::AtRule::supports($q) }, $($inner)*))
    };
    (@block rule, $b:tt, (@ supports $($q:tt)+), { $($inner:tt)* }) => {
        $b .nest_at_rule($crate::css!(@sheet { $crate::css::AtRule::supports(::std::stringify!($($q)+)) }, $($inner)*))
    };
    // Quoted selector: verbatim escape hatch.
    (@block sheet, $b:tt, ($sel:literal), { $($inner:tt)* }) => {
        $b .rule(|__mrk_b| $crate::css!(@body
            { __mrk_b .selector($crate::css::selector::Selector::raw($sel)) },
            $($inner)*))
    };
    (@block rule, $b:tt, ($sel:literal), { $($inner:tt)* }) => {
        $b .nest(|__mrk_b| $crate::css!(@body
            { __mrk_b .selector($crate::css::selector::Selector::raw($sel)) },
            $($inner)*))
    };
    // Token selector.
    (@block sheet, $b:tt, ($($sel:tt)+), { $($inner:tt)* }) => {
        $b .rule(|__mrk_b| $crate::css!(@body
            { $crate::css!(@sel { __mrk_b }, (), (), (), $($sel)+) },
            $($inner)*))
    };
    (@block rule, $b:tt, ($($sel:tt)+), { $($inner:tt)* }) => {
        $b .nest(|__mrk_b| $crate::css!(@body
            { $crate::css!(@sel { __mrk_b }, (), (), (), $($sel)+) },
            $($inner)*))
    };

    // ── @decl: split `name : value` at the first `:` ──
    (@decl $b:tt, ($($name:tt)*), : $($value:tt)*) => {
        $crate::css!(@prop $b, ($($name)*), ($($value)*))
    };
    (@decl $b:tt, ($($name:tt)*), $t:tt $($rest:tt)*) => {
        $crate::css!(@decl $b, ($($name)* $t), $($rest)*)
    };

    // ── @prop: emit `.property(name, value)` ──
    (@prop $b:tt, ($($name:tt)*), ($v:literal)) => {
        $b .property(
            ::std::stringify!($($name)*).replace(' ', ""),
            $crate::css::Value::from($v),
        )
    };
    (@prop $b:tt, ($($name:tt)*), ($($v:tt)+)) => {
        $b .property(
            ::std::stringify!($($name)*).replace(' ', ""),
            ::std::stringify!($($v)+)
                .replace("# ", "#")
                .replace("- ", "-"),
        )
    };

    // ── @sel: selector list muncher ──
    // State: (chain) folded segments | (seg) open segment parts |
    // (comb) pending combinator.
    // End of selector list: flush and emit `.selector(...)`.
    (@sel $b:tt, $chain:tt, $seg:tt, $comb:tt,) => {
        $b .selector($crate::css!(@fold $chain, ( $crate::css!(@segexpr $seg) ), $comb))
    };
    // Comma: flush this selector, start the next.
    (@sel $b:tt, $chain:tt, $seg:tt, $comb:tt, , $($rest:tt)*) => {
        $crate::css!(@sel
            { $b .selector($crate::css!(@fold $chain, ( $crate::css!(@segexpr $seg) ), $comb)) },
            (), (), (), $($rest)*)
    };
    // Combinators: flush the open segment, remember the combinator.
    (@sel $b:tt, $chain:tt, $seg:tt, $comb:tt, > $($rest:tt)*) => {
        $crate::css!(@sel $b,
            ( $crate::css!(@fold $chain, ( $crate::css!(@segexpr $seg) ), $comb) ),
            (), (>), $($rest)*)
    };
    (@sel $b:tt, $chain:tt, $seg:tt, $comb:tt, + $($rest:tt)*) => {
        $crate::css!(@sel $b,
            ( $crate::css!(@fold $chain, ( $crate::css!(@segexpr $seg) ), $comb) ),
            (), (+), $($rest)*)
    };
    (@sel $b:tt, $chain:tt, $seg:tt, $comb:tt, ~ $($rest:tt)*) => {
        $crate::css!(@sel $b,
            ( $crate::css!(@fold $chain, ( $crate::css!(@segexpr $seg) ), $comb) ),
            (), (~), $($rest)*)
    };
    // Pseudo-elements / pseudo-classes attach to the open segment.
    (@sel $b:tt, $chain:tt, ($($seg:tt)*), $comb:tt, :: $p:ident $($rest:tt)*) => {
        $crate::css!(@sel $b, $chain,
            ( $($seg)* $crate::css::selector::Selector::pseudo_element(::std::stringify!($p)) , ),
            $comb, $($rest)*)
    };
    (@sel $b:tt, $chain:tt, ($($seg:tt)*), $comb:tt, : $p:ident $($rest:tt)*) => {
        $crate::css!(@sel $b, $chain,
            ( $($seg)* $crate::css::selector::Selector::pseudo_class(::std::stringify!($p)) , ),
            $comb, $($rest)*)
    };
    // Segment starters (`.x`, `#x`, `*`, `&`, `type`).
    (@sel $b:tt, $chain:tt, $seg:tt, $comb:tt, . $name:ident $($rest:tt)*) => {
        $crate::css!(@base $b, $chain, $seg, $comb,
            ( $crate::css::selector::Selector::class(::std::stringify!($name)) ),
            $($rest)*)
    };
    (@sel $b:tt, $chain:tt, $seg:tt, $comb:tt, # $name:ident $($rest:tt)*) => {
        $crate::css!(@base $b, $chain, $seg, $comb,
            ( $crate::css::selector::Selector::id(::std::stringify!($name)) ),
            $($rest)*)
    };
    (@sel $b:tt, $chain:tt, $seg:tt, $comb:tt, * $($rest:tt)*) => {
        $crate::css!(@base $b, $chain, $seg, $comb,
            ( $crate::css::selector::Selector::universal() ),
            $($rest)*)
    };
    (@sel $b:tt, $chain:tt, $seg:tt, $comb:tt, & $($rest:tt)*) => {
        $crate::css!(@base $b, $chain, $seg, $comb,
            ( $crate::css::selector::Selector::nesting_ref() ),
            $($rest)*)
    };
    (@sel $b:tt, $chain:tt, $seg:tt, $comb:tt, $name:ident $($rest:tt)*) => {
        $crate::css!(@base $b, $chain, $seg, $comb,
            ( $crate::css::selector::Selector::type_(::std::stringify!($name)) ),
            $($rest)*)
    };

    // ── @base: open a new segment, folding the previous one ──
    (@base $b:tt, $chain:tt, (), $comb:tt, ($($new:tt)*), $($rest:tt)*) => {
        $crate::css!(@sel $b, $chain, ( ($($new)*) , ), $comb, $($rest)*)
    };
    (@base $b:tt, $chain:tt, ($($seg:tt)+), $comb:tt, ($($new:tt)*), $($rest:tt)*) => {
        $crate::css!(@sel $b,
            $crate::css!(@fold $chain, ( $crate::css!(@segexpr ($($seg)+)) ), $comb),
            ( ($($new)*) , ), (), $($rest)*)
    };

    // ── @segexpr: collapse segment parts into one selector expr ──
    (@segexpr ($($p:tt)+)) => {
        ( $crate::css::selector::Selector::Compound(::std::vec![ $($p)+ ]) )
    };

    // ── @fold: join the chain and a segment with a combinator ──
    (@fold (), $se:tt, $comb:tt) => { $se };
    (@fold ($($c:tt)+), $se:tt, ()) => {
        ( $crate::css::selector::Selector::Descendant(
            ::std::boxed::Box::new($($c)+), ::std::boxed::Box::new($se)) )
    };
    (@fold ($($c:tt)+), $se:tt, (>)) => {
        ( $crate::css::selector::Selector::Child(
            ::std::boxed::Box::new($($c)+), ::std::boxed::Box::new($se)) )
    };
    (@fold ($($c:tt)+), $se:tt, (+)) => {
        ( $crate::css::selector::Selector::Sibling(
            ::std::boxed::Box::new($($c)+), ::std::boxed::Box::new($se)) )
    };
    (@fold ($($c:tt)+), $se:tt, (~)) => {
        ( $crate::css::selector::Selector::GeneralSibling(
            ::std::boxed::Box::new($($c)+), ::std::boxed::Box::new($se)) )
    };

    // ── Entry point (must stay last: internal rules take precedence) ──
    ($($t:tt)*) => {{
        $crate::css!(@sheet { $crate::css::StyleSheet::new() }, $($t)*)
    }};
}
