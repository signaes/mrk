//! `@charset` at-rule rendering.

use std::borrow::Cow;
use std::fmt;

/// Render an `@charset "encoding";` statement.
pub fn render(f: &mut fmt::Formatter<'_>, encoding: &Cow<'static, str>) -> fmt::Result {
    write!(f, "@charset \"{}\";", encoding)
}
