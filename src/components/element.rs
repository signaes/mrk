//! The base [`ComponentElement`] struct and [`ComponentAttribute`] enum.
//!
//! [`ComponentElement`] is the foundation of component templates in the
//! `components` module. The typed wrappers (e.g. `Div`, `Span`) wrap a
//! [`ComponentElement`] and add specific attribute setters. The base
//! type itself exposes only the generic operations needed by the
//! internals:
//!
//! - [`new`](ComponentElement::new) — construct from a tag name
//! - [`attr`](ComponentElement::attr) — push a boolean attribute
//! - [`attr_dynamic`](ComponentElement::attr_dynamic) — push a
//!   runtime-evaluated attribute
//! - [`children`](ComponentElement::children) — set the children list
//!
//! [`ComponentElement`] implements [`IntoExpr`], producing an
//! [`Expr::Wrap`](crate::components::Expr::Wrap).

use std::borrow::Cow;

use crate::attributes::Attribute;
use crate::components::{Expr, IntoExpr, WrappedAttribute};
use crate::node::Node;

/// An attribute on a [`ComponentElement`], either static or dynamic.
///
/// - [`Static`](ComponentAttribute::Static) — a compile-time-known
///   [`Attribute`] (e.g. `attr("enabled").value("true")`).
/// - [`Dynamic`](ComponentAttribute::Dynamic) — a runtime-evaluated
///   attribute. The key is the attribute name and the `Expr` is
///   evaluated at render time; its text form becomes the value.
///
/// Typed wrappers (e.g. `Div::class(impl IntoExpr)`) push a
/// `Dynamic` attribute; the generic `attr(key)` method pushes a
/// `Static` boolean attribute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentAttribute {
    /// A compile-time-known attribute.
    Static(Attribute),
    /// A runtime-evaluated attribute.
    Dynamic {
        /// Attribute name (e.g. `"class"`).
        key: Cow<'static, str>,
        /// Expression evaluated at render time to produce the value.
        value: Expr,
    },
}

/// A template element: tag name + attributes + children.
///
/// Built up by typed wrappers (e.g. `Div::class(...)`),
/// `ComponentElement` is the underlying struct they all wrap. It
/// exposes only the generic operations; tag-specific attribute
/// methods live on the typed wrappers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentElement {
    /// Tag name (e.g. `"div"`, `"custom-element"`).
    pub name: Cow<'static, str>,
    /// Attributes in source order.
    pub attributes: Vec<ComponentAttribute>,
    /// Child nodes (text, raw HTML, or nested elements).
    pub children: Vec<Node>,
}

impl ComponentElement {
    /// Construct an empty element with `name` and no attributes or
    /// children.
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        ComponentElement {
            name: name.into(),
            attributes: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Push a boolean attribute (no value) onto the element.
    ///
    /// Renders as `<tag key>`. For a key-value attribute, use the
    /// typed wrapper's specific method (e.g. `Div::class`) or
    /// [`attr_dynamic`](ComponentElement::attr_dynamic).
    pub fn attr(self, key: impl Into<Cow<'static, str>>) -> Self {
        let key = key.into();
        ComponentElement {
            attributes: {
                let mut attrs = self.attributes;
                attrs.push(ComponentAttribute::Static(Attribute {
                    key: key.clone(),
                    attr: crate::attributes::AttributeType::Bool(key),
                }));
                attrs
            },
            ..self
        }
    }

    /// Push a runtime-evaluated attribute.
    ///
    /// `value` must implement [`IntoExpr`]; its `Expr` form is
    /// stored and rendered at render time.
    pub fn attr_dynamic(
        self,
        key: impl Into<Cow<'static, str>>,
        value: impl IntoExpr,
    ) -> Self {
        let key = key.into();
        let value = value.into_expr();
        ComponentElement {
            attributes: {
                let mut attrs = self.attributes;
                attrs.push(ComponentAttribute::Dynamic { key, value });
                attrs
            },
            ..self
        }
    }

    /// Replace the element's children.
    pub fn children(self, children: Vec<Node>) -> Self {
        ComponentElement { children, ..self }
    }
}

impl IntoExpr for ComponentElement {
    fn into_expr(self) -> Expr {
        let attrs = self
            .attributes
            .into_iter()
            .map(|a| match a {
                ComponentAttribute::Static(a) => WrappedAttribute::Static(a),
                ComponentAttribute::Dynamic { key, value } => {
                    WrappedAttribute::Dynamic(key, value)
                }
            })
            .collect();
        let body = self
            .children
            .into_iter()
            .map(|n| Box::new(Expr::LiteralChildren(vec![n])))
            .collect();
        Expr::Wrap {
            name: self.name,
            attrs,
            body,
        }
    }
}

/// Internal macro: define a typed wrapper for a single HTML tag.
///
/// Each typed wrapper is a struct that holds a `ComponentElement` and
/// exposes tag-specific attribute setter methods. Each setter pushes
/// a `ComponentAttribute::Dynamic` onto the inner element.
///
/// Generated by `define_component_tag!` in `src/components/html/mod.rs`
/// and `src/components/svg/mod.rs`.
#[macro_export]
#[doc(hidden)]
#[allow(missing_docs)]
macro_rules! __define_component_wrapper {
    (
        $struct_name:ident,
        $factory:ident,
        $tag:literal,
        $( $method:ident => $attr_name:literal ),* $(,)?
    ) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $struct_name(pub $crate::components::ComponentElement);

        impl $struct_name {
            /// Construct a new empty element of this tag.
            pub fn new() -> Self {
                Self($crate::components::ComponentElement::new($tag))
            }

            /// Replace the element's children.
            pub fn children(self, children: Vec<$crate::node::Node>) -> Self {
                Self(self.0.children(children))
            }

            $(
                /// Push a runtime-evaluated attribute.
                pub fn $method(self, value: impl $crate::IntoExpr) -> Self {
                    Self(self.0.attr_dynamic($attr_name, value))
                }
            )*
        }

        impl $crate::components::IntoExpr for $struct_name {
            fn into_expr(self) -> $crate::components::Expr {
                self.0.into_expr()
            }
        }

        /// Factory: create a new empty element of this tag.
        pub fn $factory() -> $struct_name {
            $struct_name::new()
        }

        impl From<$struct_name> for $crate::Node {
            fn from(w: $struct_name) -> Self {
                $crate::Node::Expr(w.into_expr())
            }
        }
    };
}

pub(crate) use __define_component_wrapper;