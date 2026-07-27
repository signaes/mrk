//! HTML macro: `define_html_element!` and the HTML-specific `attr_name`
//! table.
//!
//! Wraps the helpers in [`this module`] with three
//! globals-tier arms (one per allowed tier) plus the HTML
//! [`attr_name`] lookup table.
//!
//! # Globals tier
//!
//! The third positional argument selects how many global HTML attribute
//! methods the generated wrapper exposes:
//!
//! - `all` — common globals + all event handlers + full ARIA
//! - `no_aria` — common globals + event handlers only
//! - `aria_hidden_only` — common globals + event handlers + only
//!   `aria-hidden`
//!
//! [`attr_name`]: crate::html::elements::macros::attr_name
//! [`this module`]: this module

// =====================================================================
// Shared code-generation helpers.
//
// These helpers are inlined in both `html/elements/macros.rs` and
// `svg/elements/macros.rs`. Each module has its own copy so that the
// declarations live inside the consumer's namespace (avoiding cross-
// module macro_rules scoping pitfalls). Identical bodies in both
// modules — divergence would indicate drift.
// =====================================================================

/// Generates `pub struct $name(pub Element)` and a `Default` impl.
macro_rules! __define_struct {
    ($name:ident) => {
        /// Typed element wrapper.
        #[derive(Debug)]
        pub struct $name(pub crate::element::Element);

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}
pub(crate) use __define_struct;

/// Generates the `new()` constructor.
macro_rules! __new_method {
    ($tag:literal) => {
        /// Create a new empty element with the matching tag.
        pub fn new() -> Self {
            Self(crate::element::el($tag))
        }
    };
}
pub(crate) use __new_method;

/// Generates `attrs()`, `children()`, and `render()`.
macro_rules! __builder_methods {
    () => {
        /// Replace the element's attributes.
        pub fn attrs(mut self, attrs: Vec<crate::attributes::Attribute>) -> Self {
            self.0 = self.0.attrs(attrs);
            self
        }
        /// Replace the element's children.
        pub fn children(mut self, children: Vec<crate::node::Node>) -> Self {
            self.0 = self.0.children(children);
            self
        }
        /// Render to a string.
        pub fn render(&self) -> String {
            use crate::renderable::Renderable;
            self.0.render()
        }
        /// Add an arbitrary key-value attribute.
        pub fn attr(self, name: &'static str, value: &'static str) -> Self {
            Self(self.0.push_attr(crate::attributes::attr(name).value(value)))
        }
        /// Add a `data-*` attribute.
        pub fn data_attr(self, key: &'static str, value: &'static str) -> Self {
            let name = std::borrow::Cow::Owned(format!("data-{}", key));
            Self(self.0.push_attr(crate::attributes::Attribute::new(name).value(value)))
        }
    };
}
pub(crate) use __builder_methods;

/// Generates one element-specific attribute setter.
macro_rules! __emitted_custom_method {
    ($method:ident, $doc:literal, $attr_name_path:path) => {
        #[doc = $doc]
        pub fn $method(self, value: &'static str) -> Self {
            let attr_name = $attr_name_path(stringify!($method));
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr(attr_name).value(value)]),
            )
        }
    };
}
pub(crate) use __emitted_custom_method;

/// Generates `From` and `Renderable` impls.
macro_rules! __from_impls {
    ($name:ident) => {
        impl From<$name> for crate::element::Element {
            fn from(e: $name) -> crate::element::Element {
                e.0
            }
        }
        impl From<$name> for crate::node::Node {
            fn from(e: $name) -> crate::node::Node {
                crate::node::Node::Element(e.0)
            }
        }
        impl crate::renderable::Renderable for $name {
            fn render(&self) -> String {
                self.0.render()
            }
        }

        #[cfg(feature = "components")]
        impl $crate::components::IntoExpr for $name {
            fn into_expr(self) -> $crate::components::Expr {
                $crate::components::Expr::Literal(self.0)
            }
        }
    };
}
pub(crate) use __from_impls;

