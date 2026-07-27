//! SVG macro: `define_svg_element!` and the SVG-specific `attr_name`
//! table.
//!
//! Wraps the helpers in [`this module`] and adds SVG-specific
//! `attr_name` lookup rules. All generated wrappers use the `all`
//! globals tier per SVG 2.

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

/// Only the `aria-hidden` setter. (Not used by SVG which only ever
/// uses the `all` tier; declared here only for parity with the HTML
/// macro so the two files remain interchangeable.)
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
// SVG never uses `__aria_hidden_methods` (only `all` tier); the
// `pub(crate) use` is conditionally re-exported so consumers can
// still reference it by full path.
#[allow(unused_imports)]
pub(crate) use __aria_hidden_methods;


macro_rules! define_svg_element {
    ($name:ident, $tag:literal) => {
        $crate::svg::elements::macros::__define_struct!($name);
        impl $name {
            $crate::svg::elements::macros::__new_method!($tag);
            $crate::svg::elements::macros::__builder_methods!();
            $crate::svg::elements::macros::__common_globals_methods!();
            $crate::svg::elements::macros::__event_handlers_methods!();
            $crate::svg::elements::macros::__aria_all_methods!();
        }
        $crate::svg::elements::macros::__from_impls!($name);
    };

    ($name:ident, $tag:literal, $($method:ident($doc:literal)),+ $(,)?) => {
        $crate::svg::elements::macros::__define_struct!($name);
        impl $name {
            $crate::svg::elements::macros::__new_method!($tag);
            $crate::svg::elements::macros::__builder_methods!();
            $(
                $crate::svg::elements::macros::__emitted_custom_method!(
                    $method, $doc,
                    $crate::svg::elements::macros::attr_name
                );
            )+
            $crate::svg::elements::macros::__common_globals_methods!();
            $crate::svg::elements::macros::__event_handlers_methods!();
            $crate::svg::elements::macros::__aria_all_methods!();
        }
        $crate::svg::elements::macros::__from_impls!($name);
    };
}

pub(crate) use define_svg_element;

