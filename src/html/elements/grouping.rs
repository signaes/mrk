//! Grouping elements (lists, paragraphs, blockquotes, etc.).

use super::macros::{add_bool_methods, define_html_element, factory};

define_html_element!(HtmlP, "p", all);
define_html_element!(HtmlHr, "hr", all);
define_html_element!(HtmlPre, "pre", all,
    tabindex(r#"Tab navigation order for the element.

A valid integer. Negative values remove the element from the tab order. This is a global attribute; included here for convenience on `<pre>`."#),
    wrap(r#"Legacy hint for how text in the element is wrapped.

The WHATWG HTML Living Standard does not list this attribute on `<pre>`. It is preserved here for compatibility with older documents. Use CSS `white-space` for new code."#),
    name(r#"Legacy attribute on `<pre>`. Not part of current HTML.

Previously used to associate a name with the element. Use `id` instead."#),
    cols(r#"Legacy attribute on `<pre>`. Not part of current HTML.

Previously specified the preferred column count. Use CSS `width`/`max-width` instead."#));
add_bool_methods!(HtmlPre,
    autofocus(r#"Boolean attribute. When present, the element receives focus when the document or dialog is loaded.

This is a global boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`. Only one element per document may autofocus."#));
define_html_element!(HtmlBlockquote, "blockquote", all, cite(r#"URL of the source being quoted or referenced.

May be a citation for a journalistic-style block quote, a link to the source article, or a reference to a person whose words are quoted."#));
define_html_element!(HtmlOl, "ol", all,
    type_attr(r#"Kind of marker used to label each list item.

One of:
- `1` (decimal numbers; default)
- `a` (lowercase ASCII letters: `a`, `b`, `c`, ...)
- `A` (uppercase ASCII letters: `A`, `B`, `C`, ...)
- `i` (lowercase Roman numerals: `i`, `ii`, `iii`, ...)
- `I` (uppercase Roman numerals: `I`, `II`, `III`, ...)

For new content, prefer the CSS `list-style-type` property. This attribute is supported for legacy reasons."#),
    start(r#"Starting ordinal value for the list (a valid integer; default `1`).

Together with `type` and `reversed`, controls the rendered sequence."#));
add_bool_methods!(HtmlOl,
    reversed(r#"Boolean attribute. When present, the list is rendered in descending order (the `start` value is treated as the highest number).

This is a boolean attribute. In HTML, presence is sufficient; the value is conventionally an empty string or `"true"`."#));
define_html_element!(HtmlUl, "ul", all);
define_html_element!(HtmlMenu, "menu", all);
define_html_element!(HtmlLi, "li", all,
    value(r#"Override the ordinal value of this list item (a valid integer).

Applies only when the parent is an `<ol>`. The next items continue counting from this value."#));
define_html_element!(HtmlDl, "dl", all);
define_html_element!(HtmlDt, "dt", all);
define_html_element!(HtmlDd, "dd", all);
define_html_element!(HtmlFigure, "figure", all);
define_html_element!(HtmlFigcaption, "figcaption", all);
define_html_element!(HtmlDiv, "div", all);

factory!(
    /// Create a new [`HtmlP`] element (`<p>`).
    p, HtmlP
);
factory!(
    /// Create a new [`HtmlHr`] element (`<hr>`).
    hr, HtmlHr
);
factory!(
    /// Create a new [`HtmlPre`] element (`<pre>`).
    pre, HtmlPre
);
factory!(
    /// Create a new [`HtmlBlockquote`] element (`<blockquote>`).
    blockquote, HtmlBlockquote
);
factory!(
    /// Create a new [`HtmlOl`] element (`<ol>`).
    ol, HtmlOl
);
factory!(
    /// Create a new [`HtmlUl`] element (`<ul>`).
    ul, HtmlUl
);
factory!(
    /// Create a new [`HtmlMenu`] element (`<menu>`).
    menu, HtmlMenu
);
factory!(
    /// Create a new [`HtmlLi`] element (`<li>`).
    li, HtmlLi
);
factory!(
    /// Create a new [`HtmlDl`] element (`<dl>`).
    dl, HtmlDl
);
factory!(
    /// Create a new [`HtmlDt`] element (`<dt>`).
    dt, HtmlDt
);
factory!(
    /// Create a new [`HtmlDd`] element (`<dd>`).
    dd, HtmlDd
);
factory!(
    /// Create a new [`HtmlFigure`] element (`<figure>`).
    figure, HtmlFigure
);
factory!(
    /// Create a new [`HtmlFigcaption`] element (`<figcaption>`).
    figcaption, HtmlFigcaption
);
factory!(
    /// Create a new [`HtmlDiv`] element (`<div>`).
    div, HtmlDiv
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p_element() {
        assert_eq!(p().render(), "<p></p>");
    }

    #[test]
    fn hr_element() {
        assert_eq!(hr().render(), "<hr>");
    }

    #[test]
    fn pre_attrs() {
        assert_eq!(pre().tabindex("0").render(), r#"<pre tabindex="0"></pre>"#);
        assert_eq!(pre().wrap("soft").render(), r#"<pre wrap="soft"></pre>"#);
        assert_eq!(pre().name("code").render(), r#"<pre name="code"></pre>"#);
        assert_eq!(pre().cols("80").render(), r#"<pre cols="80"></pre>"#);
    }

    #[test]
    fn pre_boolean_attrs_table() {
        let cases = [("autofocus", pre().autofocus().render(), r#"<pre autofocus></pre>"#)];
        for (name, actual, expected) in cases {
            assert_eq!(actual, expected, "case: {name}");
        }
    }

    #[test]
    fn blockquote_attrs() {
        assert_eq!(blockquote().cite("http://example.com").render(), r#"<blockquote cite="http://example.com"></blockquote>"#);
    }

    #[test]
    fn ol_attrs() {
        assert_eq!(ol().type_attr("1").render(), r#"<ol type="1"></ol>"#);
        assert_eq!(ol().start("5").render(), r#"<ol start="5"></ol>"#);
    }

    #[test]
    fn ol_boolean_attrs_table() {
        let cases = [("reversed", ol().reversed().render(), r#"<ol reversed></ol>"#)];
        for (name, actual, expected) in cases {
            assert_eq!(actual, expected, "case: {name}");
        }
    }

    #[test]
    fn ul_element() {
        assert_eq!(ul().render(), "<ul></ul>");
    }

    #[test]
    fn menu_element() {
        assert_eq!(menu().render(), "<menu></menu>");
    }

    #[test]
    fn li_attrs() {
        assert_eq!(li().value("3").render(), r#"<li value="3"></li>"#);
    }

    #[test]
    fn dl_element() {
        assert_eq!(dl().render(), "<dl></dl>");
    }

    #[test]
    fn dt_element() {
        assert_eq!(dt().render(), "<dt></dt>");
    }

    #[test]
    fn dd_element() {
        assert_eq!(dd().render(), "<dd></dd>");
    }

    #[test]
    fn figure_element() {
        assert_eq!(figure().render(), "<figure></figure>");
    }

    #[test]
    fn figcaption_element() {
        assert_eq!(figcaption().render(), "<figcaption></figcaption>");
    }

    #[test]
    fn div_element() {
        assert_eq!(div().render(), "<div></div>");
    }
}
