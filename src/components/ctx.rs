//! Closure-based builder DSL for expression trees.
//!
//! This module provides [`ExprCtx`], a zero-sized context type whose
//! methods mirror the free-function constructors in
//! [`super::expr`] (`prop`, `literal`, `either`, etc.). Closures over
//! `ExprCtx` are consumed by [`super::Component::build`] — the closure
//! is run once at build time, producing a plain [`Expr`] tree with no
//! remaining closures, no `Fn` types, no opaque values.
//!
//! # Example
//!
//! ```
//! use mrk::*;
//! use mrk::components::ExprCtx;
//!
//! let card = Component::build("card", |ctx| ctx.wrap(
//!     Element::new("div").attrs(vec![attr("class").value("card")]),
//!     list![
//!         ctx.prop("title"),
//!         ctx.either(
//!             "is_admin",
//!             (ctx.prop("admin_tools"), ctx.prop("user_tools")),
//!         ),
//!     ],
//! ));
//! ```
//!
//! # `match_on` in the closure DSL
//!
//! [`ExprCtx::match_on`] takes a closure that receives an
//! [`Otherwise`] marker and returns a list of [`MatchEntry`]
//! values. The marker acts as a sentinel: passing it as the
//! first element of a tuple registers a default arm. The
//! default can appear at any position in the list.
//!
//! ```ignore
//! ctx.match_on("role", |otherwise| [
//!     ("admin",     ctx.literal(el("badge-admin"))),
//!     ("developer", ctx.literal(el("badge-dev"))),
//!     (otherwise,   ctx.literal(el("badge-guest"))),
//! ]);
//! ```

use std::borrow::Cow;

use crate::element::Element;

use super::expr::{Expr, WrappedAttribute};
use super::{Component, MatchArm};

// =====================================================================
// Otherwise marker
// =====================================================================

/// Zero-sized marker type used as the first element of a tuple to
/// register a default arm in [`ExprCtx::match_on`].
///
/// Obtain the marker from the closure parameter passed to
/// [`ExprCtx::match_on`]. It cannot be constructed elsewhere.
///
/// # Example
///
/// ```ignore
/// ctx.match_on("x", |otherwise| [
///     ("a", ctx.prop("x")),
///     (otherwise, ctx.prop("fallback")),
/// ]);
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Otherwise;

// =====================================================================
// MatchEntry
// =====================================================================

/// One entry in a [`ExprCtx::match_on`] closure's return list.
///
/// `Arm(Cow<'static, str>, Expr)` represents a value → expression
/// pair. `Default(Expr)` represents the fallback arm.
///
/// Construct via the [`From`] impls: any `(V, E)` pair where
/// `V: Into<Cow<'static, str>>` and `E: Into<Expr>` produces an
/// arm, and `(Otherwise, E)` produces the default.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchEntry {
    /// A value → expression arm.
    Arm(Cow<'static, str>, Expr),
    /// The fallback arm (evaluated when no `Arm` matches).
    Default(Expr),
}

impl<V, E> From<(V, E)> for MatchEntry
where
    V: Into<Cow<'static, str>>,
    E: Into<Expr>,
{
    fn from((v, e): (V, E)) -> Self {
        MatchEntry::Arm(v.into(), e.into())
    }
}

impl<E: Into<Expr>> From<(Otherwise, E)> for MatchEntry {
    fn from((_, e): (Otherwise, E)) -> Self {
        MatchEntry::Default(e.into())
    }
}

impl<E: Into<Expr>> From<(&Otherwise, E)> for MatchEntry {
    fn from((_, e): (&Otherwise, E)) -> Self {
        MatchEntry::Default(e.into())
    }
}

// =====================================================================
// ExprCtx
// =====================================================================

/// Zero-sized, `Copy` context type for building [`Expr`] trees
/// inside a closure.
///
/// Every method on `ExprCtx` produces a value (`Expr`,
/// [`Component`], or [`Element`]) — the context itself is never
/// mutated and never escapes the closure. The closure is run once
/// at build time by [`Component::build`]; the resulting tree is
/// plain data with no remaining closures.
#[derive(Debug, Default, Clone, Copy)]
pub struct ExprCtx;

impl ExprCtx {
    /// Create a new context. Equivalent to `ExprCtx`.
    pub fn new() -> Self {
        ExprCtx
    }