/// Resolve an SVG method identifier to its attribute name.
///
/// SVG uses `camelCase` attribute names — the macro translates
/// `view_box` → `viewBox`, `preserve_aspect_ratio` →
/// `preserveAspectRatio`, etc.
pub(crate) fn attr_name(ident: &str) -> &'static str {
    match ident {
        // ---- camelCase SVG attributes ----
        "view_box" => "viewBox",
        "preserve_aspect_ratio" => "preserveAspectRatio",
        "gradient_transform" => "gradientTransform",
        "gradient_units" => "gradientUnits",
        "spread_method" => "spreadMethod",
        "pattern_units" => "patternUnits",
        "pattern_content_units" => "patternContentUnits",
        "pattern_transform" => "patternTransform",
        "clip_path_units" => "clipPathUnits",
        "clip_path" => "clip-path",
        "marker_units" => "markerUnits",
        "marker_width" => "markerWidth",
        "marker_height" => "markerHeight",
        "ref_x" => "refX",
        "ref_y" => "refY",
        "ref_width" => "refWidth",
        "ref_height" => "refHeight",
        "marker_start" => "marker-start",
        "marker_mid" => "marker-mid",
        "marker_end" => "marker-end",
        "std_deviation" => "stdDeviation",
        "base_frequency" => "baseFrequency",
        "num_octaves" => "numOctaves",
        "target_x" => "targetX",
        "target_y" => "targetY",
        "kernel_unit_length" => "kernelUnitLength",
        "kernel_matrix" => "kernelMatrix",
        "divisor" => "divisor",
        "bias" => "bias",
        "specular_constant" => "specularConstant",
        "specular_exponent" => "specularExponent",
        "limiting_cone_angle" => "limitingConeAngle",
        "points_at_x" => "pointsAtX",
        "points_at_y" => "pointsAtY",
        "points_at_z" => "pointsAtZ",
        "surface_scale" => "surfaceScale",
        "diffuse_constant" => "diffuseConstant",
        "table_values" => "tableValues",
        "attribute_name" => "attributeName",
        "attribute_type" => "attributeType",
        "calc_mode" => "calcMode",
        "key_times" => "keyTimes",
        "key_splines" => "keySplines",
        "key_points" => "keyPoints",
        "dur" => "dur",
        "begin" => "begin",
        "end" => "end",
        "restart_count" => "restart",
        "restart_attr" => "restart",
        "repeat_count" => "repeatCount",
        "repeat_dur" => "repeatDur",
        "fill_attr" => "fill",
        "auto_reverse" => "autoReverse",
        "accumulate_attr" => "accumulate",
        "additive_attr" => "additive",
        "begin_offset" => "beginOffset",
        "begin_event_count" => "beginEventCount",
        "end_offset" => "endOffset",
        "end_event_count" => "endEventCount",
        "active_duration" => "activeDuration",
        "simple_result" => "simpleResult",
        "upper_limit" => "upperLimit",
        "lower_limit" => "lowerLimit",
        "sync_behavior" => "syncBehavior",
        "sync_tolerance" => "syncTolerance",
        "preserve_alpha" => "preserveAlpha",

        // ---- underscore → dash mappings ----
        "stroke_width" => "stroke-width",
        "stroke_dasharray" => "stroke-dasharray",
        "stroke_dashoffset" => "stroke-dashoffset",
        "stroke_linecap" => "stroke-linecap",
        "stroke_linejoin" => "stroke-linejoin",
        "stroke_miterlimit" => "stroke-miterlimit",
        "stroke_opacity" => "stroke-opacity",
        "fill_rule" => "fill-rule",
        "fill_opacity" => "fill-opacity",
        "stop_color" => "stop-color",
        "stop_opacity" => "stop-opacity",
        "flood_color" => "flood-color",
        "flood_opacity" => "flood-opacity",
        "lighting_color" => "lighting-color",
        "color_interpolation" => "color-interpolation",
        "color_interpolation_filters" => "color-interpolation-filters",
        "pointer_events" => "pointer-events",
        "font_family" => "font-family",
        "font_size" => "font-size",
        "font_size_adjust" => "font-size-adjust",
        "font_style" => "font-style",
        "font_weight" => "font-weight",
        "font_stretch" => "font-stretch",
        "font_variant" => "font-variant",
        "letter_spacing" => "letter-spacing",
        "word_spacing" => "word-spacing",
        "text_anchor" => "text-anchor",
        "text_decoration" => "text-decoration",
        "text_rendering" => "text-rendering",
        "dominant_baseline" => "dominant-baseline",
        "alignment_baseline" => "alignment-baseline",
        "baseline_shift" => "baseline-shift",
        "writing_mode" => "writing-mode",
        "start_offset" => "startOffset",
        "offset" => "offset",
        "text_length" => "textLength",
        "length_adjust" => "lengthAdjust",
        "required_features" => "requiredFeatures",
        "required_extensions" => "requiredExtensions",
        "system_language" => "systemLanguage",
        "filter_units" => "filterUnits",
        "mask_units" => "maskUnits",
        "mask_content_units" => "maskContentUnits",
        "primitive_units" => "primitiveUnits",
        "channel_selector" => "channelSelector",
        "edge_mode" => "edgeMode",
        "in_" => "in",
        "in2" => "in2",
        "x_channel_selector" => "xChannelSelector",
        "y_channel_selector" => "yChannelSelector",
        "stitch_tiles" => "stitchTiles",
        "xlink_href" => "xlink:href",
        "result" => "result",
        "k1" => "k1",
        "k2" => "k2",
        "k3" => "k3",
        "k4" => "k4",

        // ---- _attr suffix stripping (note: `from` is a Rust keyword and
        //       cannot be used as a method name; use `from_attr` for that) ----
        "type_attr" => "type",
        "href_attr" => "href",
        "offset_attr" => "offset",
        "from_attr" => "from",
        "to_attr" => "to",
        "by_attr" => "by",
        "cx_attr" => "cx",
        "cy_attr" => "cy",
        "r_attr" => "r",
        "x_attr" => "x",
        "y_attr" => "y",
        "dx_attr" => "dx",
        "dy_attr" => "dy",
        "width_attr" => "width",
        "height_attr" => "height",
        "rx_attr" => "rx",
        "ry_attr" => "ry",
        "d_attr" => "d",

        // ---- Path and presentation attributes used as-is ----
        "id" => "id",
        "class" => "class",
        "style" => "style",
        "fill" => "fill",
        "stroke" => "stroke",
        "opacity" => "opacity",
        "color" => "color",
        "d" => "d",
        "cx" => "cx",
        "cy" => "cy",
        "r" => "r",
        "rx" => "rx",
        "ry" => "ry",
        "x" => "x",
        "y" => "y",
        "width" => "width",
        "height" => "height",
        "x1" => "x1",
        "y1" => "y1",
        "x2" => "x2",
        "y2" => "y2",
        "dx" => "dx",
        "dy" => "dy",
        "from" => "from",
        "to" => "to",
        "by" => "by",
        "href" => "href",
        "role" => "role",
        "title" => "title",
        "version" => "version",
        "azimuth" => "azimuth",
        "elevation" => "elevation",
        "filter" => "filter",
        "transform" => "transform",
        "mode" => "mode",

        // ---- ARIA identifiers (underscore → dash) ----
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

        // ---- Common globals (no translation) ----
        "tabindex" => "tabindex",
        "lang_global" => "lang",
        "dir" => "dir",
        "hidden" => "hidden",
        "draggable" => "draggable",
        "spellcheck" => "spellcheck",
        "translate" => "translate",
        "contenteditable" => "contenteditable",
        "slot" => "slot",
        "part" => "part",
        "inputmode" => "inputmode",
        "enterkeyhint" => "enterkeyhint",
        "is_content" => "is",

        // ---- Plain names used as-is ----
        "xmlns" => "xmlns",
        "xmlns_xlink" => "xmlns:xlink",
        "hkern" => "hkern",
        "vkern" => "vkern",
        "glyph" => "glyph",
        "missing_glyph" => "missing-glyph",
        "font" => "font",
        "font_face" => "font-face",
        "font_face_src" => "font-face-src",
        "font_face_uri" => "font-face-uri",
        "font_face_format" => "font-face-format",
        "font_face_name" => "font-face-name",
        "horiz_adv_x" => "horiz-adv-x",
        "horiz_origin_x" => "horiz-origin-x",
        "horiz_origin_y" => "horiz-origin-y",
        "vert_adv_y" => "vert-adv-y",
        "vert_origin_x" => "vert-origin-x",
        "vert_origin_y" => "vert-origin-y",
        "underline_position" => "underline-position",
        "underline_thickness" => "underline-thickness",
        "overline_position" => "overline-position",
        "overline_thickness" => "overline-thickness",
        "strike_through_position" => "strike-through-position",
        "strike_through_thickness" => "strike-through-thickness",
        "unicode" => "unicode",
        "glyph_name" => "glyph-name",
        "arabic_form" => "arabic-form",
        "orientation" => "orientation",
        "units_per_em" => "units-per-em",
        "ascent" => "ascent",
        "descent" => "descent",
        "over" => "over",

        // ---- Suffix variants of conflicting names (kept for clarity) ----
        "tabindex_global" => "tabindex",
        "nonce_global" => "nonce",
        "title_global" => "title",
        "autofocus_global" => "autofocus",
        "autocomplete_global" => "autocomplete",
        "list_global" => "list",
        "spellcheck_global" => "spellcheck",
        "form_global" => "form",
        "nonce" => "nonce",

        other => panic!("svg attr_name: unmapped identifier '{other}'"),
    }
}

