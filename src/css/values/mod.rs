//! Typed CSS values.
//!
//! Each value type in this module is the strongly-typed counterpart
//! to a CSS lexical form. They power [`crate::css::Value`]
//! (populated in Phase 3) and [`crate::css::Color`] (Phase 1.14+).

mod numeric;

mod length;
pub use length::Length;

mod percentage;
pub use percentage::Percentage;

mod time;
pub use time::Time;

mod angle;
pub use angle::Angle;

mod frequency;
pub use frequency::Frequency;

mod resolution;
pub use resolution::Resolution;

mod number;
pub use number::{Number, Integer};

mod identifier;
pub use identifier::Ident;

mod url;
pub use url::Url;

mod string;
pub use string::CssString;

mod custom_property;
pub use custom_property::CustomProperty;

mod easing;
pub use easing::EasingFunction;

mod color;
#[allow(unused_imports)]
pub use color::{
    named_color_srgb, Color, ColorKind, ColorMix, ColorMixMethod, ColorMixSpace,
    ConversionError, ColorParseError, ColorSpace,
};