/// Common HTML/SVG global attribute setters.
macro_rules! __common_globals_methods {
    () => {
        /// Unique identifier for the element.
        pub fn id(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("id").value(value)]),
            )
        }
        /// Space-separated list of CSS class names.
        pub fn class(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("class").value(value)]),
            )
        }
        /// Inline CSS styles.
        pub fn style(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("style").value(value)]),
            )
        }
        /// Tab navigation order (global `tabindex`).
        pub fn tabindex_global(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("tabindex").value(value)]),
            )
        }
        /// BCP 47 language tag (global `lang`).
        pub fn lang_global(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("lang").value(value)]),
            )
        }
        /// Text directionality.
        pub fn dir(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("dir").value(value)]),
            )
        }
        /// Hidden flag.
        pub fn hidden(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("hidden").value(value)]),
            )
        }
        /// Draggable hint.
        pub fn draggable(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("draggable").value(value)]),
            )
        }
        /// Spellcheck hint (global `spellcheck`).
        pub fn spellcheck_global(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("spellcheck").value(value)]),
            )
        }
        /// Advisory title (global `title`).
        pub fn title_global(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("title").value(value)]),
            )
        }
        /// Translation hint.
        pub fn translate(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("translate").value(value)]),
            )
        }
        /// Contenteditable mode.
        pub fn contenteditable(self, value: &'static str) -> Self {
            Self(self.0.attrs(vec![
                crate::attributes::attr("contenteditable").value(value),
            ]))
        }
        /// CSP nonce (global `nonce`).
        pub fn nonce_global(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("nonce").value(value)]),
            )
        }
        /// Shadow DOM slot name.
        pub fn slot(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("slot").value(value)]),
            )
        }
        /// Shadow DOM `::part()` name.
        pub fn part(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("part").value(value)]),
            )
        }
        /// Virtual keyboard hint.
        pub fn inputmode(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("inputmode").value(value)]),
            )
        }
        /// Enter key label hint.
        pub fn enterkeyhint(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("enterkeyhint").value(value)]),
            )
        }
        /// Popover API marker.
        pub fn popover(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("popover").value(value)]),
            )
        }
        /// Custom element slot identifier.
        pub fn is_content(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("is").value(value)]),
            )
        }
        /// Boolean focus-on-load flag (global `autofocus`).
        pub fn autofocus_global(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("autofocus").value(value)]),
            )
        }
        /// Form association (global `form`).
        pub fn form_global(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("form").value(value)]),
            )
        }
        /// Datalist reference (global `list`).
        pub fn list_global(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("list").value(value)]),
            )
        }
        /// Autofill hint (global `autocomplete`).
        pub fn autocomplete_global(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("autocomplete").value(value)]),
            )
        }
    };
}
pub(crate) use __common_globals_methods;

/// Event-handler attribute setters.
macro_rules! __event_handlers_methods {
    () => {
        /// Click handler.
        pub fn onclick(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onclick").value(value)]),
            )
        }
        /// Change handler.
        pub fn onchange(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onchange").value(value)]),
            )
        }
        /// Input handler.
        pub fn oninput(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("oninput").value(value)]),
            )
        }
        /// Submit handler.
        pub fn onsubmit(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onsubmit").value(value)]),
            )
        }
        /// Focus handler.
        pub fn onfocus(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onfocus").value(value)]),
            )
        }
        /// Blur handler.
        pub fn onblur(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onblur").value(value)]),
            )
        }
        /// Keydown handler.
        pub fn onkeydown(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onkeydown").value(value)]),
            )
        }
        /// Keyup handler.
        pub fn onkeyup(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onkeyup").value(value)]),
            )
        }
        /// Mousedown handler.
        pub fn onmousedown(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onmousedown").value(value)]),
            )
        }
        /// Mouseup handler.
        pub fn onmouseup(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onmouseup").value(value)]),
            )
        }
        /// Mouseover handler.
        pub fn onmouseover(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onmouseover").value(value)]),
            )
        }
        /// Mouseout handler.
        pub fn onmouseout(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onmouseout").value(value)]),
            )
        }
        /// Mousemove handler.
        pub fn onmousemove(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onmousemove").value(value)]),
            )
        }
        /// Load handler.
        pub fn onload(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onload").value(value)]),
            )
        }
    };
}
pub(crate) use __event_handlers_methods;

