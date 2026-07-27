//! Text-level semantic elements (`<a>`, `<em>`, `<strong>`, `<span>`, etc.).

use super::macros::{define_html_element, factory};

define_html_element!(HtmlA, "a", all,
    href(r#"URL the link points to.

If absent, the `<a>` element is a placeholder hyperlink; it does not navigate. May be a relative URL, an absolute URL, a fragment identifier (`#id`), or a `mailto:`/`tel:`/etc. URL."#),
    target(r#"Browsing context for the linked resource.

One of:
- `_self` (default)
- `_blank`
- `_parent`
- `_top`
- a navigable target name

`target="_blank"` on user-supplied links is a tabnabbing risk; pair it with `rel="noopener noreferrer"`."#),
    rel(r#"Relationship between the current document and the linked resource.

A space-separated list of link types. Common values:
- `alternate`
- `author`
- `bookmark`
- `external`
- `help`
- `license`
- `next`
- `nofollow`
- `noopener`
- `noreferrer`
- `prev`
- `search`
- `tag`

`noopener` and `noreferrer` are recommended for `target="_blank"`."#),
    hreflang(r#"Language of the linked resource as a BCP 47 language tag (e.g. `en`, `en-US`).

For use with the `hreflang` hint; user agents may use it for accessibility or rendering decisions."#),
    type_attr(r#"Hint for the MIME type of the linked resource (e.g. `text/html`, `application/pdf`).

User agents may use this to skip fetching a resource that they know they cannot handle. Not a definitive statement of the resource's type."#),
    download(r#"If present, the linked resource is downloaded instead of navigated to. The value, if provided, suggests the filename.

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Same-origin URLs are required for non-empty filenames; cross-origin downloads are typically ignored by browsers."#),
    ping(r#"Space-separated list of URLs to ping with a `POST` request when the link is followed.

Used for click-through tracking. Pings are sent in the background, do not block navigation, and are subject to referrer policy."#),
    referrerpolicy(r#"Referrer policy for the request.

One of:
- `no-referrer`
- `no-referrer-when-downgrade`
- `same-origin`
- `origin`
- `strict-origin`
- `origin-when-cross-origin`
- `strict-origin-when-cross-origin`
- `unsafe-url`"#));
define_html_element!(HtmlEm, "em", all);
define_html_element!(HtmlStrong, "strong", all);
define_html_element!(HtmlSmall, "small", all);
define_html_element!(HtmlS, "s", all);
define_html_element!(HtmlCite, "cite", all);
define_html_element!(HtmlQ, "q", all, cite(r#"URL of the source being quoted or referenced.

A citation for the quotation, exposed to assistive technologies and used by browsers to offer a "go to citation" affordance."#));
define_html_element!(HtmlDfn, "dfn", all);
define_html_element!(HtmlAbbr, "abbr", all, title_attr(r#"Full expansion of the abbreviation.

A human-readable expansion of the term. Optional; if absent, the abbreviation is expanded only when explicitly defined (e.g. by a surrounding `<dfn>`)."#));
define_html_element!(HtmlRuby, "ruby", all);
define_html_element!(HtmlRt, "rt", all);
define_html_element!(HtmlRp, "rp", all);
define_html_element!(HtmlData, "data", all, value(r#"Machine-readable equivalent of the element's contents.

A string that the script or application can read from the `value` IDL property. The visible text is the element's child content."#));
define_html_element!(HtmlTime, "time", all, datetime(r#"Machine-readable equivalent of the element's contents, as a global date or global date-time string.

Valid forms include:
- A valid date string (e.g. `2025-01-15`)
- A valid time string (e.g. `13:45:00`)
- A valid local date-time string (e.g. `2025-01-15T13:45`)
- A valid global date-time string (e.g. `2025-01-15T13:45:00Z` or `2025-01-15T13:45:00+02:00`)
- A valid duration string (e.g. `P3DT4H`)
- A week string (e.g. `2025-W03`)

If absent, the element's text content is parsed instead."#));
define_html_element!(HtmlCode, "code", all);
define_html_element!(HtmlVar, "var", all);
define_html_element!(HtmlSamp, "samp", all);
define_html_element!(HtmlKbd, "kbd", all);
define_html_element!(HtmlSub, "sub", all);
define_html_element!(HtmlSup, "sup", all);
define_html_element!(HtmlI, "i", all);
define_html_element!(HtmlB, "b", all);
define_html_element!(HtmlU, "u", all);
define_html_element!(HtmlMark, "mark", all);
define_html_element!(HtmlBdi, "bdi", all);
define_html_element!(HtmlBdo, "bdo", all);
define_html_element!(HtmlSpan, "span", all);
define_html_element!(HtmlBr, "br", aria_hidden_only);
define_html_element!(HtmlWbr, "wbr", aria_hidden_only);

factory!(
    /// Create a new [`HtmlA`] element (`<a>`).
    a, HtmlA
);
factory!(
    /// Create a new [`HtmlEm`] element (`<em>`).
    em, HtmlEm
);
factory!(
    /// Create a new [`HtmlStrong`] element (`<strong>`).
    strong, HtmlStrong
);
factory!(
    /// Create a new [`HtmlSmall`] element (`<small>`).
    small, HtmlSmall
);
factory!(
    /// Create a new [`HtmlS`] element (`<s>`).
    s, HtmlS
);
factory!(
    /// Create a new [`HtmlCite`] element (`<cite>`).
    cite, HtmlCite
);
factory!(
    /// Create a new [`HtmlQ`] element (`<q>`).
    q, HtmlQ
);
factory!(
    /// Create a new [`HtmlDfn`] element (`<dfn>`).
    dfn, HtmlDfn
);
factory!(
    /// Create a new [`HtmlAbbr`] element (`<abbr>`).
    abbr, HtmlAbbr
);
factory!(
    /// Create a new [`HtmlRuby`] element (`<ruby>`).
    ruby, HtmlRuby
);
factory!(
    /// Create a new [`HtmlRt`] element (`<rt>`).
    rt, HtmlRt
);
factory!(
    /// Create a new [`HtmlRp`] element (`<rp>`).
    rp, HtmlRp
);
factory!(
    /// Create a new [`HtmlData`] element (`<data>`).
    data, HtmlData
);
factory!(
    /// Create a new [`HtmlTime`] element (`<time>`).
    time, HtmlTime
);
factory!(
    /// Create a new [`HtmlCode`] element (`<code>`).
    code, HtmlCode
);
factory!(
    /// Create a new [`HtmlVar`] element (`<var>`).
    var, HtmlVar
);
factory!(
    /// Create a new [`HtmlSamp`] element (`<samp>`).
    samp, HtmlSamp
);
factory!(
    /// Create a new [`HtmlKbd`] element (`<kbd>`).
    kbd, HtmlKbd
);
factory!(
    /// Create a new [`HtmlSub`] element (`<sub>`).
    sub, HtmlSub
);
factory!(
    /// Create a new [`HtmlSup`] element (`<sup>`).
    sup, HtmlSup
);
factory!(
    /// Create a new [`HtmlI`] element (`<i>`).
    i, HtmlI
);
factory!(
    /// Create a new [`HtmlB`] element (`<b>`).
    b, HtmlB
);
factory!(
    /// Create a new [`HtmlU`] element (`<u>`).
    u, HtmlU
);
factory!(
    /// Create a new [`HtmlMark`] element (`<mark>`).
    mark, HtmlMark
);
factory!(
    /// Create a new [`HtmlBdi`] element (`<bdi>`).
    bdi, HtmlBdi
);
factory!(
    /// Create a new [`HtmlBdo`] element (`<bdo>`).
    bdo, HtmlBdo
);
factory!(
    /// Create a new [`HtmlSpan`] element (`<span>`).
    span, HtmlSpan
);
factory!(
    /// Create a new [`HtmlBr`] element (`<br>`).
    br, HtmlBr
);
factory!(
    /// Create a new [`HtmlWbr`] element (`<wbr>`).
    wbr, HtmlWbr
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_attrs() {
        assert_eq!(a().href("/page").render(), r#"<a href="/page"></a>"#);
        assert_eq!(a().target("_blank").render(), r#"<a target="_blank"></a>"#);
        assert_eq!(a().rel("noopener").render(), r#"<a rel="noopener"></a>"#);
        assert_eq!(a().hreflang("en").render(), r#"<a hreflang="en"></a>"#);
        assert_eq!(a().type_attr("text/html").render(), r#"<a type="text/html"></a>"#);
        assert_eq!(a().download("file.txt").render(), r#"<a download="file.txt"></a>"#);
        assert_eq!(a().ping("/track").render(), r#"<a ping="/track"></a>"#);
        assert_eq!(a().referrerpolicy("no-referrer").render(), r#"<a referrerpolicy="no-referrer"></a>"#);
    }

    #[test]
    fn em_element() {
        assert_eq!(em().render(), "<em></em>");
    }

    #[test]
    fn strong_element() {
        assert_eq!(strong().render(), "<strong></strong>");
    }

    #[test]
    fn small_element() {
        assert_eq!(small().render(), "<small></small>");
    }

    #[test]
    fn s_element() {
        assert_eq!(s().render(), "<s></s>");
    }

    #[test]
    fn cite_element() {
        assert_eq!(cite().render(), "<cite></cite>");
    }

    #[test]
    fn q_attrs() {
        assert_eq!(q().cite("http://example.com").render(), r#"<q cite="http://example.com"></q>"#);
    }

    #[test]
    fn dfn_element() {
        assert_eq!(dfn().render(), "<dfn></dfn>");
    }

    #[test]
    fn abbr_attrs() {
        assert_eq!(abbr().title_attr("HyperText Markup Language").render(), r#"<abbr title="HyperText Markup Language"></abbr>"#);
    }

    #[test]
    fn ruby_element() {
        assert_eq!(ruby().render(), "<ruby></ruby>");
    }

    #[test]
    fn rt_element() {
        assert_eq!(rt().render(), "<rt></rt>");
    }

    #[test]
    fn rp_element() {
        assert_eq!(rp().render(), "<rp></rp>");
    }

    #[test]
    fn data_attrs() {
        assert_eq!(data().value("123").render(), r#"<data value="123"></data>"#);
    }

    #[test]
    fn time_attrs() {
        assert_eq!(time().datetime("2024-01-01").render(), r#"<time datetime="2024-01-01"></time>"#);
    }

    #[test]
    fn code_element() {
        assert_eq!(code().render(), "<code></code>");
    }

    #[test]
    fn var_element() {
        assert_eq!(var().render(), "<var></var>");
    }

    #[test]
    fn samp_element() {
        assert_eq!(samp().render(), "<samp></samp>");
    }

    #[test]
    fn kbd_element() {
        assert_eq!(kbd().render(), "<kbd></kbd>");
    }

    #[test]
    fn sub_element() {
        assert_eq!(sub().render(), "<sub></sub>");
    }

    #[test]
    fn sup_element() {
        assert_eq!(sup().render(), "<sup></sup>");
    }

    #[test]
    fn i_element() {
        assert_eq!(i().render(), "<i></i>");
    }

    #[test]
    fn b_element() {
        assert_eq!(b().render(), "<b></b>");
    }

    #[test]
    fn u_element() {
        assert_eq!(u().render(), "<u></u>");
    }

    #[test]
    fn mark_element() {
        assert_eq!(mark().render(), "<mark></mark>");
    }

    #[test]
    fn bdi_element() {
        assert_eq!(bdi().render(), "<bdi></bdi>");
    }

    #[test]
    fn bdo_element() {
        assert_eq!(bdo().render(), "<bdo></bdo>");
    }

    #[test]
    fn span_element() {
        assert_eq!(span().render(), "<span></span>");
    }

    #[test]
    fn br_element() {
        assert_eq!(br().render(), "<br>");
    }

    #[test]
    fn wbr_element() {
        assert_eq!(wbr().render(), "<wbr>");
    }
}
