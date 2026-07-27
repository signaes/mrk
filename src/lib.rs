//! # mrk
//!
//! A minimal markup builder library for Rust.
//!
//! `mrk` provides a fluent, type-safe API for building structured markup
//! trees. Compose elements with [`el`], attach attributes with [`attr`],
//! and build children lists with the [`nodes!`] macro.
//!
//! ## Features
//!
//! `mrk` splits capability into opt-in Cargo features. By default the
//! crate provides only the data model (no rendering, no persistence):
//!
//! ```toml
//! [dependencies]
//! mrk = "0.6.1"            # data model only
//! ```
//!
//! ### Available features
//!
//! | Feature       | Pulls in                                         |
//! |---------------|--------------------------------------------------|
//! | *(default)*   | data model: `el`, `attr`, `Node`, `Element`     |
//! | `html`        | 114 HTML tag factories, void elements, escaping |
//! | `svg`         | 67 SVG 2 tag factories, presentation attrs      |
//! | `components`  | [`Component`] + [`Expr`] trees, [`Props`]        |
//! | `ir`          | `[`.mrk`][crate::ir] wire format codec (depends on `components`) |

//! Combine features freely:
//!
//! ```toml
//! [dependencies]
//! mrk = { version = "0.6.1", features = ["html", "svg", "ir"] }
//! ```

#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod attributes;
mod constants;
mod element;
mod macros;
mod node;
mod render_core;
mod renderable;

pub use attributes::{Attribute, AttributeType, attr};
pub use element::{Element, el};
pub use node::Node;
pub use renderable::{Renderable, render};

/// Templated component data model: [`Component`], [`Expr`], [`Props`],
/// and the render engine.
///
/// Opt-in via the `components` Cargo feature. Pulled in by `ir`.
///
/// See the [module documentation](self) for an overview.
#[cfg(feature = "components")]
pub mod components;

#[doc(inline)]
#[cfg(feature = "components")]
pub use components::{
    arm, component, either, list_expr, literal, map, match_on, maybe, prop, wrap, Component,
    Expr, IntoExpr, MatchArm, Number, NumberKind, PropType, Props, RenderError,
};

/// The `.mrk` wire format: encode/decode a [`Component`] to bytes or
/// UTF-8 strings.
///
/// Opt-in via the `ir` Cargo feature (which depends on `components`).
/// Provides [`Mrk`] (encode/decode), [`ParseError`], and
/// [`MAX_PAYLOAD`].
#[cfg(feature = "ir")]
pub mod ir;

#[doc(inline)]
#[cfg(feature = "ir")]
pub use ir::{MAX_PAYLOAD, Mrk, ParseError};

#[cfg(all(test, feature = "components"))]
mod components_tests {
    include!("components/tests.rs");
}

#[cfg(all(test, feature = "ir"))]
mod ir_tests {
    include!("ir/tests.rs");
}

#[cfg(feature = "html")]
pub mod html;

#[cfg(feature = "svg")]
pub mod svg;
