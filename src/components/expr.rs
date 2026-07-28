//! Expression trees and the [`IntoExpr`] trait that supports them.
//!
//! An [`Expr`] is a tree node. The [`IntoExpr`] trait plus the
//! [`list!`] macro let you compose trees in literal style instead of
//! writing the nested `Box<Expr>` structure by hand.
//!
//! Most construction is done through macros: [`component!`] is the
//! entry point, [`switch!`](crate::switch) builds match expressions,
//! and [`text!`](crate::text) concatenates text. The only remaining
//! free function builder is [`prop`], used for runtime prop-name keys.
//!
//! [`component!`]: crate::component
//! [`prop`]: crate::components::prop

use std::borrow::Cow;

use crate::attributes::Attribute;
use crate::element::Element;
use crate::node::Node;

/// An attribute on a [`Expr::Wrap`] element, either static or dynamic.
///
/// - [`Static`](WrappedAttribute::Static) — a compile-time-known
///   [`Attribute`] produced by the [`ComponentElement`](super::ComponentElement)
///   base struct's `attr(key)` builder.
/// - [`Dynamic`](WrappedAttribute::Dynamic) — a runtime-evaluated
///   attribute produced by the typed wrapper methods (`div().class(...)`
///   etc.). The key is the attribute name and the `Expr` is evaluated
///   at render time; its text form becomes the attribute value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WrappedAttribute {
    /// A compile-time-known attribute.
    Static(Attribute),
    /// A runtime-evaluated attribute.
    Dynamic(Cow<'static, str>, Expr),
}

/// The expression tree of a [`Component`](super::Component).
///
/// Each variant defines what its rendered output looks like:
///
/// | Variant   | Output                                           |
/// |-----------|--------------------------------------------------|
/// | `Literal` | One `Node::Element` with the literal element.   |
/// | `Prop`    | One `Node::Text` with the prop's text form.     |
/// | `List`    | Concatenation of each sub-expression's output.  |
/// | `Match`   | The arm whose `value` matches the prop's string. |
/// | `Either`  | `then` if condition is true, else `otherwise`.   |
/// | `Maybe`   | `then` if condition is true, else empty.          |
/// | `Map`     | For each list item, scoped eval of `body`.        |
/// | `Wrap`    | A new element with `body` as children.           |
/// | `LiteralChildren` | Pre-evaluated nodes, returned as-is.     |
///
/// Lenient vs strict: `Prop` is **lenient** (missing or wrong-typed
/// values render as the empty string); `Match`, `Either`, `Maybe`,
/// and `Map` are **strict** (type mismatches return `RenderError`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// Render a literal element.
    Literal(Element),

    /// Substitute a Prop's value as a Text node.
    ///
    /// **Lenient**: missing or wrong type → Text("").
    Prop(Cow<'static, str>),

    /// Concatenate a sequence of expressions. Each is evaluated and the
    /// resulting `Vec<Node>` are concatenated.
    List(Vec<Box<Expr>>),

    /// Multi-way string match. Looks up `Prop[key]` as a String.
    /// The first arm whose `value` matches the prop's string form
    /// wins; falls back to `default` if none match.
    /// **Strict**: type mismatch on `key` → `RenderError`.
    Match {
        /// Prop key holding the string to match against.
        key: Cow<'static, str>,
        /// Candidate arms, in declaration order. First match wins.
        arms: Vec<MatchArm>,
        /// Expression evaluated when no arm matches.
        default: Box<Expr>,
    },

    /// Two-way conditional. Looks up `Prop[condition]` as a Bool.
    /// True → eval `then`; false → eval `otherwise`.
    /// `otherwise` is required (use [`Maybe`](Expr::Maybe) for optional).
    /// **Strict**: type mismatch on condition → `RenderError`.
    Either {
        /// Prop key holding the boolean condition.
        condition: Cow<'static, str>,
        /// Expression evaluated when the condition is `true`.
        then: Box<Expr>,
        /// Expression evaluated when the condition is `false`.
        otherwise: Box<Expr>,
    },

    /// One-way conditional. If `Prop[condition]` is `Bool(true)`,
    /// eval `then`. Else produce an empty `Vec<Node>`.
    /// **Strict**: type mismatch on condition → `RenderError`.
    Maybe {
        /// Prop key holding the boolean condition.
        condition: Cow<'static, str>,
        /// Expression evaluated when the condition is `true`.
        then: Box<Expr>,
    },

    /// Iteration. Looks up `Prop[input]` as a `List`.
    /// For each item, builds a scoped `Props` with the item bound to
    /// `Props["item"]` and the index bound to `Props["index"]`,
    /// then evaluates `body`. Concatenates the results.
    /// **Strict**: type mismatch on `input` → `RenderError`.
    Map {
        /// Prop key holding the input list.
        input: Cow<'static, str>,
        /// Body expression. Re-evaluated for each item with a scoped
        /// `Props` (the item as `Prop["item"]`, the index as
        /// `Prop["index"]`).
        body: Box<Expr>,
    },

    /// Introduce a new element. The body is a sequence of expressions
    /// whose eval results are concatenated as children.
    ///
    /// Produced by [`ComponentElement`](super::ComponentElement)
    /// (and the typed wrappers that wrap it) when used inside
    /// [`component!`](crate::component).
    Wrap {
        /// Tag name.
        name: Cow<'static, str>,
        /// Attributes on the wrapped element (static or dynamic).
        attrs: Vec<WrappedAttribute>,
        /// Child expressions. Their rendered outputs are concatenated
        /// as the element's children.
        body: Vec<Box<Expr>>,
    },

    /// A pre-evaluated list of nodes. Eval returns these as-is.
    LiteralChildren(Vec<Node>),
}