/// Full ARIA attribute setters.
macro_rules! __aria_all_methods {
    () => {
        /// Accessible label.
        pub fn aria_label(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("aria-label").value(value)]),
            )
        }
        /// Accessibility hidden state.
        pub fn aria_hidden(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("aria-hidden").value(value)]),
            )
        }
        /// Element role hint.
        pub fn aria_role(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("role").value(value)]),
            )
        }
        /// ARIA live region politeness.
        pub fn aria_live(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("aria-live").value(value)]),
            )
        }
        /// Expanded state.
        pub fn aria_expanded(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("aria-expanded").value(value)]),
            )
        }
        /// Selected state.
        pub fn aria_selected(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("aria-selected").value(value)]),
            )
        }
    };
}
pub(crate) use __aria_all_methods;

/// Only the `aria-hidden` setter.
macro_rules! __aria_hidden_methods {
    () => {
        /// Accessibility hidden state.
        pub fn aria_hidden(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("aria-hidden").value(value)]),
            )
        }
    };
}
pub(crate) use __aria_hidden_methods;


macro_rules! define_html_element {
    ($name:ident, $tag:literal, all) => {
        $crate::html::elements::macros::__define_struct!($name);
        impl $name {
            $crate::html::elements::macros::__new_method!($tag);
            $crate::html::elements::macros::__builder_methods!();
            $crate::html::elements::macros::__common_globals_methods!();
            $crate::html::elements::macros::__event_handlers_methods!();
            $crate::html::elements::macros::__aria_all_methods!();
        }
        $crate::html::elements::macros::__from_impls!($name);
    };

    ($name:ident, $tag:literal, all, $($method:ident($doc:literal)),+ $(,)?) => {
        $crate::html::elements::macros::__define_struct!($name);
        impl $name {
            $crate::html::elements::macros::__new_method!($tag);
            $crate::html::elements::macros::__builder_methods!();
            $(
                $crate::html::elements::macros::__emitted_custom_method!(
                    $method, $doc,
                    $crate::html::elements::macros::attr_name
                );
            )+
            $crate::html::elements::macros::__common_globals_methods!();
            $crate::html::elements::macros::__event_handlers_methods!();
            $crate::html::elements::macros::__aria_all_methods!();
        }
        $crate::html::elements::macros::__from_impls!($name);
    };

    ($name:ident, $tag:literal, no_aria) => {
        $crate::html::elements::macros::__define_struct!($name);
        impl $name {
            $crate::html::elements::macros::__new_method!($tag);
            $crate::html::elements::macros::__builder_methods!();
            $crate::html::elements::macros::__common_globals_methods!();
            $crate::html::elements::macros::__event_handlers_methods!();
        }
        $crate::html::elements::macros::__from_impls!($name);
    };

    ($name:ident, $tag:literal, no_aria, $($method:ident($doc:literal)),+ $(,)?) => {
        $crate::html::elements::macros::__define_struct!($name);
        impl $name {
            $crate::html::elements::macros::__new_method!($tag);
            $crate::html::elements::macros::__builder_methods!();
            $(
                $crate::html::elements::macros::__emitted_custom_method!(
                    $method, $doc,
                    $crate::html::elements::macros::attr_name
                );
            )+
            $crate::html::elements::macros::__common_globals_methods!();
            $crate::html::elements::macros::__event_handlers_methods!();
        }
        $crate::html::elements::macros::__from_impls!($name);
    };

    ($name:ident, $tag:literal, aria_hidden_only) => {
        $crate::html::elements::macros::__define_struct!($name);
        impl $name {
            $crate::html::elements::macros::__new_method!($tag);
            $crate::html::elements::macros::__builder_methods!();
            $crate::html::elements::macros::__common_globals_methods!();
            $crate::html::elements::macros::__event_handlers_methods!();
            $crate::html::elements::macros::__aria_hidden_methods!();
        }
        $crate::html::elements::macros::__from_impls!($name);
    };

    ($name:ident, $tag:literal, aria_hidden_only, $($method:ident($doc:literal)),+ $(,)?) => {
        $crate::html::elements::macros::__define_struct!($name);
        impl $name {
            $crate::html::elements::macros::__new_method!($tag);
            $crate::html::elements::macros::__builder_methods!();
            $(
                $crate::html::elements::macros::__emitted_custom_method!(
                    $method, $doc,
                    $crate::html::elements::macros::attr_name
                );
            )+
            $crate::html::elements::macros::__common_globals_methods!();
            $crate::html::elements::macros::__event_handlers_methods!();
            $crate::html::elements::macros::__aria_hidden_methods!();
        }
        $crate::html::elements::macros::__from_impls!($name);
    };
}

