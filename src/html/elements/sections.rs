//! Section-level elements (`<body>`, `<article>`, `<nav>`, headings, etc.).

use super::macros::{define_html_element, factory};

define_html_element!(HtmlBody, "body", all);
define_html_element!(HtmlArticle, "article", all);
define_html_element!(HtmlSection, "section", all);
define_html_element!(HtmlNav, "nav", all);
define_html_element!(HtmlAside, "aside", all);
define_html_element!(HtmlH1, "h1", all);
define_html_element!(HtmlH2, "h2", all);
define_html_element!(HtmlH3, "h3", all);
define_html_element!(HtmlH4, "h4", all);
define_html_element!(HtmlH5, "h5", all);
define_html_element!(HtmlH6, "h6", all);
define_html_element!(HtmlHeader, "header", all);
define_html_element!(HtmlFooter, "footer", all);
define_html_element!(HtmlAddress, "address", all);
define_html_element!(HtmlMain, "main", all);
define_html_element!(HtmlSearch, "search", all);
define_html_element!(HtmlHgroup, "hgroup", all);

factory!(
    /// Create a new [`HtmlBody`] element (`<body>`).
    body, HtmlBody
);
factory!(
    /// Create a new [`HtmlArticle`] element (`<article>`).
    article, HtmlArticle
);
factory!(
    /// Create a new [`HtmlSection`] element (`<section>`).
    section, HtmlSection
);
factory!(
    /// Create a new [`HtmlNav`] element (`<nav>`).
    nav, HtmlNav
);
factory!(
    /// Create a new [`HtmlAside`] element (`<aside>`).
    aside, HtmlAside
);
factory!(
    /// Create a new [`HtmlH1`] element (`<h1>`).
    h1, HtmlH1
);
factory!(
    /// Create a new [`HtmlH2`] element (`<h2>`).
    h2, HtmlH2
);
factory!(
    /// Create a new [`HtmlH3`] element (`<h3>`).
    h3, HtmlH3
);
factory!(
    /// Create a new [`HtmlH4`] element (`<h4>`).
    h4, HtmlH4
);
factory!(
    /// Create a new [`HtmlH5`] element (`<h5>`).
    h5, HtmlH5
);
factory!(
    /// Create a new [`HtmlH6`] element (`<h6>`).
    h6, HtmlH6
);
factory!(
    /// Create a new [`HtmlHeader`] element (`<header>`).
    header, HtmlHeader
);
factory!(
    /// Create a new [`HtmlFooter`] element (`<footer>`).
    footer, HtmlFooter
);
factory!(
    /// Create a new [`HtmlAddress`] element (`<address>`).
    address, HtmlAddress
);
factory!(
    /// Create a new [`HtmlMain`] element (`<main>`).
    main, HtmlMain
);
factory!(
    /// Create a new [`HtmlSearch`] element (`<search>`).
    search, HtmlSearch
);
factory!(
    /// Create a new [`HtmlHgroup`] element (`<hgroup>`).
    hgroup, HtmlHgroup
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_element() {
        assert_eq!(body().render(), "<body></body>");
    }

    #[test]
    fn article_element() {
        assert_eq!(article().render(), "<article></article>");
    }

    #[test]
    fn section_element() {
        assert_eq!(section().render(), "<section></section>");
    }

    #[test]
    fn nav_element() {
        assert_eq!(nav().render(), "<nav></nav>");
    }

    #[test]
    fn aside_element() {
        assert_eq!(aside().render(), "<aside></aside>");
    }

    #[test]
    fn heading_elements() {
        assert_eq!(h1().render(), "<h1></h1>");
        assert_eq!(h2().render(), "<h2></h2>");
        assert_eq!(h3().render(), "<h3></h3>");
        assert_eq!(h4().render(), "<h4></h4>");
        assert_eq!(h5().render(), "<h5></h5>");
        assert_eq!(h6().render(), "<h6></h6>");
    }

    #[test]
    fn header_element() {
        assert_eq!(header().render(), "<header></header>");
    }

    #[test]
    fn footer_element() {
        assert_eq!(footer().render(), "<footer></footer>");
    }

    #[test]
    fn address_element() {
        assert_eq!(address().render(), "<address></address>");
    }

    #[test]
    fn main_element() {
        assert_eq!(main().render(), "<main></main>");
    }

    #[test]
    fn search_element() {
        assert_eq!(search().render(), "<search></search>");
    }

    #[test]
    fn hgroup_element() {
        assert_eq!(hgroup().render(), "<hgroup></hgroup>");
    }
}
