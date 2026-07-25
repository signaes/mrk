//! HTML rendering and tag factories for `mrk`.
//!
//! Enabled with the `html` feature:
//!
//! ```toml
//! [dependencies]
//! mrk = { version = "0.3", features = ["html"] }
//! ```
//!
//! ## Quick start
//!
//! ```
//! use mrk::*;
//!
//! let html = el("a")
//!     .attrs(vec![attr("href").value("/")])
//!     .children(nodes!["Home"])
//!     .render();
//!
//! assert_eq!(html, r#"<a href="/">Home</a>"#);
//! ```

mod constants;
mod factories;
mod raw;
mod render;

pub use factories::*;
pub use raw::Raw;
