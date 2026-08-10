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
//! crate provides the data model with built-in rendering (no persistence):
//!
//! ```toml
//! [dependencies]
//! mrk = "0.10.0"            # data model + rendering
//! ```
//!
//! ### Available features
//!
//! | Feature       | Pulls in                                         |
//! |---------------|--------------------------------------------------|
//! | *(default)*   | data model + rendering: `el`, `attr`, `Node`, `Element`, `Renderable` |
//! | `html`        | 114 HTML tag factories, `html!` macro, void elements, escaping |
//! | `svg`         | 67 SVG 2 tag factories, `svg!` macro             |
//!
//! For templated components see the
//! [`mrk_components`](https://crates.io/crates/mrk-components) crate;
//! for the `.mrk` wire format codec, see the
//! [`mrk_ir`](https://crates.io/crates/mrk-ir) crate.

#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
// The markup macros recognize structure with token-munching helper
// macros that cost one recursion frame per token; lift the default
// limit of 64. Downstream crates compiling very large templates may
// need to raise their own limit.
#![recursion_limit = "256"]

mod attributes;
mod constants;
mod display;
mod element;
mod macros;
mod node;
mod render_core;
mod renderable;

pub use attributes::{Attribute, AttributeType, attr};
pub use element::{Element, el};
pub use node::Node;
pub use renderable::{Renderable, render};

#[cfg(feature = "html")]
pub mod html;

#[cfg(feature = "svg")]
pub mod svg;

// Type-safe CSS authoring moved to the standalone `mrk-css` crate
// (https://github.com/signaes/mrk-css), and templated components to
// `mrk-components`; both depend on `mrk` for the `Renderable` trait
// and the `Node`/`Element` data model.
