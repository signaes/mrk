//! HTML rendering and tag factories for `mrk`.
//!
//! Enabled with the `html` feature:
//!
//! ```toml
//! [dependencies]
//! mrk = { version = "0.7.0", features = ["html"] }
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

mod elements;
mod raw;

pub use elements::*;
pub use raw::Raw;
