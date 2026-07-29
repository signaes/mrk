//! `@namespace` at-rule rendering.

use std::borrow::Cow;
use std::fmt;

/// Render an `@namespace [prefix] url;` statement.
pub fn render(
    f: &mut fmt::Formatter<'_>,
    prefix: Option<&Cow<'static, str>>,
    url: &Cow<'static, str>,
) -> fmt::Result {
    let mut s = String::from("@namespace");
    if let Some(p) = prefix { s.push_str(&format!(" {}", p)); }
    s.push_str(&format!(" \"{}\";", url));
    f.write_str(&s)
}
