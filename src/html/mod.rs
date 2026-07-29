//! HTML rendering and tag factories for `mrk`.
//!
//! Enabled with the `html` feature:
//!
//! ```toml
//! [dependencies]
//! mrk = { version = "0.8.0", features = ["html"] }
//! ```
//!
//! ## Quick start
//!
//! ```
//! use mrk::*;
//! use mrk::html::*;
//!
//! let html = div()
//!     .attrs(vec![attr("class").value("container")])
//!     .children(nodes!["Hello"])
//!     .render();
//! ```
//!
//! ## Declarative macro
//!
//! The [`html!`](crate::html) macro builds the same trees with a
//! markup-like syntax:
//!
//! ```
//! use mrk::*;
//!
//! let tree = html! { div(class="container") { span() { "Hello" } } };
//! assert_eq!(tree.render(), r#"<div class="container"><span>Hello</span></div>"#);
//! ```

mod elements;
mod raw;

pub use elements::*;
pub use raw::Raw;
