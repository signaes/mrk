//! Gradient elements: `<linearGradient>`, `<radialGradient>`, `<stop>`.

use super::macros::{define_svg_element, svg_factory};

define_svg_element!(SvgLinearGradient, "linearGradient",
    x1(r#"X coordinate of the first point defining the gradient direction.

By default the gradient is horizontal, from `x1=0%` to `x2=100%`."#),
    y1(r#"Y coordinate of the first point (default `0%`)."#),
    x2(r#"X coordinate of the second point (default `100%`)."#),
    y2(r#"Y coordinate of the second point (default `0%`)."#),
    gradient_units(r#"Units used for the gradient vector.

One of `objectBoundingBox` (default) or `userSpaceOnUse`."#),
    spread_method(r#"Method to use outside the gradient.

One of `pad` (default), `reflect`, `repeat`."#),
    gradient_transform(r#"Transform applied to the gradient.

A list of transform functions (e.g. `rotate(45)`)."#),
    href(r#"Reference to another gradient from which to inherit stops."#));

define_svg_element!(SvgRadialGradient, "radialGradient",
    cx(r#"X coordinate of the gradient center (default `50%`)."#),
    cy(r#"Y coordinate of the gradient center (default `50%`)."#),
    r(r#"Radius of the gradient (default `50%`)."#),
    fx(r#"X coordinate of the focal point (default `cx`)."#),
    fy(r#"Y coordinate of the focal point (default `cy`)."#),
    fr(r#"Focal point radius (default `0`)."#),
    gradient_units(r#"Units for the gradient geometry."#),
    spread_method(r#"Method to use outside the gradient.

One of `pad`, `reflect`, `repeat`."#),
    gradient_transform(r#"Transform applied to the gradient."#),
    href(r#"Reference to another gradient from which to inherit stops."#));

define_svg_element!(SvgStop, "stop",
    offset(r#"Stop's position along the gradient (a `<number>` or `<percentage>`).

`0` represents the gradient's start; `1` (or `100%`) the end. Values in
between map interpolations."#),
    stop_color(r#"Color of the stop (default `black`)."#),
    stop_opacity(r#"Opacity of the stop (default `1`)."#));

svg_factory!(
    /// Create a new [`SvgLinearGradient`] element (`<linearGradient>`).
    linear_gradient, SvgLinearGradient
);
svg_factory!(
    /// Create a new [`SvgRadialGradient`] element (`<radialGradient>`).
    radial_gradient, SvgRadialGradient
);
svg_factory!(
    /// Create a new [`SvgStop`] element (`<stop>`).
    stop, SvgStop
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderable::Renderable;

    #[test]
    fn linear_gradient_attrs() {
        linear_gradient().x1("0%").render();
    }

    #[test]
    fn radial_gradient_attrs() {
        radial_gradient().cx("50%").render();
    }

    #[test]
    fn stop_attrs() {
        stop().offset("0%").render();
    }
}
