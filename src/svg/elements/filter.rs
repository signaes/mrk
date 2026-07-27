//! Filter primitives (`feBlend`, `feColorMatrix`, etc.) and top-level
//! `<filter>` container.

use super::macros::{define_svg_element, svg_factory};

define_svg_element!(SvgFilter, "filter",
    x(r#"X coordinate of the filter region (default `-10%`)."#),
    y(r#"Y coordinate of the filter region (default `-10%`)."#),
    width(r#"Width of the filter region (default `120%`)."#),
    height(r#"Height of the filter region (default `120%`)."#),
    filter_units(r#"Units used for `x`/`y`/`width`/`height` of the filter region.

One of `userSpaceOnUse` (default) or `objectBoundingBox`."#),
    primitive_units(r#"Units for any child primitive's coordinates.

One of `userSpaceOnUse` (default) or `objectBoundingBox`."#));

define_svg_element!(SvgFeBlend, "feBlend",
    in_(r#"Identifier of the source graphic (the first operand of the blend).

For composition with `in2`, the `<filter>` chain must define an input with
this identifier."#),
    in2(r#"Identifier of the second operand of the blend."#),
    mode(r#"Blend mode to apply between the two input graphics.

One of `normal`, `multiply`, `screen`, `darken`, `lighten`, `overlay`,
`color-dodge`, `color-burn`, `hard-light`, `soft-light`, `difference`,
`exclusion`, `hue`, `saturation`, `color`, `luminosity`."#),
    result(r#"Identifier under which the filter primitive's output is exposed."#));

define_svg_element!(SvgFeColorMatrix, "feColorMatrix",
    in_(r#"Input graphic identifier."#),
    r#type(r#"Color matrix interpretation mode (note: the method name is
`type_attr` in this wrapper to avoid the Rust keyword).

One of `matrix`, `saturate`, `hueRotate`, `luminanceToAlpha`."#),
    values(r#"Comma/space-separated matrix values (a `matrix(…)` 4×5 or
single numeric value depending on `type`)."#),
    result(r#"Output identifier."#));

define_svg_element!(SvgFeComponentTransfer, "feComponentTransfer",
    in_(r#"Input graphic identifier."#),
    result(r#"Output identifier."#));

define_svg_element!(SvgFeComposite, "feComposite",
    in_(r#"First operand of the composite operator."#),
    in2(r#"Second operand."#),
    operator(r#"Composite operator (one of `over`, `in`, `out`, `atop`,
`xor`, `arithmetic`)."#),
    k1(r#"Arithmetic `k1` coefficient (default `0`)."#),
    k2(r#"Arithmetic `k2` coefficient (default `0`)."#),
    k3(r#"Arithmetic `k3` coefficient (default `0`)."#),
    k4(r#"Arithmetic `k4` coefficient (default `0`)."#),
    result(r#"Output identifier."#));

define_svg_element!(SvgFeConvolveMatrix, "feConvolveMatrix",
    in_(r#"Input graphic identifier."#),
    kernel_matrix(r#"Comma- or space-separated values for the convolution kernel."#),
    divisor(r#"Constant divisor for the convolution (default `1`).

Use a non-zero value to keep the output in a reasonable range."#),
    bias(r#"Constant bias added to each channel after convolution (default `0`)."#),
    target_x(r#"X offset of the kernel matrix (positive integer)."#),
    target_y(r#"Y offset of the kernel matrix (positive integer)."#),
    edge_mode(r#"Behavior at the edges of the input.

One of `duplicate`, `wrap`, `none` (default)."#),
    kernel_unit_length(r#"Length units for `dx`/`dy` of the kernel (in user units)."#),
    preserve_alpha(r#"Whether to keep alpha unchanged.

`true` keeps alpha intact; `false` (default) processes alpha as well."#),
    order(r#"Number of kernel rows/columns as `<cols>,<rows>`."#),
    result(r#"Output identifier."#));

define_svg_element!(SvgFeDiffuseLighting, "feDiffuseLighting",
    in_(r#"Input graphic identifier (typically a normal map)."#),
    surface_scale(r#"Surface scaling factor for the diffuse lighting (default `1`)."#),
    diffuse_constant(r#"Diffuse constant `kd` (default `1`)."#),
    kernel_unit_length(r#"Length units for kernel sampling in user space."#),
    result(r#"Output identifier."#));

define_svg_element!(SvgFeDisplacementMap, "feDisplacementMap",
    in_(r#"Input graphic to displace."#),
    in2(r#"Displacement map input (per-pixel offset data)."#),
    scale(r#"Displacement scale factor (a `<number>`).

Positive values displace in the same direction as the map; negative values
invert."#),
    x_channel_selector(r#"Color channel from the displacement map used for X.

One of `A`, `R`, `G`, `B` (default `A`)."#),
    y_channel_selector(r#"Color channel from the displacement map used for Y.

One of `A`, `R`, `G`, `B` (default `A`)."#),
    result(r#"Output identifier."#));

define_svg_element!(SvgFeDistantLight, "feDistantLight",
    azimuth(r#"Light direction's azimuth angle in degrees."#),
    elevation(r#"Light direction's elevation angle in degrees."#));

define_svg_element!(SvgFeFlood, "feFlood",
    flood_color(r#"Fill color of the flood rectangle (default `black`)."#),
    flood_opacity(r#"Opacity of the flood rectangle (default `1`)."#),
    result(r#"Output identifier."#));

define_svg_element!(SvgFeFuncA, "feFuncA",
    table_values(r#"Tabulated values for the alpha transfer function.

A list of numbers the input alpha is mapped through."#),
    slope(r#"Linear slope of the transfer function."#),
    intercept(r#"Linear intercept."#),
    amplitude(r#"Amplitude of the periodic transfer function."#),
    exponent(r#"Exponent of the periodic transfer function."#),
    offset(r#"Constant offset added to the result."#));

define_svg_element!(SvgFeFuncR, "feFuncR",
    table_values(r#"Tabulated values for the red transfer function."#),
    slope(r#"Linear slope."#),
    intercept(r#"Linear intercept."#),
    amplitude(r#"Periodic amplitude."#),
    exponent(r#"Periodic exponent."#),
    offset(r#"Constant offset."#));

define_svg_element!(SvgFeFuncG, "feFuncG",
    table_values(r#"Tabulated values for the green transfer function."#),
    slope(r#"Linear slope."#),
    intercept(r#"Linear intercept."#),
    amplitude(r#"Periodic amplitude."#),
    exponent(r#"Periodic exponent."#),
    offset(r#"Constant offset."#));

define_svg_element!(SvgFeFuncB, "feFuncB",
    table_values(r#"Tabulated values for the blue transfer function."#),
    slope(r#"Linear slope."#),
    intercept(r#"Linear intercept."#),
    amplitude(r#"Periodic amplitude."#),
    exponent(r#"Periodic exponent."#),
    offset(r#"Constant offset."#));

define_svg_element!(SvgFeGaussianBlur, "feGaussianBlur",
    in_(r#"Input graphic identifier."#),
    std_deviation(r#"Standard deviation of the blur (one or two values: `x[,y]`).

Each value is a `<number>` (interpreted as user units) or `0`."#),
    edge_mode(r#"Behavior at the edges.

One of `duplicate` (default), `wrap`, or `none`."#),
    result(r#"Output identifier."#));

define_svg_element!(SvgFeImage, "feImage",
    href(r#"URL of the image to load into the filter pipeline.

A data URL or HTTP(S) URL pointing to an image resource."#),
    result(r#"Output identifier."#),
    preserve_aspect_ratio(r#"Aspect-ratio handling for the image."#));

define_svg_element!(SvgFeMerge, "feMerge",
    result(r#"Output identifier."#));

define_svg_element!(SvgFeMergeNode, "feMergeNode",
    in_(r#"Input graphic identifier to merge."#));

define_svg_element!(SvgFeMorphology, "feMorphology",
    in_(r#"Input graphic identifier."#),
    operator(r#"Morphology operator: `erode` or `dilate` (default `erode`)."#),
    radius(r#"Effect radius (default `0`). Two values for X/Y radius: `<rx>,<ry>`."#),
    result(r#"Output identifier."#));

define_svg_element!(SvgFeOffset, "feOffset",
    in_(r#"Input graphic identifier."#),
    dx(r#"Horizontal offset (positive `<length>`; default `0`)."#),
    dy(r#"Vertical offset (positive `<length>`; default `0`)."#),
    result(r#"Output identifier."#));

define_svg_element!(SvgFePointLight, "fePointLight",
    x(r#"X coordinate of the light source in user space."#),
    y(r#"Y coordinate of the light source in user space."#),
    z(r#"Z coordinate of the light source in user space."#));

define_svg_element!(SvgFeSpecularLighting, "feSpecularLighting",
    in_(r#"Input graphic identifier."#),
    surface_scale(r#"Surface scaling factor (default `1`)."#),
    specular_constant(r#"Specular constant `ks` (default `1`)."#),
    specular_exponent(r#"Specular exponent `phong` (default `1`)."#),
    kernel_unit_length(r#"Length units for kernel sampling."#),
    result(r#"Output identifier."#));

define_svg_element!(SvgFeSpotLight, "feSpotLight",
    x(r#"X coordinate of the light source."#),
    y(r#"Y coordinate of the light source."#),
    z(r#"Z coordinate of the light source."#),
    points_at_x(r#"X coordinate that the spotlight points at."#),
    points_at_y(r#"Y coordinate that the spotlight points at."#),
    points_at_z(r#"Z coordinate that the spotlight points at."#),
    specular_exponent(r#"Spotlight cone falloff exponent."#),
    limiting_cone_angle(r#"Limiting cone angle in degrees."#));

define_svg_element!(SvgFeTile, "feTile",
    in_(r#"Input graphic identifier."#),
    result(r#"Output identifier."#));

define_svg_element!(SvgFeTurbulence, "feTurbulence",
    base_frequency(r#"Base frequency of the turbulence noise.

Either a single value (same for both axes) or `<x>,<y>`."#),
    num_octaves(r#"Number of noise octaves (a positive integer, default `1`)."#),
    seed(r#"Integer seed for the pseudo-random noise generator (default `0`)."#),
    stitch_tiles(r#"How the turbulence tiles at the edges.

One of `stitch` (default) or `noStitch`."#),
    r#type(r#"Fractal type: `fractalNoise` (default) or `fractalSum`."#),
    result(r#"Output identifier."#));

// Create factories.
svg_factory!(filter, SvgFilter);
svg_factory!(fe_blend, SvgFeBlend);
svg_factory!(fe_color_matrix, SvgFeColorMatrix);
svg_factory!(fe_component_transfer, SvgFeComponentTransfer);
svg_factory!(fe_composite, SvgFeComposite);
svg_factory!(fe_convolve_matrix, SvgFeConvolveMatrix);
svg_factory!(fe_diffuse_lighting, SvgFeDiffuseLighting);
svg_factory!(fe_displacement_map, SvgFeDisplacementMap);
svg_factory!(fe_distant_light, SvgFeDistantLight);
svg_factory!(fe_flood, SvgFeFlood);
svg_factory!(fe_func_a, SvgFeFuncA);
svg_factory!(fe_func_r, SvgFeFuncR);
svg_factory!(fe_func_g, SvgFeFuncG);
svg_factory!(fe_func_b, SvgFeFuncB);
svg_factory!(fe_gaussian_blur, SvgFeGaussianBlur);
svg_factory!(fe_image, SvgFeImage);
svg_factory!(fe_merge, SvgFeMerge);
svg_factory!(fe_merge_node, SvgFeMergeNode);
svg_factory!(fe_morphology, SvgFeMorphology);
svg_factory!(fe_offset, SvgFeOffset);
svg_factory!(fe_point_light, SvgFePointLight);
svg_factory!(fe_specular_lighting, SvgFeSpecularLighting);
svg_factory!(fe_spot_light, SvgFeSpotLight);
svg_factory!(fe_tile, SvgFeTile);
svg_factory!(fe_turbulence, SvgFeTurbulence);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderable::Renderable;

    #[test]
    fn filter_no_attrs() {
        assert_eq!(filter().render(), "<filter></filter>");
    }

    #[test]
    fn fe_blend_attrs() {
        assert_eq!(
            fe_blend().mode("multiply").render(),
            r#"<feBlend mode="multiply"></feBlend>"#
        );
    }

    #[test]
    fn fe_gaussian_blur_attrs() {
        assert_eq!(
            fe_gaussian_blur().std_deviation("2").render(),
            r#"<feGaussianBlur stdDeviation="2"></feGaussianBlur>"#
        );
    }

    #[test]
    fn fe_offset_attrs() {
        assert_eq!(
            fe_offset().dx("10").render(),
            r#"<feOffset dx="10"></feOffset>"#
        );
    }

    #[test]
    fn fe_turbulence_attrs() {
        assert_eq!(
            fe_turbulence().base_frequency("0.05").render(),
            r#"<feTurbulence baseFrequency="0.05"></feTurbulence>"#
        );
    }
}
