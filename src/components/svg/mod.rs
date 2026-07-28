//! Typed SVG wrappers for the `components` module.
//!
//! This sub-module is **independent** of `mrk::svg` — the wrappers
//! here are defined separately and have their own attribute setters.
//! Each wrapper accepts `impl IntoExpr` for dynamic values.
//!
//! # Usage
//!
//! ```ignore
//! use mrk::components::svg::{circle, rect};
//!
//! component!(Shape, {
//!     circle().cx(prop("cx")).cy(prop("cy")).r(prop("r"))
//! });
//! ```
#![allow(missing_docs)]
#![allow(non_snake_case)]

use crate::components::IntoExpr;

/// Internal: define a typed SVG wrapper for a tag. Each wrapper
/// exposes the most common SVG attributes as methods, all accepting
/// `impl IntoExpr` for dynamic values.
#[macro_export]
#[doc(hidden)]
macro_rules! __component_svg_define {
    ($struct:ident, $factory:ident, $tag:literal $(,)?) => {
        $crate::components::element::__define_component_wrapper!(
            $struct,
            $factory,
            $tag,
            // Most common SVG attributes (lower-case to dash form)
            href => "href",
            xlink_href => "xlink:href",
            transform => "transform",
            view_box => "viewBox",
            preserve_aspect_ratio => "preserveAspectRatio",
            d => "d",
            cx => "cx",
            cy => "cy",
            r => "r",
            rx => "rx",
            ry => "ry",
            x => "x",
            y => "y",
            x1 => "x1",
            y1 => "y1",
            x2 => "x2",
            y2 => "y2",
            width => "width",
            height => "height",
            dx => "dx",
            dy => "dy",
            fill => "fill",
            fill_rule => "fill-rule",
            fill_opacity => "fill-opacity",
            stroke => "stroke",
            stroke_width => "stroke-width",
            stroke_dasharray => "stroke-dasharray",
            stroke_dashoffset => "stroke-dashoffset",
            stroke_linecap => "stroke-linecap",
            stroke_linejoin => "stroke-linejoin",
            stroke_miterlimit => "stroke-miterlimit",
            stroke_opacity => "stroke-opacity",
            opacity => "opacity",
            color => "color",
            gradient_transform => "gradientTransform",
            gradient_units => "gradientUnits",
            spread_method => "spreadMethod",
            pattern_units => "patternUnits",
            pattern_content_units => "patternContentUnits",
            pattern_transform => "patternTransform",
            clip_path_units => "clipPathUnits",
            clip_path => "clip-path",
            marker_units => "markerUnits",
            marker_width => "markerWidth",
            marker_height => "markerHeight",
            ref_x => "refX",
            ref_y => "refY",
            ref_width => "refWidth",
            ref_height => "refHeight",
            marker_start => "marker-start",
            marker_mid => "marker-mid",
            marker_end => "marker-end",
            std_deviation => "stdDeviation",
            base_frequency => "baseFrequency",
            num_octaves => "numOctaves",
            target_x => "targetX",
            target_y => "targetY",
            kernel_unit_length => "kernelUnitLength",
            kernel_matrix => "kernelMatrix",
            divisor => "divisor",
            bias => "bias",
            specular_constant => "specularConstant",
            specular_exponent => "specularExponent",
            limiting_cone_angle => "limitingConeAngle",
            points_at_x => "pointsAtX",
            points_at_y => "pointsAtY",
            points_at_z => "pointsAtZ",
            surface_scale => "surfaceScale",
            diffuse_constant => "diffuseConstant",
            table_values => "tableValues",
            attribute_name => "attributeName",
            attribute_type => "attributeType",
            calc_mode => "calcMode",
            key_times => "keyTimes",
            key_splines => "keySplines",
            key_points => "keyPoints",
            dur => "dur",
            begin => "begin",
            end => "end",
            restart => "restart",
            repeat_count => "repeatCount",
            repeat_dur => "repeatDur",
            fill_anim => "fill",
            auto_reverse => "autoReverse",
            accumulate => "accumulate",
            additive => "additive",
            begin_offset => "beginOffset",
            end_offset => "endOffset",
            upper_limit => "upperLimit",
            lower_limit => "lowerLimit",
            sync_behavior => "syncBehavior",
            sync_tolerance => "syncTolerance",
            preserve_alpha => "preserveAlpha",
            font_family => "font-family",
            font_size => "font-size",
            font_style => "font-style",
            font_weight => "font-weight",
            text_anchor => "text-anchor",
            dominant_baseline => "dominant-baseline",
            alignment_baseline => "alignment-baseline",
            start_offset => "startOffset",
            text_length => "textLength",
            length_adjust => "lengthAdjust",
            required_features => "requiredFeatures",
            required_extensions => "requiredExtensions",
            system_language => "systemLanguage",
            filter_units => "filterUnits",
            mask_units => "maskUnits",
            mask_content_units => "maskContentUnits",
            primitive_units => "primitiveUnits",
            channel_selector => "channelSelector",
            edge_mode => "edgeMode",
            in_ => "in",
            in2 => "in2",
            x_channel_selector => "xChannelSelector",
            y_channel_selector => "yChannelSelector",
            stitch_tiles => "stitchTiles",
            result => "result",
            k1 => "k1",
            k2 => "k2",
            k3 => "k3",
            k4 => "k4",
            version => "version",
            xmlns_attr => "xmlns",
            xmlns_xlink => "xmlns:xlink",
            filter => "filter",
            mask => "mask",
            clip_path_attr => "clip-path",
            mode => "mode",
            path => "d",
        );
    };
}

