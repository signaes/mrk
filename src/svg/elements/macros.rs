//! SVG macro: `define_svg_element!` and the SVG-specific `attr_name`
//! table.
//!
//! Wraps the helpers in [`crate::shared_macros`] and adds SVG-specific
//! `attr_name` lookup rules. All generated wrappers use the `all`
//! globals tier per SVG 2.

macro_rules! define_svg_element {
    ($name:ident, $tag:literal) => {
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

    ($name:ident, $tag:literal, $($method:ident($doc:literal)),+ $(,)?) => {
        $crate::shared_macros::__define_struct!($name);
        impl $name {
            $crate::shared_macros::__new_method!($tag);
            $crate::shared_macros::__builder_methods!();
            $(
                $crate::shared_macros::__emitted_custom_method!(
                    $method, $doc,
                    $crate::svg::elements::macros::attr_name
                );
            )+
            $crate::shared_macros::__common_globals_methods!();
            $crate::shared_macros::__event_handlers_methods!();
            $crate::shared_macros::__aria_all_methods!();
        }
        $crate::shared_macros::__from_impls!($name);
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
        "data_x" => "data-x",
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
    use crate::renderable::Renderable;

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
        let s = test_circle().cx("50").render();
        assert_eq!(s, r#"<circle cx="50"></circle>"#);
    }

    #[test]
    fn svg_element_render_with_globals() {
        let s = test_svg().id("root").render();
        assert_eq!(s, r#"<svg id="root"></svg>"#);
    }

    #[test]
    #[should_panic(expected = "unmapped identifier")]
    fn svg_attr_name_unmapped_identifier_panics() {
        super::attr_name("not_in_table");
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
            "data_x",
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
