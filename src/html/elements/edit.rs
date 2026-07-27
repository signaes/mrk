//! Edit elements (`<ins>`, `<del>`).

use super::macros::{define_html_element, factory};

define_html_element!(HtmlIns, "ins", all,
    cite(r#"URL pointing to a resource that explains why the content was inserted.

Typical use: a changelog entry, ticket, or revision that justifies the edit."#),
    datetime(r#"Date and optional time when the change was inserted, as a global date or global date-time string (e.g. `2025-01-15`, `2025-01-15T13:45:00Z`).

Machine-readable; rendered text is what the user sees."#));
define_html_element!(HtmlDel, "del", all,
    cite(r#"URL pointing to a resource that explains why the content was removed.

Typical use: a changelog entry, ticket, or revision that justifies the edit."#),
    datetime(r#"Date and optional time when the change was removed, as a global date or global date-time string (e.g. `2025-01-15`, `2025-01-15T13:45:00Z`).

Machine-readable; rendered text is what the user sees."#));

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
