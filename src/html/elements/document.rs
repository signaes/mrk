//! Document metadata elements.

use super::macros::{define_html_element, factory};

define_html_element!(HtmlHtml, "html", no_aria);
define_html_element!(HtmlHead, "head", no_aria);
define_html_element!(HtmlTitle, "title", no_aria);
define_html_element!(HtmlBase, "base", no_aria,
    href(r#"Document base URL for resolving relative URLs.

Must be an absolute URL."#),
    target(r#"Default browsing context for hyperlinks and forms without an explicit `target`.

One of:
- `_self` (default if omitted)
- `_blank`
- `_parent`
- `_top`
- a navigable target name (must be 1 or more ASCII letters, followed by zero or more ASCII letters, digits, or hyphens)"#));
define_html_element!(HtmlLink, "link", no_aria,
    href(r#"URL of the linked resource."#),
    rel(r#"Relationship between the current document and the linked resource.

A space-separated list of link types. Common values:
- `alternate`
- `author`
- `canonical`
- `dns-prefetch`
- `help`
- `icon`
- `license`
- `manifest`
- `modulepreload`
- `next`
- `pingback`
- `preconnect`
- `prefetch`
- `preload`
- `prev`
- `search`
- `stylesheet`

Some tokens (e.g. `preload`) require the `as` attribute to describe the destination."#),
    type_attr(r#"MIME type hint for the linked resource (e.g. `text/css`, `image/png`, `application/json`).

For `rel="stylesheet"`, valid values are `text/css` (only, if present). User agents must not consider this attribute a definitive statement of the resource's type."#),
    media(r#"Media query list for which the resource applies (e.g. `screen`, `print`, `(min-width: 800px)`).

Accepts any valid media query list; default is `all`."#),
    sizes(r#"Icon sizes for `rel="icon"`.

Comma-separated list of sizes, each either `any` or `<width>x<height>` in CSS pixels:
- `any`
- `16x16`
- `32x32 64x64`"#),
    crossorigin(r#"CORS setting for the request.

One of:
- `anonymous`
- `use-credentials`"#),
    integrity(r#"Subresource Integrity hash (e.g. `sha384-...`).

A base64-encoded cryptographic hash of the resource, prefixed with the algorithm name. The browser refuses to apply the resource if the hash does not match."#),
    hreflang(r#"Language of the linked resource as a BCP 47 language tag (e.g. `en`, `en-US`)."#),
    referrerpolicy(r#"Referrer policy for the request.

One of:
- `no-referrer`
- `no-referrer-when-downgrade`
- `same-origin`
- `origin`
- `strict-origin`
- `origin-when-cross-origin`
- `strict-origin-when-cross-origin`
- `unsafe-url`"#),
    as_attr(r#"Fetch destination for `rel="preload"` or `rel="modulepreload"`.

One of:
- `audio`
- `audioworklet`
- `document`
- `embed`
- `fetch`
- `font`
- `frame`
- `iframe`
- `image`
- `json`
- `manifest`
- `object`
- `paintworklet`
- `report`
- `script`
- `serviceworker`
- `sharedworker`
- `style`
- `track`
- `video`
- `worker`
- `xslt`"#),
    color(r#"Color hint for `rel="icon"` (e.g. `#fff`, `tomato`).

Used by some user agents to render the UI accent alongside the icon (notably the address-bar icon on macOS Safari)."#),
    disabled(r#"Boolean attribute. When present, the link is disabled and not fetched.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. User agents ignore disabled links when discovering resources."#),
    fetchpriority(r#"Hint for the relative fetch priority of the link.

One of:
- `high`
- `low`
- `auto` (default)"#),
    imagesizes(r#"Sizes for an image source set used in conjunction with `imagesrcset`.

Same syntax as the `sizes` attribute on `<img>` (e.g. `(max-width: 600px) 100vw, 50vw`)."#),
    imagesrcset(r#"Source set for an image used in conjunction with `imagesizes`.

Same syntax as the `srcset` attribute on `<img>` (e.g. `small.webp 1x, large.webp 2x`)."#));
define_html_element!(HtmlMeta, "meta", no_aria,
    name(r#"Metadata name (e.g. `application-name`, `author`, `description`, `generator`, `keywords`, `referrer`, `theme-color`, `color-scheme`, `viewport`).

Pairs with `content` to form a name/value pair. Some standard names are also recognized as `<meta>` pragmas when used with `http-equiv`."#),
    content(r#"Metadata value associated with the `name` or `http-equiv` attribute."#),
    charset(r#"Character encoding declaration for the document.

The standard value is `UTF-8`. Equivalent to `<meta charset="UTF-8">` and must appear within the first 1024 bytes of the document."#),
    http_equiv(r#"Pragma directive, simulating an HTTP response header.

Common values:
- `content-type`
- `default-style`
- `refresh`
- `x-ua-compatible`
- `content-security-policy`

The meaning of `content` depends on the directive; e.g. `refresh` uses `content="5; url=/next"` to redirect after 5 seconds."#),
    media(r#"Media query for which the metadata applies.

Accepts any valid media query list. Currently only meaningful for `name="theme-color"` (controls per-scheme browser chrome color)."#));
define_html_element!(HtmlStyle, "style", no_aria,
    type_attr(r#"MIME type of the style sheet.

The standard value is `text/css`. In practice this attribute should be omitted; the value, if present, must be `text/css`."#),
    media(r#"Media query list for which the styles apply (e.g. `screen`, `print`, `(min-width: 800px)`).

Accepts any valid media query list; default is `all`."#),
    nonce(r#"Cryptographic nonce used by Content Security Policy to permit inline styles that would otherwise be blocked."#),
    title(r#"Title of the stylesheet; user agents may expose this in alternate-stylesheet pickers."#),
    blocking(r#"Tokens indicating the stylesheet blocks rendering until fetched and applied.

Currently one keyword:
- `render`

The element is render-blocking only when this attribute contains `render`."#));

factory!(
    /// Create a new [`HtmlHtml`] element (`<html>`).
    html, HtmlHtml
);
factory!(
    /// Create a new [`HtmlHead`] element (`<head>`).
    head, HtmlHead
);
factory!(
    /// Create a new [`HtmlTitle`] element (`<title>`).
    title, HtmlTitle
);
factory!(
    /// Create a new [`HtmlBase`] element (`<base>`).
    base, HtmlBase
);
factory!(
    /// Create a new [`HtmlLink`] element (`<link>`).
    link, HtmlLink
);
factory!(
    /// Create a new [`HtmlMeta`] element (`<meta>`).
    meta, HtmlMeta
);
factory!(
    /// Create a new [`HtmlStyle`] element (`<style>`).
    style, HtmlStyle
);

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
