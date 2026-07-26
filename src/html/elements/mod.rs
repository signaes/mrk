//! Typed HTML element wrappers.
//!
//! Each HTML tag has a corresponding typed wrapper struct (e.g.,
//! [`HtmlDiv`], [`HtmlAnchor`]). These wrap the generic [`Element`]
//! type and add element-specific attribute setter methods.
//!
//! Use the free factory functions ([`div`], [`a`], etc.) to construct
//! elements, then chain attribute setters and children.
//!
//! Attribute name conversion rules:
//!
//! - `_attr` suffix is stripped (`for_attr` → `for`, `type_attr` → `type`)
//! - remaining `_` becomes `-` (`view_box` → `view-box`, `http_equiv` → `http-equiv`)
//!
//! [`Element`]: crate::element::Element

mod macros;
mod document;
mod edit;
mod embedded;
mod forms;
mod foreign;
mod grouping;
mod interactive;
mod sections;
mod tables;
mod text;

pub use document::*;
pub use edit::*;
pub use embedded::*;
pub use forms::*;
pub use foreign::*;
pub use grouping::*;
pub use interactive::*;
pub use sections::*;
pub use tables::*;
pub use text::*;