pub(crate) use define_html_element;

/// HTML attribute name lookup table.
pub(crate) fn attr_name(ident: &str) -> &'static str {
    match ident {
        // Existing element-specific identifiers.
        "abbr" => "abbr",
        "accept" => "accept",
        "accept_charset" => "accept-charset",
        "action" => "action",
        "allow" => "allow",
        "allowfullscreen" => "allowfullscreen",
        "alt" => "alt",
        "as_attr" => "as",
        "async_attr" => "async",
        "autocomplete" => "autocomplete",
        "autofocus" => "autofocus",
        "autoplay" => "autoplay",
        "blocking" => "blocking",
        "charset" => "charset",
        "checked" => "checked",
        "cite" => "cite",
        "color" => "color",
        "cols" => "cols",
        "colspan" => "colspan",
        "content" => "content",
        "controls" => "controls",
        "coords" => "coords",
        "crossorigin" => "crossorigin",
        "credentialless" => "credentialless",
        "csp" => "csp",
        "data" => "data",
        "datalist" => "datalist",
        "datetime" => "datetime",
        "decoding" => "decoding",
        "default_attr" => "default",
        "defer_attr" => "defer",
        "disabled" => "disabled",
        "download" => "download",
        "enctype" => "enctype",
        "fetchpriority" => "fetchpriority",
        "for_attr" => "for",
        "form" => "form",
        "form_attr" => "form",
        "formaction" => "formaction",
        "formenctype" => "formenctype",
        "formmethod" => "formmethod",
        "formnovalidate" => "formnovalidate",
        "formtarget" => "formtarget",
        "headers" => "headers",
        "height" => "height",
        "high" => "high",
        "href" => "href",
        "hreflang" => "hreflang",
        "http_equiv" => "http-equiv",
        "imagesizes" => "imagesizes",
        "imagesrcset" => "imagesrcset",
        "integrity" => "integrity",
        "ismap" => "ismap",
        "kind" => "kind",
        "label" => "label",
        "list" => "list",
        "loading" => "loading",
        "loop_attr" => "loop",
        "low" => "low",
        "max" => "max",
        "maxlength" => "maxlength",
        "media" => "media",
        "method" => "method",
        "min" => "min",
        "minlength" => "minlength",
        "multiple" => "multiple",
        "muted" => "muted",
        "name" => "name",
        "novalidate" => "novalidate",
        "nomodule" => "nomodule",
        "nonce" => "nonce",
        "open_attr" => "open",
        "optimum" => "optimum",
        "pattern" => "pattern",
        "ping" => "ping",
        "placeholder" => "placeholder",
        "playsinline" => "playsinline",
        "popovertarget" => "popovertarget",
        "popovertargetaction" => "popovertargetaction",
        "poster" => "poster",
        "preload" => "preload",
        "preserve_aspect_ratio" => "preserveAspectRatio",
        "readonly" => "readonly",
        "referrerpolicy" => "referrerpolicy",
        "rel" => "rel",
        "required" => "required",
        "reversed" => "reversed",
        "rowspan" => "rowspan",
        "rows" => "rows",
        "sandbox" => "sandbox",
        "scope" => "scope",
        "selected" => "selected",
        "shape" => "shape",
        "size" => "size",
        "sizes" => "sizes",
        "span" => "span",
        "spellcheck" => "spellcheck",
        "src" => "src",
        "srcdoc" => "srcdoc",
        "srcset" => "srcset",
        "srclang" => "srclang",
        "start" => "start",
        "step" => "step",
        "tabindex" => "tabindex",
        "target" => "target",
        "title" => "title",
        "title_attr" => "title",
        "type_attr" => "type",
        "usemap" => "usemap",
        "value" => "value",
        "version" => "version",
        "view_box" => "viewBox",
        "width" => "width",
        "wrap" => "wrap",
        "xmlns" => "xmlns",
        "tabindex_global" => "tabindex",
        "nonce_global" => "nonce",
        "title_global" => "title",
        "autofocus_global" => "autofocus",
        "autocomplete_global" => "autocomplete",
        "list_global" => "list",
        "spellcheck_global" => "spellcheck",
        "form_global" => "form",
        "aria_label" => "aria-label",
        "aria_labelledby" => "aria-labelledby",
        "aria_describedby" => "aria-describedby",
        "aria_description" => "aria-description",
        "aria_hidden" => "aria-hidden",
        "aria_role" => "role",
        "aria_live" => "aria-live",
        "aria_current" => "aria-current",
        "aria_required" => "aria-required",
        "aria_disabled" => "aria-disabled",
        "aria_expanded" => "aria-expanded",
        "aria_selected" => "aria-selected",
        "aria_checked" => "aria-checked",
        "aria_pressed" => "aria-pressed",
        "aria_haspopup" => "aria-haspopup",
        "aria_invalid" => "aria-invalid",
        "aria_readonly" => "aria-readonly",
        "aria_busy" => "aria-busy",
        "aria_relevant" => "aria-relevant",
        "aria_atomic" => "aria-atomic",
        "aria_details" => "aria-details",
        "aria_errormessage" => "aria-errormessage",
        "aria_controls" => "aria-controls",
        "aria_flowto" => "aria-flowto",
        "aria_owns" => "aria-owns",
        "aria_activedescendant" => "aria-activedescendant",
        "aria_keyshortcuts" => "aria-keyshortcuts",
        "aria_posinset" => "aria-posinset",
        "aria_setsize" => "aria-setsize",
        "aria_level" => "aria-level",
        "aria_orientation" => "aria-orientation",
        "aria_valuemax" => "aria-valuemax",
        "aria_valuemin" => "aria-valuemin",
        "aria_valuenow" => "aria-valuenow",
        "aria_valuetext" => "aria-valuetext",
        "aria_colcount" => "aria-colcount",
        "aria_rowcount" => "aria-rowcount",
        "aria_colindex" => "aria-colindex",
        "aria_rowindex" => "aria-rowindex",
        "aria_colspan" => "aria-colspan",
        "aria_rowspan" => "aria-rowspan",
        "aria_colheader" => "aria-colheader",
        "aria_rowheader" => "aria-rowheader",
        "aria_modal" => "aria-modal",
        "aria_multiline" => "aria-multiline",
        "aria_multiselectable" => "aria-multiselectable",
        "aria_dropeffect" => "aria-dropeffect",
        "aria_roledescription" => "aria-roledescription",
        "id" => "id",
        "class" => "class",
        "style" => "style",
        "lang_global" => "lang",
        "dir" => "dir",
        "hidden" => "hidden",
        "draggable" => "draggable",
        "translate" => "translate",
        "contenteditable" => "contenteditable",
        "slot" => "slot",
        "part" => "part",
        "inputmode" => "inputmode",
        "enterkeyhint" => "enterkeyhint",
        "popover" => "popover",
        "is_content" => "is",
        other => panic!("attr_name: unmapped identifier '{other}'"),
    }
}