// ============================================================================
// Container
// ============================================================================

__component_svg_define!(Svg, svg, "svg");
__component_svg_define!(G, g, "g");
__component_svg_define!(Defs, defs, "defs");
__component_svg_define!(Symbol, symbol, "symbol");
__component_svg_define!(Use, use_, "use");
__component_svg_define!(A, a, "a");
__component_svg_define!(Switch, switch, "switch");
__component_svg_define!(Marker, marker, "marker");

// ============================================================================
// Shapes
// ============================================================================

__component_svg_define!(Circle, circle, "circle");
__component_svg_define!(Ellipse, ellipse, "ellipse");
__component_svg_define!(Line, line, "line");
__component_svg_define!(Polyline, polyline, "polyline");
__component_svg_define!(Polygon, polygon, "polygon");
__component_svg_define!(Rect, rect, "rect");
__component_svg_define!(Path, path, "path");

// ============================================================================
// Text
// ============================================================================

__component_svg_define!(Text, text, "text");
__component_svg_define!(Tspan, tspan, "tspan");
__component_svg_define!(TextPath, textPath, "textPath");

// ============================================================================
// Descriptive
// ============================================================================

__component_svg_define!(Title, title, "title");
__component_svg_define!(Desc, desc, "desc");
__component_svg_define!(Metadata, metadata, "metadata");
__component_svg_define!(ForeignObject, foreignObject, "foreignObject");

// ============================================================================
// Gradient
// ============================================================================

__component_svg_define!(LinearGradient, linearGradient, "linearGradient");
__component_svg_define!(RadialGradient, radialGradient, "radialGradient");
__component_svg_define!(Stop, stop, "stop");

// ============================================================================
// Pattern
// ============================================================================

__component_svg_define!(Pattern, pattern, "pattern");

// ============================================================================
// Mask
// ============================================================================

__component_svg_define!(Mask, mask, "mask");
__component_svg_define!(ClipPath, clipPath, "clipPath");

// ============================================================================
// Filter
// ============================================================================

__component_svg_define!(Filter, filter, "filter");
__component_svg_define!(FeBlend, feBlend, "feBlend");
__component_svg_define!(FeColorMatrix, feColorMatrix, "feColorMatrix");
__component_svg_define!(FeComponentTransfer, feComponentTransfer, "feComponentTransfer");
__component_svg_define!(FeComposite, feComposite, "feComposite");
__component_svg_define!(FeConvolveMatrix, feConvolveMatrix, "feConvolveMatrix");
__component_svg_define!(FeDiffuseLighting, feDiffuseLighting, "feDiffuseLighting");
__component_svg_define!(FeDisplacementMap, feDisplacementMap, "feDisplacementMap");
__component_svg_define!(FeDistantLight, feDistantLight, "feDistantLight");
__component_svg_define!(FeFlood, feFlood, "feFlood");
__component_svg_define!(FeFuncA, feFuncA, "feFuncA");
__component_svg_define!(FeFuncR, feFuncR, "feFuncR");
__component_svg_define!(FeFuncG, feFuncG, "feFuncG");
__component_svg_define!(FeFuncB, feFuncB, "feFuncB");
__component_svg_define!(FeGaussianBlur, feGaussianBlur, "feGaussianBlur");
__component_svg_define!(FeImage, feImage, "feImage");
__component_svg_define!(FeMerge, feMerge, "feMerge");
__component_svg_define!(FeMergeNode, feMergeNode, "feMergeNode");
__component_svg_define!(FeMorphology, feMorphology, "feMorphology");
__component_svg_define!(FeOffset, feOffset, "feOffset");
__component_svg_define!(FePointLight, fePointLight, "fePointLight");
__component_svg_define!(FeSpecularLighting, feSpecularLighting, "feSpecularLighting");
__component_svg_define!(FeSpotLight, feSpotLight, "feSpotLight");
__component_svg_define!(FeTile, feTile, "feTile");
__component_svg_define!(FeTurbulence, feTurbulence, "feTurbulence");

// ============================================================================
// Animation
// ============================================================================

__component_svg_define!(Animate, animate, "animate");
__component_svg_define!(Set, set, "set");
__component_svg_define!(AnimateTransform, animateTransform, "animateTransform");
__component_svg_define!(AnimateMotion, animateMotion, "animateMotion");

// ============================================================================
// Font
// ============================================================================

__component_svg_define!(Font, font, "font");
__component_svg_define!(FontFace, fontFace, "font-face");
__component_svg_define!(FontFaceSrc, fontFaceSrc, "font-face-src");
__component_svg_define!(FontFaceUri, fontFaceUri, "font-face-uri");
__component_svg_define!(FontFaceFormat, fontFaceFormat, "font-face-format");
__component_svg_define!(FontFaceName, fontFaceName, "font-face-name");
__component_svg_define!(MissingGlyph, missingGlyph, "missing-glyph");
__component_svg_define!(Glyph, glyph, "glyph");
__component_svg_define!(HKern, hkern, "hkern");
__component_svg_define!(VKern, vkern, "vkern");