//! SVG rendering and tag factories for `mrk`.
//!
//! Enabled with the `svg` feature:
//!
//! ```toml
//! [dependencies]
//! mrk = { version = "0.7.1", features = ["svg"] }
//! ```
//!
//! ## Quick start
//!
//! ```
//! use mrk::{nodes, Node};
//! use mrk::svg::*;
//!
//! let child: Node = circle().cx("50").into();
//! let out = svg()
//!     .view_box("0 0 100 100")
//!     .xmlns("http://www.w3.org/2000/svg")
//!     .children(nodes![child])
//!     .render();
//! ```
//!
//! The `svg` module is independent of the `html` feature — both can be
//! enabled together. Attribute rendering uses the same [`Renderable`]
//! trait from the data model. Method names that translate to `camelCase`
//! SVG attributes use underscores (`view_box` → `viewBox`,
//! `preserve_aspect_ratio` → `preserveAspectRatio`).
//!
//! [`Renderable`]: crate::renderable::Renderable

mod elements;

pub use elements::*;
