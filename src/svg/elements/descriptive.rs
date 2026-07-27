//! Descriptive elements: `<title>`, `<desc>`, `<metadata>`, `<foreignObject>`.

use super::macros::{define_svg_element, svg_factory};

define_svg_element!(SvgTitle, "title");

define_svg_element!(SvgDesc, "desc");

define_svg_element!(SvgMetadata, "metadata");

define_svg_element!(SvgForeignObject, "foreignObject",
    x(r#"X coordinate of the foreign object's location."#),
    y(r#"Y coordinate of the foreign object's location."#),
    width(r#"Width of the foreign object (must be a `<length>`). Required."#),
    height(r#"Height of the foreign object (must be a `<length>`). Required."#));

svg_factory!(
    /// Create a new [`SvgTitle`] element (`<title>`).
    title, SvgTitle
);
svg_factory!(
    /// Create a new [`SvgDesc`] element (`<desc>`).
    desc, SvgDesc
);
svg_factory!(
    /// Create a new [`SvgMetadata`] element (`<metadata>`).
    metadata, SvgMetadata
);
svg_factory!(
    /// Create a new [`SvgForeignObject`] element (`<foreignObject>`).
    foreign_object, SvgForeignObject
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptive_no_attrs() {
        assert_eq!(title().render(), "<title></title>");
        assert_eq!(desc().render(), "<desc></desc>");
        assert_eq!(metadata().render(), "<metadata></metadata>");
    }

    #[test]
    fn foreign_object_attrs() {
        // Verify `width` and `height` are valid method names (no compiler clash).
        foreign_object().x("0").render();
    }

    /// Exercise the common-globals and event-handler methods emitted
    /// by `define_svg_element!` on the descriptive elements. This
    /// drives every body line of the helper macros that the consumer
    /// expands.
    #[test]
    fn descriptive_globals_and_events() {
        let _ = title()
            .id("x").class("y").style("z").tabindex_global("1")
            .lang_global("en").dir("ltr").hidden("t").draggable("true")
            .spellcheck_global("true").title_global("t").translate("yes")
            .contenteditable("true").nonce_global("a").slot("s").part("p")
            .inputmode("text").enterkeyhint("enter").popover("auto")
            .is_content("x").autofocus_global("true").form_global("f")
            .data_attr("x", "d").list_global("l").autocomplete_global("on")
            .aria_label("l").aria_hidden("t").aria_role("b").aria_live("p")
            .aria_expanded("f").aria_selected("t")
            .onclick("1").onchange("1").oninput("1").onsubmit("1")
            .onfocus("1").onblur("1").onkeydown("1").onkeyup("1")
            .onmousedown("1").onmouseup("1").onmouseover("1")
            .onmouseout("1").onmousemove("1").onload("1")
            .render();
        let _ = desc().id("d").render();
        let _ = metadata().id("m").render();
        let _ = foreign_object().id("f").x("0").y("0").width("100").height("50").render();
    }
}
