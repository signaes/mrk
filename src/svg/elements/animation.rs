//! Animation elements (`<animate>`, `<set>`, `<animateTransform>`,
//! `<animateMotion>`).

use super::macros::{define_svg_element, svg_factory};

define_svg_element!(SvgAnimate, "animate",
    attribute_name(r#"Name of the target attribute to animate (e.g. `cx`, `r`)."#),
    attribute_type(r#"Type of the target attribute (one of `CSS`, `XML`, `auto`)."#),
    from_attr(r#"Initial value of the animated attribute (same type as the attribute)."#),
    to_attr(r#"Final value of the animated attribute."#),
    by_attr(r#"Relative offset added to the animated attribute's value."#),
    values(r#"Semicolon-separated list of values the attribute cycles through."#),
    key_times(r#"Semicolon-separated time fractions (0..1) matching `values`."#),
    key_splines(r#"Bezier control points (e.g. `0.5 0 0.5 1`) matching `key_times`."#),
    calc_mode(r#"Interpolation mode: `discrete`, `linear` (default), `paced`, `spline`."#),
    dur(r#"Duration of one animation cycle (e.g. `3s`, `indefinite`)."#),
    begin(r#"Begin time (offset or event reference)."#),
    end(r#"End time."#),
    min(r#"Minimum duration (advances the end time if active duration is shorter)."#),
    max(r#"Maximum duration."#),
    restart_attr(r#"Restart behavior: `always`, `whenNotActive`, `never`."#),
    repeat_count(r#"Number of repetitions (a number or `indefinite`)."#),
    repeat_dur(r#"Total repeat duration (e.g. `5s` or `indefinite`)."#),
    fill_attr(r#"Final state of the attribute: `remove` (default) or `freeze`."#),
    accumulate_attr(r#"Accumulation mode: `none` (default) or `sum`."#),
    additive_attr(r#"Additive mode: `replace` (default) or `sum`."#),
    auto_reverse(r#"Whether the animation plays in reverse on alternate iterations."#),
    begin_offset(r#"Component offset within the parent timing model."#));

define_svg_element!(SvgSet, "set",
    attribute_name(r#"Name of the target attribute to set."#),
    attribute_type(r#"Type of the target attribute."#),
    to_attr(r#"Value to set on the target attribute (required)."#),
    begin(r#"Begin time."#),
    end(r#"End time."#),
    dur(r#"Duration (defaults to `indefinite`)."#),
    fill_attr(r#"Final state: `remove` or `freeze`."#));

define_svg_element!(SvgAnimateTransform, "animateTransform",
    attribute_name(r#"Name of the transform attribute to animate (one of `transform` or a transform list)."#),
    attribute_type(r#"Type of the target attribute."#),
    r#type(r#"Kind of transform to animate: `translate`, `scale`, `rotate`, `skewX`, `skewY`."#),
    from_attr(r#"Initial transform value."#),
    to_attr(r#"Final transform value."#),
    by_attr(r#"Relative transform offset."#),
    values(r#"Semicolon-separated list of transform values."#),
    key_times(r#"Semicolon-separated time fractions matching `values`."#),
    key_splines(r#"Bezier control points matching `key_times`."#),
    calc_mode(r#"Interpolation mode: `discrete`, `linear`, `paced`, `spline`."#),
    dur(r#"Duration of one cycle."#),
    begin(r#"Begin time."#),
    end(r#"End time."#),
    repeat_count(r#"Repetition count."#),
    repeat_dur(r#"Total repeat duration."#),
    fill_attr(r#"Final state of the transform."#),
    additive_attr(r#"Additive behavior with respect to the underlying transform."#),
    accumulate_attr(r#"Accumulation of repeated iterations."#));

define_svg_element!(SvgAnimateMotion, "animateMotion",
    path(r#"Path data describing the motion trajectory."#),
    rotate_count(r#"Rotation behavior along the path: `auto`, `auto-reverse`, or a fixed angle."#),
    from_attr(r#"Initial offset along the motion path."#),
    to_attr(r#"Final offset along the motion path."#),
    by_attr(r#"Relative offset along the motion path."#),
    dur(r#"Duration of one cycle."#),
    begin(r#"Begin time."#),
    end(r#"End time."#),
    repeat_count(r#"Repetition count."#),
    repeat_dur(r#"Total repeat duration."#),
    fill_attr(r#"Final state: `remove` or `freeze`."#),
    additive_attr(r#"Additive behavior."#),
    accumulate_attr(r#"Accumulation mode."#),
    calc_mode(r#"Interpolation mode."#),
    key_times(r#"Semicolon-separated time fractions."#),
    key_splines(r#"Bezier control points."#),
    key_points(r#"Distance fractions paired with `key_times` when `calc_mode=\"linear\"`."#));

// Create factories.
svg_factory!(animate, SvgAnimate);
svg_factory!(set, SvgSet);
svg_factory!(animate_transform, SvgAnimateTransform);
svg_factory!(animate_motion, SvgAnimateMotion);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::renderable::Renderable;

    #[test]
    fn animate_with_dur() {
        animate().dur("3s").render();
    }

    #[test]
    fn set_with_to() {
        set().to_attr("red").render();
    }

    #[test]
    fn animate_transform_with_type() {
        use crate::attributes::attr;
        animate_transform().attrs(vec![attr("type").value("scale")]).render();
    }
}