    /// `Expr::Prop(key)`.
    pub fn prop(&self, key: impl Into<Cow<'static, str>>) -> Expr {
        Expr::Prop(key.into())
    }

    /// `Expr::Literal(el)`.
    pub fn literal(&self, el: impl Into<Element>) -> Expr {
        Expr::Literal(el.into())
    }

    /// `Expr::Either { condition, then: bodies.0, otherwise: bodies.1 }`.
    pub fn either(
        &self,
        key: impl Into<Cow<'static, str>>,
        bodies: (impl Into<Expr>, impl Into<Expr>),
    ) -> Expr {
        Expr::Either {
            condition: key.into(),
            then: Box::new(bodies.0.into()),
            otherwise: Box::new(bodies.1.into()),
        }
    }

    /// `Expr::Maybe { condition, then }`.
    pub fn maybe(
        &self,
        key: impl Into<Cow<'static, str>>,
        body: impl Into<Expr>,
    ) -> Expr {
        Expr::Maybe {
            condition: key.into(),
            then: Box::new(body.into()),
        }
    }

    /// `Expr::Map { input, body }`.
    pub fn map(
        &self,
        input: impl Into<Cow<'static, str>>,
        body: impl Into<Expr>,
    ) -> Expr {
        Expr::Map {
            input: input.into(),
            body: Box::new(body.into()),
        }
    }

    /// Build a multi-way string-match expression.
    ///
    /// The `arms` closure receives an [`Otherwise`] marker and must
    /// return something that converts into a `Vec<MatchEntry>`.
    /// Tuples `(V, E)` become value arms; `(Otherwise, E)` becomes
    /// the default arm. The default can appear at any position.
    ///
    /// # Panics
    ///
    /// Panics if zero or more than one default arm is provided.
    ///
    /// # Example
    ///
    /// ```ignore
    /// ctx.match_on("role", |otherwise| [
    ///     ("admin", ctx.literal(el("badge"))),
    ///     (otherwise, ctx.prop("fallback")),
    /// ]);
    /// ```
    pub fn match_on<F, I>(
        &self,
        key: impl Into<Cow<'static, str>>,
        arms: F,
    ) -> Expr
    where
        F: FnOnce(&Otherwise) -> I,
        I: Into<Vec<MatchEntry>>,
    {
        let otherwise = Otherwise;
        let entries: Vec<MatchEntry> = arms(&otherwise).into();

        let mut default_count = 0u8;
        for entry in &entries {
            if matches!(entry, MatchEntry::Default(_)) {
                default_count += 1;
            }
        }
        assert!(
            default_count == 1,
            "match_on requires exactly one default arm (found {default_count})"
        );

        let mut regular_arms = Vec::new();
        let mut default = None;
        for entry in entries {
            match entry {
                MatchEntry::Arm(value, result) => {
                    regular_arms.push(MatchArm {
                        value,
                        result: Box::new(result),
                    });
                }
                MatchEntry::Default(expr) => {
                    default = Some(Box::new(expr));
                }
            }
        }

        Expr::Match {
            key: key.into(),
            arms: regular_arms,
            default: default.expect("validated above"),
        }
    }

    /// `Expr::Wrap { name = el.name, attrs = [Static(a) for a in el.attrs], body = [el.children..., body] }`.
    pub fn wrap(
        &self,
        el: Element,
        body: impl Into<Expr>,
    ) -> Expr {
        let mut body_exprs: Vec<Box<Expr>> = Vec::new();
        if !el.children.is_empty() {
            body_exprs.push(Box::new(Expr::LiteralChildren(el.children)));
        }
        body_exprs.push(Box::new(body.into()));
        Expr::Wrap {
            name: el.name,
            attrs: el.attributes.into_iter().map(WrappedAttribute::Static).collect(),
            body: body_exprs,
        }
    }

    /// Build a [`Component`] from a nested closure.
    ///
    /// The closure is run once; the resulting [`Expr`] tree is stored
    /// on the returned `Component`. Equivalent to
    /// [`Component::build`].
    pub fn component(
        &self,
        name: impl Into<Cow<'static, str>>,
        body: impl Into<Expr>,
    ) -> Component {
        Component {
            name: name.into(),
            expr: body.into(),
        }
    }
}
