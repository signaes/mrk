//! Text-level semantic elements (`<a>`, `<em>`, `<strong>`, `<span>`, etc.).

use super::macros::{define_html_element, factory};

define_html_element!(HtmlA, "a",
    href("URL of the hyperlink."),
    target("Frame target for the link."),
    rel("Relationship to the linked resource."),
    hreflang("Language of the linked resource."),
    type_attr("MIME type of the linked resource."),
    download("Filename for downloading the link."),
    ping("URLs to ping when the link is clicked."),
    referrerpolicy("Referrer policy for the request."),
    shape("Area shape for image map links."),
    coords("Area coordinates for image map links."));
define_html_element!(HtmlEm, "em");
define_html_element!(HtmlStrong, "strong");
define_html_element!(HtmlSmall, "small");
define_html_element!(HtmlS, "s");
define_html_element!(HtmlCite, "cite");
define_html_element!(HtmlQ, "q", cite("URL of the source of the quotation."));
define_html_element!(HtmlDfn, "dfn");
define_html_element!(HtmlAbbr, "abbr", title_attr("Full expansion of the abbreviation."));
define_html_element!(HtmlRuby, "ruby");
define_html_element!(HtmlRt, "rt");
define_html_element!(HtmlRp, "rp");
define_html_element!(HtmlRtc, "rtc");
define_html_element!(HtmlRb, "rb");
define_html_element!(HtmlData, "data", value("Machine-readable value."));
define_html_element!(HtmlTime, "time", datetime("Machine-readable date/time value."));
define_html_element!(HtmlCode, "code");
define_html_element!(HtmlVar, "var");
define_html_element!(HtmlSamp, "samp");
define_html_element!(HtmlKbd, "kbd");
define_html_element!(HtmlSub, "sub");
define_html_element!(HtmlSup, "sup");
define_html_element!(HtmlI, "i");
define_html_element!(HtmlB, "b");
define_html_element!(HtmlU, "u");
define_html_element!(HtmlMark, "mark");
define_html_element!(HtmlBdi, "bdi");
define_html_element!(HtmlBdo, "bdo");
define_html_element!(HtmlSpan, "span");
define_html_element!(HtmlBr, "br");
define_html_element!(HtmlWbr, "wbr");

// Create a new [`HtmlA`] element (`<a>`).
factory!(a, HtmlA);
// Create a new [`HtmlEm`] element (`<em>`).
factory!(em, HtmlEm);
// Create a new [`HtmlStrong`] element (`<strong>`).
factory!(strong, HtmlStrong);
// Create a new [`HtmlSmall`] element (`<small>`).
factory!(small, HtmlSmall);
// Create a new [`HtmlS`] element (`<s>`).
factory!(s, HtmlS);
// Create a new [`HtmlCite`] element (`<cite>`).
factory!(cite, HtmlCite);
// Create a new [`HtmlQ`] element (`<q>`).
factory!(q, HtmlQ);
// Create a new [`HtmlDfn`] element (`<dfn>`).
factory!(dfn, HtmlDfn);
// Create a new [`HtmlAbbr`] element (`<abbr>`).
factory!(abbr, HtmlAbbr);
// Create a new [`HtmlRuby`] element (`<ruby>`).
factory!(ruby, HtmlRuby);
// Create a new [`HtmlRt`] element (`<rt>`).
factory!(rt, HtmlRt);
// Create a new [`HtmlRp`] element (`<rp>`).
factory!(rp, HtmlRp);
// Create a new [`HtmlRtc`] element (`<rtc>`).
factory!(rtc, HtmlRtc);
// Create a new [`HtmlRb`] element (`<rb>`).
factory!(rb, HtmlRb);
// Create a new [`HtmlData`] element (`<data>`).
factory!(data, HtmlData);
// Create a new [`HtmlTime`] element (`<time>`).
factory!(time, HtmlTime);
// Create a new [`HtmlCode`] element (`<code>`).
factory!(code, HtmlCode);
// Create a new [`HtmlVar`] element (`<var>`).
factory!(var, HtmlVar);
// Create a new [`HtmlSamp`] element (`<samp>`).
factory!(samp, HtmlSamp);
// Create a new [`HtmlKbd`] element (`<kbd>`).
factory!(kbd, HtmlKbd);
// Create a new [`HtmlSub`] element (`<sub>`).
factory!(sub, HtmlSub);
// Create a new [`HtmlSup`] element (`<sup>`).
factory!(sup, HtmlSup);
// Create a new [`HtmlI`] element (`<i>`).
factory!(i, HtmlI);
// Create a new [`HtmlB`] element (`<b>`).
factory!(b, HtmlB);
// Create a new [`HtmlU`] element (`<u>`).
factory!(u, HtmlU);
// Create a new [`HtmlMark`] element (`<mark>`).
factory!(mark, HtmlMark);
// Create a new [`HtmlBdi`] element (`<bdi>`).
factory!(bdi, HtmlBdi);
// Create a new [`HtmlBdo`] element (`<bdo>`).
factory!(bdo, HtmlBdo);
// Create a new [`HtmlSpan`] element (`<span>`).
factory!(span, HtmlSpan);
// Create a new [`HtmlBr`] element (`<br>`).
factory!(br, HtmlBr);
// Create a new [`HtmlWbr`] element (`<wbr>`).
factory!(wbr, HtmlWbr);

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
        assert_eq!(a().shape("rect").render(), r#"<a shape="rect"></a>"#);
        assert_eq!(a().coords("0,0,100,100").render(), r#"<a coords="0,0,100,100"></a>"#);
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
    fn rtc_element() {
        assert_eq!(rtc().render(), "<rtc></rtc>");
    }

    #[test]
    fn rb_element() {
        assert_eq!(rb().render(), "<rb></rb>");
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
