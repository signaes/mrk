//! # mrk
//!
//! A small HTML builder library for Rust.
//!
//! `mrk` lets you construct HTML with a fluent builder API: create elements
//! with [`el`], attach attributes with [`attr`], add children with the
//! [`children!`] macro (mixing [`text`] and nested elements freely), then
//! call `.render()` to produce an HTML string. For custom output, implement
//! the [`Renderable`] trait.
//!
//! ## Quick start
//!
//! ```
//! use mrk::*;
//!
//! let html = el("a")
//!     .attrs(vec![attr("href").value("/")])
//!     .children(children![text("Home")])
//!     .render();
//!
//! assert_eq!(html, r#"<a href="/">Home</a>"#);
//! ```
//!
//! ## Factories
//!
//! For common HTML tags, use the factory functions (e.g. [`div`], [`p`], [`span`]):
//!
//! ```
//! use mrk::*;
//!
//! let html = div().children(children![
//!     text("Hello, "),
//!     el("strong").children(children![text("world")]),
//! ]).render();
//!
//! assert_eq!(html, "<div>Hello, <strong>world</strong></div>");
//! ```
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
mod constants;
mod element;
mod elements;
mod macros;
mod node;
mod renderable;

pub use attributes::attr;
pub use element::el;
pub use elements::*;
pub use node::{node, text, Node};
pub use renderable::{Renderable, render};