macro_rules! svg_factory {
    ($(#[$meta:meta])* $fn_name:ident, $type:ident) => {
        $(#[$meta])*
        pub fn $fn_name() -> $type {
            $type::new()
        }
    };
}

pub(crate) use svg_factory;

#[cfg(test)]
mod tests {
    super::define_svg_element!(
        TestCircle,
        "circle",
        cx("X coordinate."),
        cy("Y coordinate."),
        r("Radius.")
    );
    super::define_svg_element!(TestSvgNoAttrs, "svg");

    super::svg_factory!(test_circle, TestCircle);
    super::svg_factory!(test_svg, TestSvgNoAttrs);

    #[test]
    fn svg_attr_name_viewbox_translates_to_camel_case() {
        assert_eq!(super::attr_name("view_box"), "viewBox");
        assert_eq!(
            super::attr_name("preserve_aspect_ratio"),
            "preserveAspectRatio"
        );
        assert_eq!(super::attr_name("gradient_transform"), "gradientTransform");
    }

    #[test]
    fn svg_attr_name_underscore_to_dash() {
        assert_eq!(super::attr_name("stroke_width"), "stroke-width");
        assert_eq!(super::attr_name("font_family"), "font-family");
    }

    #[test]
    fn svg_attr_name_attr_suffix_strip() {
        assert_eq!(super::attr_name("type_attr"), "type");
        assert_eq!(super::attr_name("href_attr"), "href");
    }

    #[test]
    fn svg_attr_name_plain() {
        assert_eq!(super::attr_name("cx"), "cx");
        assert_eq!(super::attr_name("cy"), "cy");
        assert_eq!(super::attr_name("d"), "d");
    }

    #[test]
    fn svg_element_render_with_custom_method() {
        let s = TestCircle::new().cx("50").render();
        assert_eq!(s, r#"<circle cx="50"></circle>"#);
    }

    #[test]
    fn svg_element_render_with_globals() {
        let s = TestSvgNoAttrs::new().id("root").render();
        assert_eq!(s, r#"<svg id="root"></svg>"#);
    }

    #[test]
    fn svg_builder_attr_adds_arbitrary() {
        let el = TestCircle::new().attr("data-test", "value");
        assert_eq!(el.0.attributes.len(), 1);
        assert_eq!(el.0.attributes[0].key, "data-test");
    }

    #[test]
    fn svg_builder_data_attr_adds_prefix() {
        let el = TestCircle::new().data_attr("id", "btn");
        assert_eq!(el.0.attributes.len(), 1);
        assert_eq!(el.0.attributes[0].key, "data-id");
    }

    #[test]
    #[should_panic(expected = "unmapped identifier")]
    fn svg_attr_name_unmapped_identifier_panics() {
        super::attr_name("not_in_table");
    }

    /// Exhaustive coverage: every common-global method on the SVG
    /// `TestCircle`/`TestSvgNoAttrs` to ensure the body lines of
    /// `__common_globals_methods!` are reachable through the macro
    /// expansion at runtime.
    #[test]
    fn svg_globals_method_bodies_covered() {
        let _ = TestCircle::new()
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
            .autocomplete_global("on")
            .aria_label("l")
            .aria_hidden("t")
            .aria_role("b")
            .aria_live("polite")
            .aria_expanded("false")
            .aria_selected("true")
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

    /// Exhaustive coverage: every event handler body in
    /// `__event_handlers_methods!` for SVG.
    #[test]
    fn svg_event_handlers_bodies_covered() {
        let _ = TestCircle::new()
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
    /// `__aria_all_methods!` for SVG.
    #[test]
    fn svg_aria_all_methods_bodies_covered() {
        use crate::attributes::attr;
        let el = TestSvgNoAttrs::new().attrs(vec![
            attr("aria-label").value("l"),
            attr("aria-hidden").value("true"),
            attr("role").value("button"),
            attr("aria-live").value("polite"),
            attr("aria-expanded").value("false"),
            attr("aria-selected").value("true"),
        ]);
        assert_eq!(el.0.attributes.len(), 6);
    }

    /// Verify the per-setter expansion by calling each SVG ARIA
    /// setter in isolation.
    #[test]
    fn svg_aria_all_methods_individual_expansion() {
        assert_eq!(TestSvgNoAttrs::new().aria_label("l").0.attributes[0].key, "aria-label");
        assert_eq!(TestSvgNoAttrs::new().aria_hidden("t").0.attributes[0].key, "aria-hidden");
        assert_eq!(TestSvgNoAttrs::new().aria_role("b").0.attributes[0].key, "role");
        assert_eq!(TestSvgNoAttrs::new().aria_live("polite").0.attributes[0].key, "aria-live");
        assert_eq!(TestSvgNoAttrs::new().aria_expanded("false").0.attributes[0].key, "aria-expanded");
        assert_eq!(TestSvgNoAttrs::new().aria_selected("true").0.attributes[0].key, "aria-selected");
    }

    /// Coverage for `__emitted_custom_method!` on SVG.
    #[test]
    fn svg_emitted_custom_method_body_covered() {
        use crate::attributes::attr;
        let el = TestCircle::new().attrs(vec![
            attr("cx").value("1"),
            attr("cy").value("2"),
            attr("r").value("3"),
        ]);
        assert_eq!(el.0.attributes.len(), 3);
    }

    /// Verify the per-setter expansion by calling each emitted
    /// custom method in isolation.
    #[test]
    fn svg_emitted_custom_method_individual_expansion() {
        assert_eq!(TestCircle::new().cx("1").0.attributes[0].key, "cx");
        assert_eq!(TestCircle::new().cy("2").0.attributes[0].key, "cy");
        assert_eq!(TestCircle::new().r("3").0.attributes[0].key, "r");
    }

    /// Coverage for `__from_impls!` on SVG.
    #[test]
    fn svg_from_impls_bodies_covered() {
        let s = TestSvgNoAttrs::new();
        let _e: crate::element::Element = s.into();
        let s2 = TestSvgNoAttrs::new();
        let _n: crate::node::Node = s2.into();
        let s3 = TestSvgNoAttrs::new();
        let _r = crate::renderable::Renderable::render(&s3);
    }

    /// Exercise the `Default::default()` impl produced by
    /// `__define_struct` so its body line (`Self::new()`) is reached.
    #[test]
    fn svg_default_impl_runs_new() {
        let s: TestCircle = TestCircle::default();
        assert_eq!(s.0.name, "circle");
    }

    /// Exercise `__builder_methods!` — in particular the
    /// `children()` setter body line.
    #[test]
    fn svg_builder_methods_bodies_covered() {
        use crate::node::Node;
        let s = TestSvgNoAttrs::new().children(vec![Node::Text("hi".into())]);
        assert_eq!(s.0.children.len(), 1);
    }

    /// Exhaustively call `attr_name` for every identifier defined in the
    /// SVG lookup table. Guards against regressions when an element
    /// references a method whose identifier has been forgotten.
    #[test]
    fn svg_attr_name_covers_every_identifier() {
        let identifiers: &[&str] = &[
            // CamelCase SVG attribute names.
            "view_box",
            "preserve_aspect_ratio",
            "gradient_transform",
            "gradient_units",
            "spread_method",
            "pattern_units",
            "pattern_content_units",
            "pattern_transform",
            "clip_path",
            "clip_path_units",
            "marker_units",
            "marker_width",
            "marker_height",
            "ref_x",
            "ref_y",
            "ref_width",
            "ref_height",
            "std_deviation",
            "base_frequency",
            "num_octaves",
            "target_x",
            "target_y",
            "kernel_unit_length",
            "kernel_matrix",
            "divisor",
            "bias",
            "specular_constant",
            "specular_exponent",
            "limiting_cone_angle",
            "points_at_x",
            "points_at_y",
            "points_at_z",
            "surface_scale",
            "diffuse_constant",
            "table_values",
            "attribute_name",
            "attribute_type",
            "calc_mode",
            "key_times",
            "key_splines",
            "key_points",
            "dur",
            "begin",
            "end",
            "restart_attr",
            "restart_count",
            "repeat_count",
            "repeat_dur",
            "begin_offset",
            "begin_event_count",
            "end_offset",
            "end_event_count",
            "active_duration",
            "simple_result",
            "upper_limit",
            "lower_limit",
            "sync_behavior",
            "sync_tolerance",
            "preserve_alpha",
            // Underscore → dash mappings.
            "stroke_width",
            "stroke_dasharray",
            "stroke_dashoffset",
            "stroke_linecap",
            "stroke_linejoin",
            "stroke_miterlimit",
            "stroke_opacity",
            "fill_rule",
            "fill_opacity",
            "stop_color",
            "stop_opacity",
            "flood_color",
            "flood_opacity",
            "lighting_color",
            "color_interpolation",
            "color_interpolation_filters",
            "pointer_events",
            "font_family",
            "font_size",
            "font_size_adjust",
            "font_style",
            "font_weight",
            "font_stretch",
            "font_variant",
            "letter_spacing",
            "word_spacing",
            "text_anchor",
            "text_decoration",
            "text_rendering",
            "dominant_baseline",
            "alignment_baseline",
            "baseline_shift",
            "writing_mode",
            "start_offset",
            "offset",
            "text_length",
            "length_adjust",
            "required_features",
            "required_extensions",
            "system_language",
            "filter_units",
            "mask_units",
            "mask_content_units",
            "primitive_units",
            "edge_mode",
            "in_",
            "in2",
            "channel_selector",
            "x_channel_selector",
            "y_channel_selector",
            "stitch_tiles",
            "xlink_href",
            "result",
            "k1",
            "k2",
            "k3",
            "k4",
            "auto_reverse",
            "additive_attr",
            "accumulate_attr",
            // _attr-suffix variants.
            "type_attr",
            "href_attr",
            "offset_attr",
            "from_attr",
            "to_attr",
            "by_attr",
            "cx_attr",
            "cy_attr",
            "r_attr",
            "x_attr",
            "y_attr",
            "dx_attr",
            "dy_attr",
            "width_attr",
            "height_attr",
            "rx_attr",
            "ry_attr",
            "d_attr",
            // Plain names.
            "id",
            "class",
            "style",
            "fill",
            "stroke",
            "opacity",
            "color",
            "d",
            "cx",
            "cy",
            "r",
            "rx",
            "ry",
            "x",
            "y",
            "width",
            "height",
            "x1",
            "y1",
            "x2",
            "y2",
            "dx",
            "dy",
            "from",
            "to",
            "by",
            "href",
            "role",
            "title",
            "version",
            "azimuth",
            "elevation",
            "filter",
            "transform",
            "mode",
            // ARIA identifiers.
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
            // Common globals.
            "tabindex",
            "lang_global",
            "dir",
            "hidden",
            "draggable",
            "spellcheck",
            "translate",
            "contenteditable",
            "slot",
            "part",
            "inputmode",
            "enterkeyhint",
            "is_content",
            // Suffix variants of conflicting names.
            "tabindex_global",
            "nonce_global",
            "title_global",
            "autofocus_global",
            "autocomplete_global",
            "list_global",
            "spellcheck_global",
            "form_global",
            "nonce",
            // Font + XML plain names.
            "xmlns",
            "xmlns_xlink",
            "hkern",
            "vkern",
            "glyph",
            "missing_glyph",
            "font",
            "font_face",
            "font_face_src",
            "font_face_uri",
            "font_face_format",
            "font_face_name",
            "horiz_adv_x",
            "horiz_origin_x",
            "horiz_origin_y",
            "vert_adv_y",
            "vert_origin_x",
            "vert_origin_y",
            "underline_position",
            "underline_thickness",
            "overline_position",
            "overline_thickness",
            "strike_through_position",
            "strike_through_thickness",
            "unicode",
            "glyph_name",
            "arabic_form",
            "orientation",
            "units_per_em",
            "ascent",
            "descent",
            "over",
            // Special names from per-element definitions.
            "over",
            "marker_start",
            "marker_mid",
            "marker_end",
            "fill_attr",
        ];
        for ident in identifiers {
            let mapped = super::attr_name(ident);
            assert!(!mapped.is_empty(), "attr_name({ident}) returned empty");
        }
    }
}
