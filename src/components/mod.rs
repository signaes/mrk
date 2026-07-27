//! Templated component data model.
//!
//! A [`Component`] is a named expression tree. Rendering it with a
//! [`Props`] value produces a list of [`Node`]s. The whole thing is
//! data — there are no closures, no `Fn` types, no opaque values. The
//! `.mrk` wire format encodes a `Component` for persistence and
//! exchange.
//!
//! # Feature flag
//!
//! This module is gated behind the `components` Cargo feature
//! (off by default). It also transitively pulls in any [`crate::ir`]
//! types via the `ir` feature, which depends on `components`.
//!
//! # Example
//!
//! ```
//! use mrk::*;
//!
//! let card = component(
//!     "card",
//!     wrap(
//!         Element::new("div").attrs(vec![attr("class").value("card")]),
//!         list![
//!             prop("title"),
//!             either("is_admin", prop("admin_tools"), prop("user_tools")),
//!         ],
//!     ),
//! );
//! ```
//!
//! # Architecture
//!
//! - [`Component`] — the tree's name and root expression.
//! - [`Props`] — typed input values keyed by `Cow<'static, str>`.
//! - [`PropType`], [`Number`], [`NumberKind`] — the value types that
//!   make up a `Props` bag.
//! - [`Expr`], [`MatchArm`] — the expression-tree node types.
//! - [`IntoExpr`] + the [`list!`] macro — ergonomic `Expr` builders.
//! - Constructor helpers ([`literal`], [`prop`], [`list_expr`],
//!   [`either`], [`maybe`], [`map`], [`match_on`], [`arm`], [`wrap`])
//!   — short-named builders that avoid `Expr::…` ceremony at call sites.
//! - [`Component::render`] + [`RenderError`] — turning a tree +
//!   `Props` into a `Vec<Node>`.
//!
//! See [`crate::ir`] for the wire-format codec that round-trips a
//! [`Component`] and back.

mod expr;
mod props;

use std::borrow::Cow;
use std::fmt;

use crate::element::Element;
use crate::node::Node;

pub use expr::{arm, component, either, list_expr, literal, map, match_on, maybe, prop, wrap};
pub use props::{Number, NumberKind, PropType, Props};

// Re-export the expression-tree public API at the crate root.
// Done here (rather than via `pub use expr::…`) so the doc-pointers
// resolve against the crate-root level.
pub use expr::{Expr, IntoExpr, MatchArm};

// =====================================================================
// Component
// =====================================================================

/// A serializable [`Component`]: a name plus an expression tree.
///
/// Render with [`Component::render`] + [`Props`] to produce a
/// `Vec<Node>`. Encoded/decoded through the `.mrk` wire format (see
/// [`crate::ir::Mrk`]) — there are no closures, no `Fn` types, no
/// opaque values. The whole tree is data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Component {
    /// Identifier used as the root of the `.mrk` representation.
    pub name: Cow<'static, str>,
    /// Root expression.
    pub expr: Expr,
}

impl Component {
    /// Render this `Component` against the given [`Props`].
    ///
    /// Returns `Err([`RenderError`])` if a required prop has the wrong
    /// type or is missing for an `Expr` variant that requires strict
    /// typing (`Match`, `Either`, `Maybe`, `Map`).
    pub fn render(&self, props: &Props) -> Result<Vec<Node>, RenderError> {
        render_expr(&self.expr, props)
    }
}

// =====================================================================
// RenderError
// =====================================================================

/// Errors produced by [`Component::render`] when a required prop has
/// the wrong type or is missing.
///
/// The wire format (see [`crate::ir`]) is type-strict for `Match`,
/// `Either`, `Maybe`, and `Map`. `Expr::Prop` is lenient and renders
/// missing/wrong-typed props as an empty string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenderError {
    /// A prop key was expected to be a specific type (`expected`) but
    /// the supplied `Props` had it as a different type (`found`), or
    /// the key was missing entirely (`found == "<missing>"`).
    TypeMismatch {
        /// The prop key that triggered the failure.
        key: Cow<'static, str>,
        /// The type the renderer expected (`"bool"`, `"string"`, `"list"`).
        expected: &'static str,
        /// What was actually found: the actual type name, or
        /// `<missing>` if the key wasn't in `Props` at all.
        found: Cow<'static, str>,
    },
}

impl fmt::Display for RenderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderError::TypeMismatch { key, expected, found } => write!(
                f,
                "prop `{key}` expected {expected}, found {found}"
            ),
        }
    }
}

impl std::error::Error for RenderError {}

