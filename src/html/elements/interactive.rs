//! Interactive elements (`<canvas>`, `<dialog>`, `<script>`, etc.).

use super::macros::{define_html_element, factory};

define_html_element!(HtmlCanvas, "canvas", all,
    width(r#"Rendered width in CSS pixels (a valid non-negative integer; default `300`).

Used to size the bitmap. Distinct from the CSS `width` property: the attribute sets the bitmap size, which is then scaled by CSS."#),
    height(r#"Rendered height in CSS pixels (a valid non-negative integer; default `150`).

Used to size the bitmap. Distinct from the CSS `height` property: the attribute sets the bitmap size, which is then scaled by CSS."#));
define_html_element!(HtmlDetails, "details", all, open_attr(r#"Boolean attribute. When present, the child `<summary>`'s siblings are shown; when absent, they are hidden.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Toggling this attribute (or user interaction with the summary) shows or hides the disclosure body."#));
define_html_element!(HtmlSummary, "summary", all);
define_html_element!(HtmlDialog, "dialog", all, open_attr(r#"Boolean attribute. When present, the dialog is shown and is interactive.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Without this attribute, the dialog must be displayed via JavaScript (`dialog.show()`, `dialog.showModal()`) or the Popover API."#));
define_html_element!(HtmlScript, "script", all,
    src(r#"URL of an external script.

Resolves relative to the document. The fetched resource is parsed and executed as the script's content."#),
    type_attr(r#"Type of the script.

Standard values:
- Omitted (or `text/javascript`): classic JavaScript
- `module`: JavaScript module (ESM); implies `defer` and CORS-enabled fetching
- `importmap`: an import map for module resolution
- `speculationrules`: a Speculation Rules document
- `text/plain` and other MIME types: a data block (not executed)

A value starting with a JavaScript MIME type essence (e.g. `text/javascript`, `application/javascript`) is treated as a JavaScript script. Authors should usually omit the attribute."#),
    async_attr(r#"Boolean attribute. When present, the script is fetched in parallel and executed as soon as it is available, even before parsing is complete.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Not allowed for `type="module"` scripts, which always behave as if deferred."#),
    defer_attr(r#"Boolean attribute. When present, the script is fetched in parallel and executed after the document has been parsed but before firing `DOMContentLoaded`.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Implicit for `type="module"` scripts."#),
    nomodule(r#"Boolean attribute. When present, the script is not executed by browsers that support ES modules (i.e. browsers that understand `type="module"`).

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Used to provide a fallback classic script for legacy browsers."#),
    integrity(r#"Subresource Integrity hash (e.g. `sha384-...`).

The browser refuses to execute the script if its hash does not match the resource."#),
    crossorigin(r#"CORS setting for the script request.

One of:
- `anonymous`
- `use-credentials`

Required for `type="module"` scripts to enable error reporting and to share the script across origins."#),
    nonce(r#"Cryptographic nonce used by Content Security Policy to permit inline script execution that would otherwise be blocked."#),
    referrerpolicy(r#"Referrer policy for the script request.

One of:
- `no-referrer`
- `no-referrer-when-downgrade`
- `same-origin`
- `origin`
- `strict-origin`
- `origin-when-cross-origin`
- `strict-origin-when-cross-origin`
- `unsafe-url`"#),
    blocking(r#"Tokens indicating the script blocks operations until fetched and executed.

Currently one keyword:
- `render` (blocks rendering of any content that appears after the script in the document)

The script is render-blocking only when this attribute contains `render`."#));
define_html_element!(HtmlNoscript, "noscript", all);
define_html_element!(HtmlTemplate, "template", all);
define_html_element!(HtmlSlot, "slot", all, name(r#"Name of the slot.

Slots with a `name` attribute match elements with a `slot` attribute of the same value; slots without a `name` are the default slot. Inherited from the standard slotting algorithm used by the Shadow DOM API."#));

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
