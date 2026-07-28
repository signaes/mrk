//! Common module: re-exports shared items used by every HTML tag wrapper.

pub use crate::components::{ComponentElement, IntoExpr};
pub use crate::node::Node;

pub(crate) use crate::components::element::__define_component_wrapper;

/// Helper macro to define a typed HTML wrapper with common attrs plus
/// element-specific attrs.
#[macro_export]
#[doc(hidden)]
macro_rules! __component_html_tag {
    ($struct:ident, $factory:ident, $tag:literal, $($method:ident => $attr:literal),* $(,)?) => {{
        $crate::components::element::__define_component_wrapper!(
            $struct,
            $factory,
            $tag,
            class => "class",
            id => "id",
            style => "style",
            title => "title",
            $($method => $attr),*
        );
    }};
}

pub(crate) use __component_html_tag;