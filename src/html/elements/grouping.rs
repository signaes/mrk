//! Grouping elements (lists, paragraphs, blockquotes, etc.).

use super::macros::{define_html_element, factory};

define_html_element!(HtmlP, "p");
define_html_element!(HtmlHr, "hr", color("Horizontal rule color."), width("Horizontal rule width."));
define_html_element!(HtmlPre, "pre",
    width("Visual width."),
    cols("Number of columns."),
    tabindex("Tab navigation order."),
    wrap("Text wrapping mode."),
    name("Element name."),
    autofocus("Whether to focus on page load."));
define_html_element!(HtmlBlockquote, "blockquote", cite("URL of the quoted source."));
define_html_element!(HtmlOl, "ol",
    type_attr("List numbering type (1, a, i, etc.)."),
    reversed("Whether the list is reversed."),
    start("Starting number."));
define_html_element!(HtmlUl, "ul", type_attr("List style type (disc, circle, square)."));
define_html_element!(HtmlMenu, "menu",
    type_attr("Menu type (toolbar)."),
    label("Menu label."));
define_html_element!(HtmlLi, "li",
    value("Override the list item number."),
    type_attr("List item style override."));
define_html_element!(HtmlDl, "dl");
define_html_element!(HtmlDt, "dt");
define_html_element!(HtmlDd, "dd");
define_html_element!(HtmlFigure, "figure");
define_html_element!(HtmlFigcaption, "figcaption");
define_html_element!(HtmlDiv, "div");

// Create a new [`HtmlP`] element (`<p>`).
factory!(p, HtmlP);
// Create a new [`HtmlHr`] element (`<hr>`).
factory!(hr, HtmlHr);
// Create a new [`HtmlPre`] element (`<pre>`).
factory!(pre, HtmlPre);
// Create a new [`HtmlBlockquote`] element (`<blockquote>`).
factory!(blockquote, HtmlBlockquote);
// Create a new [`HtmlOl`] element (`<ol>`).
factory!(ol, HtmlOl);
// Create a new [`HtmlUl`] element (`<ul>`).
factory!(ul, HtmlUl);
// Create a new [`HtmlMenu`] element (`<menu>`).
factory!(menu, HtmlMenu);
// Create a new [`HtmlLi`] element (`<li>`).
factory!(li, HtmlLi);
// Create a new [`HtmlDl`] element (`<dl>`).
factory!(dl, HtmlDl);
// Create a new [`HtmlDt`] element (`<dt>`).
factory!(dt, HtmlDt);
// Create a new [`HtmlDd`] element (`<dd>`).
factory!(dd, HtmlDd);
// Create a new [`HtmlFigure`] element (`<figure>`).
factory!(figure, HtmlFigure);
// Create a new [`HtmlFigcaption`] element (`<figcaption>`).
factory!(figcaption, HtmlFigcaption);
// Create a new [`HtmlDiv`] element (`<div>`).
factory!(div, HtmlDiv);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p_element() {
        assert_eq!(p().render(), "<p></p>");
    }

    #[test]
    fn hr_attrs() {
        assert_eq!(hr().color("red").render(), r#"<hr color="red">"#);
        assert_eq!(hr().width("100%").render(), r#"<hr width="100%">"#);
    }

    #[test]
    fn pre_attrs() {
        assert_eq!(pre().width("80").render(), r#"<pre width="80"></pre>"#);
        assert_eq!(pre().cols("80").render(), r#"<pre cols="80"></pre>"#);
        assert_eq!(pre().tabindex("0").render(), r#"<pre tabindex="0"></pre>"#);
        assert_eq!(pre().wrap("soft").render(), r#"<pre wrap="soft"></pre>"#);
        assert_eq!(pre().name("code").render(), r#"<pre name="code"></pre>"#);
        assert_eq!(pre().autofocus("true").render(), r#"<pre autofocus="true"></pre>"#);
    }

    #[test]
    fn blockquote_attrs() {
        assert_eq!(blockquote().cite("http://example.com").render(), r#"<blockquote cite="http://example.com"></blockquote>"#);
    }

    #[test]
    fn ol_attrs() {
        assert_eq!(ol().type_attr("1").render(), r#"<ol type="1"></ol>"#);
        assert_eq!(ol().reversed("true").render(), r#"<ol reversed="true"></ol>"#);
        assert_eq!(ol().start("5").render(), r#"<ol start="5"></ol>"#);
    }

    #[test]
    fn ul_attrs() {
        assert_eq!(ul().type_attr("disc").render(), r#"<ul type="disc"></ul>"#);
    }

    #[test]
    fn menu_attrs() {
        assert_eq!(menu().type_attr("toolbar").render(), r#"<menu type="toolbar"></menu>"#);
        assert_eq!(menu().label("Actions").render(), r#"<menu label="Actions"></menu>"#);
    }

    #[test]
    fn li_attrs() {
        assert_eq!(li().value("3").render(), r#"<li value="3"></li>"#);
        assert_eq!(li().type_attr("a").render(), r#"<li type="a"></li>"#);
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
