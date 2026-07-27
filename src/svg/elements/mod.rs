//! Typed SVG element wrappers.
//!
//! Each SVG tag has a corresponding typed wrapper struct (e.g.
//! [`SvgCircle`], [`SvgRect`]). These wrap the generic [`Element`] type
//! and add element-specific attribute setter methods.
//!
//! Use the free factory functions ([`circle`], [`rect`], etc.) to
//! construct elements, then chain attribute setters and children.
//!
//! Attribute name conversion rules (matching the HTML macro):
//!
//! - `_attr` suffix is stripped (`type_attr` → `type`)
//! - remaining `_` becomes `-` for hyphenated names
//! - explicit overrides exist for `camelCase` SVG-specific attributes:
//!   `view_box` → `viewBox`, `preserve_aspect_ratio` → `preserveAspectRatio`,
//!   `gradient_transform` → `gradientTransform`, etc.
//!
//! Every wrapper is generated with full ARIA support (the `all` tier)
//! plus all common globals and event handlers, per SVG 2.
//!
//! [`Element`]: crate::element::Element

mod macros;
mod animation;
mod container;
mod descriptive;
mod filter;
mod font;
mod gradient;
mod mask;
mod pattern_;
mod shapes;
mod text;

pub use animation::*;
pub use container::*;
pub use descriptive::*;
pub use filter::*;
pub use font::*;
pub use gradient::*;
pub use mask::*;
pub use pattern_::*;
pub use shapes::*;
pub use text::*;
