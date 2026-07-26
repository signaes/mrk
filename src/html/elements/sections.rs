//! Section-level elements (`<body>`, `<article>`, `<nav>`, headings, etc.).

use super::macros::{define_html_element, factory};

define_html_element!(HtmlBody, "body");
define_html_element!(HtmlArticle, "article");
define_html_element!(HtmlSection, "section");
define_html_element!(HtmlNav, "nav");
define_html_element!(HtmlAside, "aside");
define_html_element!(HtmlH1, "h1");
define_html_element!(HtmlH2, "h2");
define_html_element!(HtmlH3, "h3");
define_html_element!(HtmlH4, "h4");
define_html_element!(HtmlH5, "h5");
define_html_element!(HtmlH6, "h6");
define_html_element!(HtmlHeader, "header");
define_html_element!(HtmlFooter, "footer");
define_html_element!(HtmlAddress, "address");
define_html_element!(HtmlMain, "main");
define_html_element!(HtmlSearch, "search");
define_html_element!(HtmlHgroup, "hgroup");

// Create a new [`HtmlBody`] element (`<body>`).
factory!(body, HtmlBody);
// Create a new [`HtmlArticle`] element (`<article>`).
factory!(article, HtmlArticle);
// Create a new [`HtmlSection`] element (`<section>`).
factory!(section, HtmlSection);
// Create a new [`HtmlNav`] element (`<nav>`).
factory!(nav, HtmlNav);
// Create a new [`HtmlAside`] element (`<aside>`).
factory!(aside, HtmlAside);
// Create a new [`HtmlH1`] element (`<h1>`).
factory!(h1, HtmlH1);
// Create a new [`HtmlH2`] element (`<h2>`).
factory!(h2, HtmlH2);
// Create a new [`HtmlH3`] element (`<h3>`).
factory!(h3, HtmlH3);
// Create a new [`HtmlH4`] element (`<h4>`).
factory!(h4, HtmlH4);
// Create a new [`HtmlH5`] element (`<h5>`).
factory!(h5, HtmlH5);
// Create a new [`HtmlH6`] element (`<h6>`).
factory!(h6, HtmlH6);
// Create a new [`HtmlHeader`] element (`<header>`).
factory!(header, HtmlHeader);
// Create a new [`HtmlFooter`] element (`<footer>`).
factory!(footer, HtmlFooter);
// Create a new [`HtmlAddress`] element (`<address>`).
factory!(address, HtmlAddress);
// Create a new [`HtmlMain`] element (`<main>`).
factory!(main, HtmlMain);
// Create a new [`HtmlSearch`] element (`<search>`).
factory!(search, HtmlSearch);
// Create a new [`HtmlHgroup`] element (`<hgroup>`).
factory!(hgroup, HtmlHgroup);

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
