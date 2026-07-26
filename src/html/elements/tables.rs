//! Tabular elements (`<table>`, `<tr>`, `<td>`, etc.).

use super::macros::{define_html_element, factory};

define_html_element!(HtmlTable, "table");
define_html_element!(HtmlCaption, "caption");
define_html_element!(HtmlColgroup, "colgroup", span("Number of columns to span."));
define_html_element!(HtmlCol, "col", span("Number of columns to span."));
define_html_element!(HtmlTbody, "tbody");
define_html_element!(HtmlThead, "thead");
define_html_element!(HtmlTfoot, "tfoot");
define_html_element!(HtmlTr, "tr");
define_html_element!(HtmlTd, "td",
    colspan("Number of columns to span."),
    rowspan("Number of rows to span."),
    headers("Space-separated header IDs."),
    scope("Header scope (col, row, colgroup, rowgroup)."),
    abbr("Abbreviated header text."));
define_html_element!(HtmlTh, "th",
    colspan("Number of columns to span."),
    rowspan("Number of rows to span."),
    headers("Space-separated header IDs."),
    scope("Header scope (col, row, colgroup, rowgroup)."),
    abbr("Abbreviated header text."));

// Create a new [`HtmlTable`] element (`<table>`).
factory!(table, HtmlTable);
// Create a new [`HtmlCaption`] element (`<caption>`).
factory!(caption, HtmlCaption);
// Create a new [`HtmlColgroup`] element (`<colgroup>`).
factory!(colgroup, HtmlColgroup);
// Create a new [`HtmlCol`] element (`<col>`).
factory!(col, HtmlCol);
// Create a new [`HtmlTbody`] element (`<tbody>`).
factory!(tbody, HtmlTbody);
// Create a new [`HtmlThead`] element (`<thead>`).
factory!(thead, HtmlThead);
// Create a new [`HtmlTfoot`] element (`<tfoot>`).
factory!(tfoot, HtmlTfoot);
// Create a new [`HtmlTr`] element (`<tr>`).
factory!(tr, HtmlTr);
// Create a new [`HtmlTd`] element (`<td>`).
factory!(td, HtmlTd);
// Create a new [`HtmlTh`] element (`<th>`).
factory!(th, HtmlTh);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_element() {
        assert_eq!(table().render(), "<table></table>");
    }

    #[test]
    fn caption_element() {
        assert_eq!(caption().render(), "<caption></caption>");
    }

    #[test]
    fn colgroup_attrs() {
        assert_eq!(colgroup().span("3").render(), r#"<colgroup span="3"></colgroup>"#);
    }

    #[test]
    fn col_attrs() {
        assert_eq!(col().span("2").render(), r#"<col span="2">"#);
    }

    #[test]
    fn tbody_element() {
        assert_eq!(tbody().render(), "<tbody></tbody>");
    }

    #[test]
    fn thead_element() {
        assert_eq!(thead().render(), "<thead></thead>");
    }

    #[test]
    fn tfoot_element() {
        assert_eq!(tfoot().render(), "<tfoot></tfoot>");
    }

    #[test]
    fn tr_element() {
        assert_eq!(tr().render(), "<tr></tr>");
    }

    #[test]
    fn td_attrs() {
        assert_eq!(td().colspan("2").render(), r#"<td colspan="2"></td>"#);
        assert_eq!(td().rowspan("3").render(), r#"<td rowspan="3"></td>"#);
        assert_eq!(td().headers("h1 h2").render(), r#"<td headers="h1 h2"></td>"#);
        assert_eq!(td().scope("col").render(), r#"<td scope="col"></td>"#);
        assert_eq!(td().abbr("Name").render(), r#"<td abbr="Name"></td>"#);
    }

    #[test]
    fn th_attrs() {
        assert_eq!(th().colspan("2").render(), r#"<th colspan="2"></th>"#);
        assert_eq!(th().rowspan("3").render(), r#"<th rowspan="3"></th>"#);
        assert_eq!(th().headers("h1 h2").render(), r#"<th headers="h1 h2"></th>"#);
        assert_eq!(th().scope("row").render(), r#"<th scope="row"></th>"#);
        assert_eq!(th().abbr("Full").render(), r#"<th abbr="Full"></th>"#);
    }
}
