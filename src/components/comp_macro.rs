//! Declarative component template macros: [`comp!`] and [`text!`].
//!
//! These macros produce [`Expr`] values suitable for use inside the
//! closure DSL ([`Component::build`]) or as standalone expressions.
//!
//! # `comp!`
//!
//! Build an [`Expr::Wrap`] with optional attributes and children.
//! Attributes use `key=value` syntax; children are separated by commas
//! inside `[...]`. A single child without brackets is also accepted.
//!
//! ```ignore
//! comp!(div, class="card", id="main", [
//!     comp!(h1, { prop("title") }),
//!     comp!(p, { prop("body") }),
//! ])
//! ```
//!
//! # `text!`
//!
//! Concatenate one or more sub-expressions into a text [`Expr::List`].
//!
//! ```ignore
//! text!(prop("greeting"), ", ", prop("name"))
//! ```

/// Concatenate one or more sub-expressions into a text [`Expr::List`].
///
/// Accepts any number of comma-separated expressions that implement
/// [`IntoExpr`](crate::IntoExpr).
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

/// Build an [`Expr::Wrap`] with optional attributes and children.
///
/// All attributes are encoded as [`WrappedAttribute::Dynamic`], meaning
/// they are evaluated at render time. Use `key="literal"` for static
/// string values and `key=prop("...")` for dynamic values.
///
/// # Syntax
///
/// ```text
/// comp!(tag)
/// comp!(tag, { single_child })
/// comp!(tag, [child1, child2, ...])
/// comp!(tag, key=value, { single_child })
/// comp!(tag, key1=val1, key2=val2, [child1, child2])
/// ```
///
/// # Examples
///
/// ```ignore
/// // Bare element, no attrs or children
/// comp!(br)
///
/// // With a single child
/// comp!(h1, { prop("title") })
///
/// // With attributes and children
/// comp!(div, class="card", id="main", [
///     comp!(h1, { prop("title") }),
///     comp!(p, { prop("body") }),
/// ])
/// ```
#[macro_export]
macro_rules! comp {
    // comp!(tag)
    ($tag:ident) => {
        $crate::Expr::Wrap {
            name: std::borrow::Cow::Borrowed(stringify!($tag)),
            attrs: Vec::new(),
            body: Vec::new(),
        }
    };

    // comp!(tag, { single_child })
    ($tag:ident, { $child:expr }) => {
        $crate::Expr::Wrap {
            name: std::borrow::Cow::Borrowed(stringify!($tag)),
            attrs: Vec::new(),
            body: vec![Box::new(<_ as $crate::IntoExpr>::into_expr($child))],
        }
    };

    // comp!(tag, [child1, child2, ...])
    ($tag:ident, [ $($child:expr),* $(,)? ]) => {
        $crate::Expr::Wrap {
            name: std::borrow::Cow::Borrowed(stringify!($tag)),
            attrs: Vec::new(),
            body: vec![
                $(Box::new(<_ as $crate::IntoExpr>::into_expr($child))),*
            ],
        }
    };

    // comp!(tag, key=value, ..., { single_child })
    ($tag:ident, $( $key:ident = $val:expr ),+, { $child:expr }) => {
        $crate::Expr::Wrap {
            name: std::borrow::Cow::Borrowed(stringify!($tag)),
            attrs: vec![
                $(
                    $crate::WrappedAttribute::Dynamic(
                        std::borrow::Cow::Borrowed(stringify!($key)),
                        <_ as $crate::IntoExpr>::into_expr($val),
                    )
                ),+
            ],
            body: vec![Box::new(<_ as $crate::IntoExpr>::into_expr($child))],
        }
    };

    // comp!(tag, key=value, ..., [child1, child2, ...])
    ($tag:ident, $( $key:ident = $val:expr ),+, [ $($child:expr),* $(,)? ]) => {
        $crate::Expr::Wrap {
            name: std::borrow::Cow::Borrowed(stringify!($tag)),
            attrs: vec![
                $(
                    $crate::WrappedAttribute::Dynamic(
                        std::borrow::Cow::Borrowed(stringify!($key)),
                        <_ as $crate::IntoExpr>::into_expr($val),
                    )
                ),+
            ],
            body: vec![
                $(Box::new(<_ as $crate::IntoExpr>::into_expr($child))),*
            ],
        }
    };
}
