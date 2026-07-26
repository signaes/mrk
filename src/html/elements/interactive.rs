//! Interactive elements (`<canvas>`, `<dialog>`, `<script>`, etc.).

use super::macros::{define_html_element, factory};

define_html_element!(HtmlCanvas, "canvas",
    width("Width in pixels."),
    height("Height in pixels."));
define_html_element!(HtmlDetails, "details", open_attr("Whether the details are open."));
define_html_element!(HtmlSummary, "summary");
define_html_element!(HtmlDialog, "dialog", open_attr("Whether the dialog is open."));
define_html_element!(HtmlScript, "script",
    src("URL of the external script."),
    type_attr("Script type (module, text/javascript)."),
    async_attr("Whether to execute asynchronously."),
    defer_attr("Whether to defer execution until after parsing."),
    nomodule("Whether to skip in module-supporting browsers."),
    integrity("Subresource integrity hash."),
    crossorigin("CORS setting (anonymous or use-credentials)."),
    nonce("Cryptographic nonce for CSP."),
    referrerpolicy("Referrer policy for the request."),
    blocking("Blocking token for render-blocking scripts."));
define_html_element!(HtmlNoscript, "noscript");
define_html_element!(HtmlTemplate, "template");
define_html_element!(HtmlSlot, "slot", name("Slot name for shadow DOM."));

// Create a new [`HtmlCanvas`] element (`<canvas>`).
factory!(canvas, HtmlCanvas);
// Create a new [`HtmlDetails`] element (`<details>`).
factory!(details, HtmlDetails);
// Create a new [`HtmlSummary`] element (`<summary>`).
factory!(summary, HtmlSummary);
// Create a new [`HtmlDialog`] element (`<dialog>`).
factory!(dialog, HtmlDialog);
// Create a new [`HtmlScript`] element (`<script>`).
factory!(script, HtmlScript);
// Create a new [`HtmlNoscript`] element (`<noscript>`).
factory!(noscript, HtmlNoscript);
// Create a new [`HtmlTemplate`] element (`<template>`).
factory!(template, HtmlTemplate);
// Create a new [`HtmlSlot`] element (`<slot>`).
factory!(slot, HtmlSlot);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_attrs() {
        assert_eq!(canvas().width("300").render(), r#"<canvas width="300"></canvas>"#);
        assert_eq!(canvas().height("200").render(), r#"<canvas height="200"></canvas>"#);
    }

    #[test]
    fn details_attrs() {
        assert_eq!(details().open_attr("true").render(), r#"<details open="true"></details>"#);
    }

    #[test]
    fn summary_element() {
        assert_eq!(summary().render(), "<summary></summary>");
    }

    #[test]
    fn dialog_attrs() {
        assert_eq!(dialog().open_attr("true").render(), r#"<dialog open="true"></dialog>"#);
    }

    #[test]
    fn script_attrs() {
        assert_eq!(script().src("app.js").render(), r#"<script src="app.js"></script>"#);
        assert_eq!(script().type_attr("module").render(), r#"<script type="module"></script>"#);
        assert_eq!(script().async_attr("true").render(), r#"<script async="true"></script>"#);
        assert_eq!(script().defer_attr("true").render(), r#"<script defer="true"></script>"#);
        assert_eq!(script().nomodule("true").render(), r#"<script nomodule="true"></script>"#);
        assert_eq!(script().integrity("sha384-abc").render(), r#"<script integrity="sha384-abc"></script>"#);
        assert_eq!(script().crossorigin("anonymous").render(), r#"<script crossorigin="anonymous"></script>"#);
        assert_eq!(script().nonce("abc").render(), r#"<script nonce="abc"></script>"#);
        assert_eq!(script().referrerpolicy("no-referrer").render(), r#"<script referrerpolicy="no-referrer"></script>"#);
        assert_eq!(script().blocking("render").render(), r#"<script blocking="render"></script>"#);
    }

    #[test]
    fn noscript_element() {
        assert_eq!(noscript().render(), "<noscript></noscript>");
    }

    #[test]
    fn template_element() {
        assert_eq!(template().render(), "<template></template>");
    }

    #[test]
    fn slot_attrs() {
        assert_eq!(slot().name("header").render(), r#"<slot name="header"></slot>"#);
    }
}
