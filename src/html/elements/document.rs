//! Document metadata elements.

use super::macros::{define_html_element, factory};

define_html_element!(HtmlHtml, "html");
define_html_element!(HtmlHead, "head");
define_html_element!(HtmlTitle, "title");
define_html_element!(HtmlBase, "base", href("URL for relative links."), target("Default frame target."));
define_html_element!(HtmlLink, "link",
    href("URL of the linked resource."),
    rel("Relationship to the linked resource."),
    type_attr("MIME type of the resource."),
    media("Target media query."),
    sizes("Icon sizes."),
    crossorigin("CORS setting."),
    integrity("Subresource integrity hash."),
    hreflang("Language of the linked resource."),
    referrerpolicy("Referrer policy."),
    as_attr("Fetch destination."),
    color("Theme color."),
    disabled("Whether the link is disabled."),
    fetchpriority("Fetch priority hint."),
    imagesizes("Image sizes for srcset."),
    imagesrcset("Image sources for srcset."));
define_html_element!(HtmlMeta, "meta",
    name("Metadata name."),
    content("Metadata value."),
    charset("Character encoding."),
    http_equiv("Pragma directive."),
    media("Target media query."));
define_html_element!(HtmlStyle, "style",
    type_attr("MIME type of the style sheet."),
    media("Target media query."),
    nonce("Cryptographic nonce."),
    title("Style sheet title."),
    blocking("Blocking token."));

// Create a new [`HtmlHtml`] element (`<html>`).
factory!(html, HtmlHtml);
// Create a new [`HtmlHead`] element (`<head>`).
factory!(head, HtmlHead);
// Create a new [`HtmlTitle`] element (`<title>`).
factory!(title, HtmlTitle);
// Create a new [`HtmlBase`] element (`<base>`).
factory!(base, HtmlBase);
// Create a new [`HtmlLink`] element (`<link>`).
factory!(link, HtmlLink);
// Create a new [`HtmlMeta`] element (`<meta>`).
factory!(meta, HtmlMeta);
// Create a new [`HtmlStyle`] element (`<style>`).
factory!(style, HtmlStyle);

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn html_element() {
        assert_eq!(html().render(), "<html></html>");
    }

    #[test]
    fn head_element() {
        assert_eq!(head().render(), "<head></head>");
    }

    #[test]
    fn title_element() {
        assert_eq!(title().render(), "<title></title>");
    }

    #[test]
    fn base_attrs() {
        assert_eq!(base().href("/").render(), r#"<base href="/">"#);
        assert_eq!(base().target("_blank").render(), r#"<base target="_blank">"#);
    }

    #[test]
    fn link_attrs() {
        assert_eq!(link().href("style.css").render(), r#"<link href="style.css">"#);
        assert_eq!(link().rel("stylesheet").render(), r#"<link rel="stylesheet">"#);
        assert_eq!(link().type_attr("text/css").render(), r#"<link type="text/css">"#);
        assert_eq!(link().media("screen").render(), r#"<link media="screen">"#);
        assert_eq!(link().sizes("32x32").render(), r#"<link sizes="32x32">"#);
        assert_eq!(link().crossorigin("anonymous").render(), r#"<link crossorigin="anonymous">"#);
        assert_eq!(link().integrity("sha384-abc").render(), r#"<link integrity="sha384-abc">"#);
        assert_eq!(link().hreflang("en").render(), r#"<link hreflang="en">"#);
        assert_eq!(link().referrerpolicy("no-referrer").render(), r#"<link referrerpolicy="no-referrer">"#);
        assert_eq!(link().as_attr("image").render(), r#"<link as="image">"#);
        assert_eq!(link().color("#fff").render(), r##"<link color="#fff">"##);
        assert_eq!(link().disabled("true").render(), r#"<link disabled="true">"#);
        assert_eq!(link().fetchpriority("high").render(), r#"<link fetchpriority="high">"#);
        assert_eq!(link().imagesizes("100vw").render(), r#"<link imagesizes="100vw">"#);
        assert_eq!(link().imagesrcset("img.webp").render(), r#"<link imagesrcset="img.webp">"#);
    }

    #[test]
    fn meta_attrs() {
        assert_eq!(meta().name("description").render(), r#"<meta name="description">"#);
        assert_eq!(meta().content("hello").render(), r#"<meta content="hello">"#);
        assert_eq!(meta().charset("UTF-8").render(), r#"<meta charset="UTF-8">"#);
        assert_eq!(meta().http_equiv("refresh").render(), r#"<meta http-equiv="refresh">"#);
        assert_eq!(meta().media("screen").render(), r#"<meta media="screen">"#);
    }

    #[test]
    fn style_attrs() {
        assert_eq!(style().type_attr("text/css").render(), r#"<style type="text/css"></style>"#);
        assert_eq!(style().media("print").render(), r#"<style media="print"></style>"#);
        assert_eq!(style().nonce("abc").render(), r#"<style nonce="abc"></style>"#);
        assert_eq!(style().title("main").render(), r#"<style title="main"></style>"#);
        assert_eq!(style().blocking("render").render(), r#"<style blocking="render"></style>"#);
    }
}
