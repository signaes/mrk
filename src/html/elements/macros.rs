//! HTML macro: `define_html_element!` and the HTML-specific `attr_name`
//! table.
//!
//! Wraps the helpers in [`crate::shared_macros`] with three
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
//! [`crate::shared_macros`]: crate::shared_macros

macro_rules! define_html_element {
    ($name:ident, $tag:literal, all) => {
        $crate::shared_macros::__define_struct!($name);
        impl $name {
            $crate::shared_macros::__new_method!($tag);
            $crate::shared_macros::__builder_methods!();
            $crate::shared_macros::__common_globals_methods!();
            $crate::shared_macros::__event_handlers_methods!();
            $crate::shared_macros::__aria_all_methods!();
        }
        $crate::shared_macros::__from_impls!($name);
    };

    ($name:ident, $tag:literal, all, $($method:ident($doc:literal)),+ $(,)?) => {
        $crate::shared_macros::__define_struct!($name);
        impl $name {
            $crate::shared_macros::__new_method!($tag);
            $crate::shared_macros::__builder_methods!();
            $(
                $crate::shared_macros::__emitted_custom_method!(
                    $method, $doc,
                    $crate::html::elements::macros::attr_name
                );
            )+
            $crate::shared_macros::__common_globals_methods!();
            $crate::shared_macros::__event_handlers_methods!();
            $crate::shared_macros::__aria_all_methods!();
        }
        $crate::shared_macros::__from_impls!($name);
    };

    ($name:ident, $tag:literal, no_aria) => {
        $crate::shared_macros::__define_struct!($name);
        impl $name {
            $crate::shared_macros::__new_method!($tag);
            $crate::shared_macros::__builder_methods!();
            $crate::shared_macros::__common_globals_methods!();
            $crate::shared_macros::__event_handlers_methods!();
        }
        $crate::shared_macros::__from_impls!($name);
    };

    ($name:ident, $tag:literal, no_aria, $($method:ident($doc:literal)),+ $(,)?) => {
        $crate::shared_macros::__define_struct!($name);
        impl $name {
            $crate::shared_macros::__new_method!($tag);
            $crate::shared_macros::__builder_methods!();
            $(
                $crate::shared_macros::__emitted_custom_method!(
                    $method, $doc,
                    $crate::html::elements::macros::attr_name
                );
            )+
            $crate::shared_macros::__common_globals_methods!();
            $crate::shared_macros::__event_handlers_methods!();
        }
        $crate::shared_macros::__from_impls!($name);
    };

    ($name:ident, $tag:literal, aria_hidden_only) => {
        $crate::shared_macros::__define_struct!($name);
        impl $name {
            $crate::shared_macros::__new_method!($tag);
            $crate::shared_macros::__builder_methods!();
            $crate::shared_macros::__common_globals_methods!();
            $crate::shared_macros::__event_handlers_methods!();
            $crate::shared_macros::__aria_hidden_methods!();
        }
        $crate::shared_macros::__from_impls!($name);
    };

    ($name:ident, $tag:literal, aria_hidden_only, $($method:ident($doc:literal)),+ $(,)?) => {
        $crate::shared_macros::__define_struct!($name);
        impl $name {
            $crate::shared_macros::__new_method!($tag);
            $crate::shared_macros::__builder_methods!();
            $(
                $crate::shared_macros::__emitted_custom_method!(
                    $method, $doc,
                    $crate::html::elements::macros::attr_name
                );
            )+
            $crate::shared_macros::__common_globals_methods!();
            $crate::shared_macros::__event_handlers_methods!();
            $crate::shared_macros::__aria_hidden_methods!();
        }
        $crate::shared_macros::__from_impls!($name);
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
        "data_x" => "data-x",
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
    use crate::attributes::AttributeType;
    use crate::element::Element;
    use crate::node::Node;
    use crate::renderable::Renderable;
    use std::borrow::Cow;

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
            "data_x",
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
}
