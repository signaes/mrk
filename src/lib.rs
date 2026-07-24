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
//! By default, `mrk` provides only the data model and builder API. Enable
//! a feature for built-in rendering:
//!
//! - `html` — HTML rendering, 116 tag factories, void elements, escaping
//!
//! ```toml
//! [dependencies]
//! mrk = { version = "0.3", features = ["html"] }
//! ```
//!
//! ## Building trees without rendering
//!
//! Without any feature, `mrk` builds trees but doesn't render them:
//!
//! ```
//! use mrk::*;
//!
//! let tree = el("custom-tag")
//!     .attrs(vec![attr("name").value("value")])
//!     .children(nodes!["data"]);
//!
//! assert_eq!(tree.name, "custom-tag");
//! ```
//!
//! Implement [`Renderable`] for your own renderer, or enable a feature
//! to use a built-in one.
//!
//! ## Extending with [`Renderable`]
//!
//! Any type can be rendered by implementing [`Renderable`]:
//!
//! ```
//! use mrk::*;
//!
//! struct Greeting(&'static str);
//!
//! impl Renderable for Greeting {
//!     fn render(&self) -> String {
//!         format!("<p>Hello, {}!</p>", self.0)
//!     }
//! }
//!
//! assert_eq!(render(Greeting("world")), "<p>Hello, world!</p>");
//! ```

mod attributes;
mod element;
mod macros;
mod node;
mod renderable;

pub use attributes::{Attribute, AttributeType, attr};
pub use element::{Element, el};
pub use node::Node;
pub use renderable::{Renderable, render};

#[cfg(feature = "html")]
pub mod html;
