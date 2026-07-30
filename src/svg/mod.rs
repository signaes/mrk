//! SVG rendering and tag factories for `mrk`.
//!
//! Enabled with the `svg` feature:
//!
//! ```toml
//! [dependencies]
//! mrk = { version = "0.9.0", features = ["svg"] }
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
//! ## Declarative macro
//!
//! The [`svg!`](crate::svg) macro builds the same trees with a
//! markup-like syntax (attribute names are written verbatim):
//!
//! ```
//! use mrk::*;
//!
//! let icon = svg! { svg(viewBox="0 0 10 10") { circle(cx="5" cy="5" r="4") } };
//! assert_eq!(icon.render(), r#"<svg viewBox="0 0 10 10"><circle cx="5" cy="5" r="4"></circle></svg>"#);
//! ```
//!
//! [`Renderable`]: crate::renderable::Renderable

mod elements;

pub use elements::*;
