//! Edit elements (`<ins>`, `<del>`).

use super::macros::{define_html_element, factory};

define_html_element!(HtmlIns, "ins", cite("URL of the source of the change."), datetime("Date/time of the change."));
define_html_element!(HtmlDel, "del", cite("URL of the source of the change."), datetime("Date/time of the change."));

// Create a new [`HtmlIns`] element (`<ins>`).
factory!(ins, HtmlIns);
// Create a new [`HtmlDel`] element (`<del>`).
factory!(del, HtmlDel);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ins_element() {
        assert_eq!(ins().render(), "<ins></ins>");
    }

    #[test]
    fn del_element() {
        assert_eq!(del().render(), "<del></del>");
    }

    #[test]
    fn ins_attrs() {
        assert_eq!(ins().cite("http://example.com").render(), r#"<ins cite="http://example.com"></ins>"#);
        assert_eq!(ins().datetime("2024-01-01").render(), r#"<ins datetime="2024-01-01"></ins>"#);
    }

    #[test]
    fn del_attrs() {
        assert_eq!(del().cite("http://example.com").render(), r#"<del cite="http://example.com"></del>"#);
        assert_eq!(del().datetime("2024-01-01").render(), r#"<del datetime="2024-01-01"></del>"#);
    }
}