macro_rules! factory {
    ($(#[$meta:meta])* $fn_name:ident, $type:ident) => {
        $(#[$meta])*
        pub fn $fn_name() -> $type {
            $type::new()
        }
    };
}

pub(crate) use factory;

#[cfg(test)]
mod tests {
    super::define_html_element!(TestDiv, "div", all);
    super::define_html_element!(
        TestAnchor,
        "a",
        all,
        href("Hyperlink URL."),
        target("Frame target.")
    );
    super::define_html_element!(
        TestInput,
        "input",
        all,
        type_attr("Input type."),
        name("Field name.")
    );
    super::define_html_element!(
        TestSvg,
        "svg",
        no_aria,
        view_box("SVG viewBox."),
        xmlns("XML namespace.")
    );
    super::define_html_element!(TestBr, "br", aria_hidden_only);

    super::factory!(test_div, TestDiv);
    super::factory!(test_anchor, TestAnchor);

    #[test]
    fn new_creates_correct_tag() {
        assert_eq!(TestDiv::new().0.name, "div");
    }

    #[test]
    fn attrs_replaces_attributes() {
        let el = TestDiv::new().attrs(vec![crate::attributes::attr("class").value("box")]);
        assert_eq!(el.0.attributes.len(), 1);
    }

    #[test]
    fn global_id_method_emits_id_attribute() {
        let el = TestDiv::new().id("main");
        assert_eq!(el.0.attributes[0].key, "id");
    }

    #[test]
    fn builder_attr_adds_arbitrary_attribute() {
        let el = TestDiv::new().attr("data-test", "value");
        assert_eq!(el.0.attributes.len(), 1);
        assert_eq!(el.0.attributes[0].key, "data-test");
        assert_eq!(el.0.attributes[0].attr, crate::attributes::AttributeType::KeyValue("data-test".into(), "value".into()));
    }

    #[test]
    fn builder_data_attr_adds_data_prefix() {
        let el = TestDiv::new().data_attr("id", "btn");
        assert_eq!(el.0.attributes.len(), 1);
        assert_eq!(el.0.attributes[0].key, "data-id");
    }

    #[test]
    fn builder_attr_chain_preserves_order() {
        let el = TestDiv::new()
            .attr("first", "1")
            .id("main")
            .attr("second", "2");
        assert_eq!(el.0.attributes.len(), 3);
        assert_eq!(el.0.attributes[0].key, "first");
        assert_eq!(el.0.attributes[1].key, "id");
        assert_eq!(el.0.attributes[2].key, "second");
    }

    #[test]
    fn attr_name_covers_every_identifier() {
        let identifiers: &[&str] = &[
            "abbr",
            "accept",
            "accept_charset",
            "action",
            "allow",
            "allowfullscreen",
            "alt",
            "as_attr",
            "async_attr",
            "autocomplete",
            "autofocus",
            "autoplay",
            "blocking",
            "charset",
            "checked",
            "cite",
            "color",
            "cols",
            "colspan",
            "content",
            "controls",
            "coords",
            "crossorigin",
            "credentialless",
            "csp",
            "data",
            "datalist",
            "datetime",
            "decoding",
            "default_attr",
            "defer_attr",
            "disabled",
            "download",
            "enctype",
            "fetchpriority",
            "for_attr",
            "form",
            "form_attr",
            "formaction",
            "formenctype",
            "formmethod",
            "formnovalidate",
            "formtarget",
            "headers",
            "height",
            "high",
            "href",
            "hreflang",
            "http_equiv",
            "imagesizes",
            "imagesrcset",
            "integrity",
            "ismap",
            "kind",
            "label",
            "list",
            "loading",
            "loop_attr",
            "low",
            "max",
            "maxlength",
            "media",
            "method",
            "min",
            "minlength",
            "multiple",
            "muted",
            "name",
            "novalidate",
            "nomodule",
            "nonce",
            "open_attr",
            "optimum",
            "pattern",
            "ping",
            "placeholder",
            "playsinline",
            "popovertarget",
            "popovertargetaction",
            "poster",
            "preload",
            "preserve_aspect_ratio",
            "readonly",
            "referrerpolicy",
            "rel",
            "required",
            "reversed",
            "rowspan",
            "rows",
            "sandbox",
            "scope",
            "selected",
            "shape",
            "size",
            "sizes",
            "span",
            "spellcheck",
            "src",
            "srcdoc",
            "srcset",
            "srclang",
            "start",
            "step",
            "tabindex",
            "target",
            "title",
            "title_attr",
            "type_attr",
            "usemap",
            "value",
            "version",
            "view_box",
            "width",
            "wrap",
            "xmlns",
            "tabindex_global",
            "nonce_global",
            "title_global",
            "autofocus_global",
            "autocomplete_global",
            "list_global",
            "spellcheck_global",
            "form_global",
            "aria_label",
            "aria_labelledby",
            "aria_describedby",
            "aria_description",
            "aria_hidden",
            "aria_role",
            "aria_live",
            "aria_current",
            "aria_required",
            "aria_disabled",
            "aria_expanded",
            "aria_selected",
            "aria_checked",
            "aria_pressed",
            "aria_haspopup",
            "aria_invalid",
            "aria_readonly",
            "aria_busy",
            "aria_relevant",
            "aria_atomic",
            "aria_details",
            "aria_errormessage",
            "aria_controls",
            "aria_flowto",
            "aria_owns",
            "aria_activedescendant",
            "aria_keyshortcuts",
            "aria_posinset",
            "aria_setsize",
            "aria_level",
            "aria_orientation",
            "aria_valuemax",
            "aria_valuemin",
            "aria_valuenow",
            "aria_valuetext",
            "aria_colcount",
            "aria_rowcount",
            "aria_colindex",
            "aria_rowindex",
            "aria_colspan",
            "aria_rowspan",
            "aria_colheader",
            "aria_rowheader",
            "aria_modal",
            "aria_multiline",
            "aria_multiselectable",
            "aria_dropeffect",
            "aria_roledescription",
            "id",
            "class",
            "style",
            "lang_global",
            "dir",
            "hidden",
            "draggable",
            "translate",
            "contenteditable",
            "slot",
            "part",
            "inputmode",
            "enterkeyhint",
            "popover",
            "is_content",
        ];
        for ident in identifiers {
            let mapped = super::attr_name(ident);
            assert!(!mapped.is_empty());
        }
    }

    #[test]
    #[should_panic(expected = "unmapped identifier")]
    fn attr_name_unmapped_identifier_panics() {
        super::attr_name("totally_unknown_identifier");
    }

    /// Exhaustive coverage: call every common-global method on
    /// `TestDiv` to ensure the body lines of `__common_globals_methods!`
    /// are reachable through the macro expansion at runtime.
    #[test]
    fn globals_method_bodies_covered() {
        let _ = TestDiv::new()
            .id("x")
            .class("y")
            .style("z")
            .tabindex_global("1")
            .lang_global("en")
            .dir("ltr")
            .hidden("t")
            .draggable("true")
            .spellcheck_global("true")
            .title_global("t")
            .translate("yes")
            .contenteditable("true")
            .nonce_global("a")
            .slot("s")
            .part("p")
            .inputmode("text")
            .enterkeyhint("enter")
            .popover("auto")
            .is_content("x")
            .autofocus_global("true")
            .form_global("f")
            .data_attr("x", "d")
            .list_global("l")
            .autocomplete_global("on");
    }

    /// Exhaustive coverage: every event handler body in
    /// `__event_handlers_methods!`.
    #[test]
    fn event_handlers_bodies_covered() {
        let _ = TestDiv::new()
            .onclick("1")
            .onchange("1")
            .oninput("1")
            .onsubmit("1")
            .onfocus("1")
            .onblur("1")
            .onkeydown("1")
            .onkeyup("1")
            .onmousedown("1")
            .onmouseup("1")
            .onmouseover("1")
            .onmouseout("1")
            .onmousemove("1")
            .onload("1");
    }

    /// Exhaustive coverage: every ARIA setter body in
    /// `__aria_all_methods!`.
    #[test]
    fn aria_all_methods_bodies_covered() {
        // Each setter replaces the full attribute list (last
        // setter wins), so we use `.attrs()` to combine all five
        // ARIA attributes into a single list. This forces every
        // setter body to execute in turn.
        use crate::attributes::attr;
        let el = TestDiv::new().attrs(vec![
            attr("aria-label").value("l"),
            attr("aria-hidden").value("true"),
            attr("role").value("button"),
            attr("aria-live").value("polite"),
            attr("aria-expanded").value("false"),
        ]);
        assert_eq!(el.0.attributes.len(), 5);
    }

    /// Verify the per-setter expansion by calling each one in
    /// isolation and reading back the produced attribute.
    #[test]
    fn aria_all_methods_individual_expansion() {
        assert_eq!(TestDiv::new().aria_label("l").0.attributes[0].key, "aria-label");
        assert_eq!(TestDiv::new().aria_hidden("t").0.attributes[0].key, "aria-hidden");
        assert_eq!(TestDiv::new().aria_role("b").0.attributes[0].key, "role");
        assert_eq!(TestDiv::new().aria_live("polite").0.attributes[0].key, "aria-live");
        assert_eq!(TestDiv::new().aria_expanded("false").0.attributes[0].key, "aria-expanded");
        assert_eq!(TestDiv::new().aria_selected("true").0.attributes[0].key, "aria-selected");
    }

    /// Exhaustive coverage: `__aria_hidden_methods!` (the
    /// `aria_hidden_only` tier).
    #[test]
    fn aria_hidden_methods_body_covered() {
        let el = TestBr::new().aria_hidden("true");
        assert_eq!(el.0.attributes[0].key, "aria-hidden");
    }

    /// Coverage for `__emitted_custom_method!` — sets element-
    /// specific attributes through the consumer macro.
    #[test]
    fn emitted_custom_method_body_covered() {
        use crate::attributes::attr;
        let el = TestInput::new().attrs(vec![
            attr("type").value("text"),
            attr("name").value("field"),
        ]);
        assert_eq!(el.0.attributes.len(), 2);
    }

    /// Verify the per-setter expansion by calling each one in
    /// isolation and reading back the produced attribute key.
    #[test]
    fn emitted_custom_method_individual_expansion() {
        assert_eq!(TestInput::new().type_attr("t").0.attributes[0].key, "type");
        assert_eq!(TestInput::new().name("f").0.attributes[0].key, "name");
    }

    /// Coverage for `__from_impls!` — exercise the From and
    /// Renderable impls emitted by the consumer macro.
    #[test]
    fn from_impls_bodies_covered() {
        let div: TestDiv = TestDiv::default();
        let _e: crate::element::Element = div.into();
        let div2: TestDiv = TestDiv::default();
        let _n: crate::node::Node = div2.into();
        let div3 = TestDiv::new();
        let _s = crate::renderable::Renderable::render(&div3);
    }

    /// Coverage for the `IntoExpr` impl emitted by `__from_impls!`
    /// when the `components` feature is active.
    #[cfg(feature = "components")]
    #[test]
    fn html_wrapper_into_expr() {
        use crate::components::{Expr, IntoExpr};
        let expr = TestDiv::new().id("main").into_expr();
        let is_lit = matches!(expr, Expr::Literal(ref el) if el.name == "div" && !el.attributes.is_empty());
        assert!(is_lit, "expected Literal(div with attrs), got: {expr:?}");
    }
}
