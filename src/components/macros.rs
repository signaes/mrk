//! Top-level macros for the `components` module.
//!
//! - [`component!`] — entry point; produces a [`Component`] from a
//!   name and a brace-delimited body.
//! - [`switch!`] — declarative match syntax; produces an
//!   [`Expr::Match`].
//! - [`text!`] — text concatenation; produces an [`Expr::List`].
//!
//! [`Component`]: crate::components::Component
//! [`Expr::Match`]: crate::components::Expr::Match
//! [`Expr::List`]: crate::components::Expr::List

/// Build a [`Component`] from an identifier or string-literal name and
/// a brace-delimited body.
///
/// The body must be one expression that implements
/// [`IntoExpr`](crate::IntoExpr). Common forms are the typed wrappers
/// in [`crate::components::html`] and [`crate::components::svg`],
/// plus the bare [`ComponentElement`](crate::components::ComponentElement)
/// created by [`el`](crate::el).
///
/// # Example
///
/// ```ignore
/// component!(Card, {
///     div().class(prop("class")).children(nodes![
///         prop("title"),
///     ])
/// })
/// ```
#[macro_export]
macro_rules! component {
    ($name:ident, { $body:expr $(,)? }) => {
        $crate::Component {
            name: std::borrow::Cow::Borrowed(stringify!($name)),
            expr: <_ as $crate::IntoExpr>::into_expr($body),
        }
    };
    ($name:literal, { $body:expr $(,)? }) => {
        $crate::Component {
            name: std::borrow::Cow::Borrowed($name),
            expr: <_ as $crate::IntoExpr>::into_expr($body),
        }
    };
}

/// Build an [`Expr::Match`] declaratively.
///
/// The trailing `_ => expr` arm is the default. Each match arm's value
/// is a string literal compared against the prop's string form.
///
/// # Example
///
/// ```ignore
/// switch!("role", {
///     "admin" => div().class("admin"),
///     "user"  => div().class("user"),
///     _       => div().class("guest"),
/// })
/// ```
#[macro_export]
macro_rules! switch {
    ($key:expr, { $( $val:literal => $arm:expr ),+ , _ => $default:expr $(,)? }) => {{
        let __arms: Vec<$crate::MatchArm> = vec![
            $(
                $crate::MatchArm {
                    value: std::borrow::Cow::Borrowed($val),
                    result: Box::new(<_ as $crate::IntoExpr>::into_expr($arm)),
                }
            ),+
        ];
        $crate::Expr::Match {
            key: ($key).into(),
            arms: __arms,
            default: Box::new(<_ as $crate::IntoExpr>::into_expr($default)),
        }
    }};
    ($key:expr, { _ => $default:expr $(,)? }) => {{
        $crate::Expr::Match {
            key: ($key).into(),
            arms: Vec::new(),
            default: Box::new(<_ as $crate::IntoExpr>::into_expr($default)),
        }
    }};
}

/// Concatenate one or more sub-expressions into a text [`Expr::List`].
///
/// Each item must implement [`IntoExpr`](crate::IntoExpr).
///
/// # Example
///
/// ```ignore
/// text!(prop("first"), " ", prop("last"))
/// ```
#[macro_export]
macro_rules! text {
    ($($item:expr),* $(,)?) => {
        $crate::Expr::List(vec![
            $(Box::new(<_ as $crate::IntoExpr>::into_expr($item))),*
        ])
    };
}