/// One arm of a [`Match`](Expr::Match) expression.
///
/// `value` is matched literally against the prop's string form.
/// `result` is the expression to evaluate when the arm matches.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchArm {
    /// String value to compare against `Prop[key]`.
    pub value: Cow<'static, str>,
    /// Expression to evaluate when this arm matches.
    pub result: Box<Expr>,
}

/// Implicit conversion into an [`Expr`].
///
/// Used by the [`list!`] macro, [`text!`](crate::text), and any other
/// macro that wants to accept mixed content (e.g. `Element`, `Expr`,
/// `Box<Expr>`, `Node`, `&str`, and typed wrappers).
pub trait IntoExpr {
    /// Consume `self` and produce an `Expr`.
    fn into_expr(self) -> Expr;
}

impl IntoExpr for Element {
    fn into_expr(self) -> Expr {
        Expr::Literal(self)
    }
}

impl IntoExpr for Expr {
    fn into_expr(self) -> Expr {
        self
    }
}

impl IntoExpr for Box<Expr> {
    fn into_expr(self) -> Expr {
        *self
    }
}

impl IntoExpr for Node {
    fn into_expr(self) -> Expr {
        Expr::LiteralChildren(vec![self])
    }
}

impl IntoExpr for &Node {
    fn into_expr(self) -> Expr {
        Expr::LiteralChildren(vec![self.clone()])
    }
}

impl IntoExpr for &str {
    fn into_expr(self) -> Expr {
        Expr::LiteralChildren(vec![Node::Text(Cow::Owned(self.to_string()))])
    }
}

/// Concatenate a sequence of items into a `List` [`Expr`].
///
/// Items can be:
/// - `Element` — wrapped as [`Expr::Literal`].
/// - `Expr` — taken as-is.
/// - `Box<Expr>` — unwrapped (via `IntoExpr`).
/// - `Node` — wrapped as a single-element [`Expr::LiteralChildren`].
/// - `&Node` — same, but cloning the node.
///
/// Mixed forms like
/// `list![el("h1"), prop("title"), Node::Text("static".into())]`
/// work because each item implements [`IntoExpr`].
///
/// `&'static str` and `String` items become `Node::Text` via the
/// `From` impls on `Node`.
#[macro_export]
macro_rules! list {
    ($($item:expr),* $(,)?) => {
        $crate::Expr::List(vec![
            $(Box::new(<_ as $crate::IntoExpr>::into_expr($item))),*
        ])
    };
}

/// `Expr::Prop(key)`.
///
/// Lenient at render time: a missing or wrong-typed prop renders as an
/// empty text node.
///
/// This free function is the escape hatch for runtime prop names:
///
/// ```ignore
/// let key = format!("dynamic-{}", some_var);
/// let expr = prop(key);
/// ```
pub fn prop(key: impl Into<Cow<'static, str>>) -> Expr {
    Expr::Prop(key.into())
}

// Use `super::Component` to break the cycle: `expr.rs` is included from
// `mod.rs`, so `Component` lives in `super`.
