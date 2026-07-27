//! Tabular elements (`<table>`, `<tr>`, `<td>`, etc.).

use super::macros::{define_html_element, factory};

define_html_element!(HtmlTable, "table", all);
define_html_element!(HtmlCaption, "caption", all);
define_html_element!(HtmlColgroup, "colgroup", no_aria, span(r#"Number of consecutive columns the element spans (a valid positive integer; default `1`).

The element represents a group of columns; the rendered presentation may collapse this group."#));
define_html_element!(HtmlCol, "col", no_aria, span(r#"Number of consecutive columns the element spans (a valid positive integer; default `1`).

The element represents one or more columns; the rendered presentation may collapse these columns."#));
define_html_element!(HtmlTbody, "tbody", all);
define_html_element!(HtmlThead, "thead", all);
define_html_element!(HtmlTfoot, "tfoot", all);
define_html_element!(HtmlTr, "tr", all);
define_html_element!(HtmlTd, "td", all,
    colspan(r#"Number of columns the cell spans (a valid non-negative integer; default `1`).

`0` means the cell spans all remaining columns in its column group."#),
    rowspan(r#"Number of rows the cell spans (a valid non-negative integer; default `1`).

`0` means the cell spans all remaining rows in the table section."#),
    headers(r#"Space-separated list of IDs of `<th>` elements that provide headers for this cell.

The referenced `<th>` elements are exposed as the cell's headers for assistive technologies."#));
define_html_element!(HtmlTh, "th", all,
    colspan(r#"Number of columns the header cell spans (a valid non-negative integer; default `1`).

`0` means the cell spans all remaining columns in its column group."#),
    rowspan(r#"Number of rows the header cell spans (a valid non-negative integer; default `1`).

`0` means the cell spans all remaining rows in the table section."#),
    headers(r#"Space-separated list of IDs of `<th>` elements that provide headers for this header cell.

The referenced `<th>` elements are exposed as the cell's headers for assistive technologies."#),
    scope(r#"Scope of the header cell, used to associate the header with the cells it applies to.

One of:
- `row` (the header applies to the cells in its row)
- `col` (the header applies to the cells in its column)
- `rowgroup` (the header applies to the cells in its row group, e.g. `<thead>`, `<tbody>`, `<tfoot>`)
- `colgroup` (the header applies to the cells in its column group, e.g. `<colgroup>`)

Used by assistive technologies to navigate table data."#),
    abbr(r#"Abbreviated description of the header cell's content.

A short label, used in place of the cell's full content when the user agent cannot render the full content (e.g. small screens, screen readers)."#));

factory!(
    /// Create a new [`HtmlTable`] element (`<table>`).
    table, HtmlTable
);
factory!(
    /// Create a new [`HtmlCaption`] element (`<caption>`).
    caption, HtmlCaption
);
factory!(
    /// Create a new [`HtmlColgroup`] element (`<colgroup>`).
    colgroup, HtmlColgroup
);
factory!(
    /// Create a new [`HtmlCol`] element (`<col>`).
    col, HtmlCol
);
factory!(
    /// Create a new [`HtmlTbody`] element (`<tbody>`).
    tbody, HtmlTbody
);
factory!(
    /// Create a new [`HtmlThead`] element (`<thead>`).
    thead, HtmlThead
);
factory!(
    /// Create a new [`HtmlTfoot`] element (`<tfoot>`).
    tfoot, HtmlTfoot
);
factory!(
    /// Create a new [`HtmlTr`] element (`<tr>`).
    tr, HtmlTr
);
factory!(
    /// Create a new [`HtmlTd`] element (`<td>`).
    td, HtmlTd
);
factory!(
    /// Create a new [`HtmlTh`] element (`<th>`).
    th, HtmlTh
);

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