// =====================================================================
// Render engine
// =====================================================================

/// Evaluate an [`Expr`] against [`Props`] and collect its output
/// `Vec<Node>`. Recursive; this is the main entry point of the
/// render engine.
fn render_expr(expr: &Expr, props: &Props) -> Result<Vec<Node>, RenderError> {
    match expr {
        Expr::Literal(el) => Ok(vec![Node::Element(el.clone())]),
        Expr::Prop(key) => Ok(vec![Node::Text(prop_to_text_value(props, key))]),
        Expr::List(items) => {
            let mut out = Vec::new();
            for c in items {
                out.extend(render_expr(c, props)?);
            }
            Ok(out)
        }
        Expr::Match { key, arms, default } => {
            let value = match props.get(key) {
                Some(PropType::String(s)) => s.clone(),
                Some(other) => {
                    return Err(RenderError::TypeMismatch {
                        key: key.clone(),
                        expected: "string",
                        found: Cow::Owned(other.type_name().to_string()),
                    });
                }
                None => {
                    return Err(RenderError::TypeMismatch {
                        key: key.clone(),
                        expected: "string",
                        found: Cow::Borrowed("<missing>"),
                    });
                }
            };
            for arm in arms {
                if arm.value == value {
                    return render_expr(&arm.result, props);
                }
            }
            render_expr(default, props)
        }
        Expr::Either { condition, then, otherwise } => {
            let truthy = require_bool(props, condition, "either")?;
            if truthy {
                render_expr(then, props)
            } else {
                render_expr(otherwise, props)
            }
        }
        Expr::Maybe { condition, then } => {
            let truthy = require_bool(props, condition, "maybe")?;
            if truthy {
                render_expr(then, props)
            } else {
                Ok(Vec::new())
            }
        }
        Expr::Map { input, body } => {
            let items = require_list(props, input, "map")?;
            let mut out = Vec::new();
            for (index, item) in items.iter().enumerate() {
                let mut scoped = props.0.clone();
                scoped.insert(Cow::Borrowed("item"), item.clone());
                scoped.insert(
                    Cow::Borrowed("index"),
                    PropType::Number(Number::int(index.to_string())),
                );
                let item_props = Props(scoped);
                out.extend(render_expr(body, &item_props)?);
            }
            Ok(out)
        }
        Expr::Wrap { name, attrs, body } => {
            let mut children = Vec::new();
            for c in body {
                children.extend(render_expr(c, props)?);
            }
            Ok(vec![Node::Element(
                Element {
                    name: name.clone(),
                    attributes: attrs.clone(),
                    children,
                },
            )])
        }
        Expr::LiteralChildren(nodes) => Ok(nodes.clone()),
    }
}

/// Lenient [`Expr::Prop`] substitution: missing key → `""`.
fn prop_to_text_value(props: &Props, key: &str) -> Cow<'static, str> {
    match props.get(key) {
        Some(p) => p.to_text(),
        None => Cow::Borrowed(""),
    }
}

/// Strict [`Expr::Either`] / [`Expr::Maybe`] boolean lookup.
fn require_bool(
    props: &Props,
    key: &str,
    ctx: &str,
) -> Result<bool, RenderError> {
    match props.get(key) {
        Some(PropType::Bool(b)) => Ok(*b),
        Some(other) => Err(RenderError::TypeMismatch {
            key: Cow::Owned(key.to_string()),
            expected: "bool",
            found: Cow::Owned(other.type_name().to_string()),
        }),
        None => Err(RenderError::TypeMismatch {
            key: Cow::Owned(key.to_string()),
            expected: "bool",
            found: Cow::Borrowed("<missing>"),
        }),
    }
    .inspect_err(|e| {
        // `ctx` is reserved for a future error-context annotation.
        let _ = ctx;
        let _ = e;
    })
}

/// Strict [`Expr::Map`] list lookup.
fn require_list<'a>(
    props: &'a Props,
    key: &str,
    ctx: &str,
) -> Result<&'a Vec<PropType>, RenderError> {
    let _ = ctx;
    match props.get(key) {
        Some(PropType::List(items)) => Ok(items),
        Some(other) => Err(RenderError::TypeMismatch {
            key: Cow::Owned(key.to_string()),
            expected: "list",
            found: Cow::Owned(other.type_name().to_string()),
        }),
        None => Err(RenderError::TypeMismatch {
            key: Cow::Owned(key.to_string()),
            expected: "list",
            found: Cow::Borrowed("<missing>"),
        }),
    }
}

// Re-export the macro crate-wide.
pub use crate::list;
