//! Typed HTML wrappers for the `components` module.
//!
//! This sub-module is **independent** of `mrk::html` — the wrappers
//! here are defined separately and have their own attribute setters.
//! Each wrapper accepts `impl IntoExpr` so dynamic attribute values
//! (e.g. `prop("class")`) work directly.
//!
//! Every typed wrapper provides a common set of attribute methods
//! (`class`, `id`, `style`, `href`, `src`, `type_attr`, `title`,
//! `lang`, `dir`, `tabindex`, `role`, `value`, `placeholder`,
//! `name`, `alt`, etc.). For tag-specific attributes that aren't
//! covered, use the generic
//! [`ComponentElement::attr_dynamic`](crate::components::ComponentElement::attr_dynamic)
//! method via the wrapped `ComponentElement`.
//!
//! # Usage
//!
//! ```ignore
//! use mrk::components::html::{div, span};
//!
//! component!(Card, {
//!     div().class(prop("class")).children(nodes![
//!         span().children(nodes![text!(prop("translation_text"))]),
//!     ])
//! });
//! ```
#![allow(missing_docs)]

use crate::components::IntoExpr;

/// Internal: define a typed HTML wrapper for a tag. Each wrapper
/// exposes the most common attributes as methods, all accepting
/// `impl IntoExpr` for dynamic values.
#[macro_export]
#[doc(hidden)]
macro_rules! __component_html_define {
    ($struct:ident, $factory:ident, $tag:literal $(,)?) => {
        $crate::components::element::__define_component_wrapper!(
            $struct,
            $factory,
            $tag,
            // Most common HTML attributes
            class => "class",
            id => "id",
            style => "style",
            href => "href",
            src => "src",
            alt => "alt",
            type_attr => "type",
            title_attr => "title",
            value => "value",
            placeholder => "placeholder",
            name => "name",
            role => "role",
            lang => "lang",
            dir => "dir",
            tabindex => "tabindex",
            hidden => "hidden",
            disabled => "disabled",
            readonly => "readonly",
            required => "required",
            checked => "checked",
            selected => "selected",
            autofocus => "autofocus",
            multiple => "multiple",
            target => "target",
            rel => "rel",
            for_attr => "for",
            action => "action",
            method => "method",
            colspan => "colspan",
            rowspan => "rowspan",
            headers => "headers",
            scope => "scope",
            datetime => "datetime",
            cite => "cite",
            width => "width",
            height => "height",
            min => "min",
            max => "max",
            step => "step",
            minlength => "minlength",
            maxlength => "maxlength",
            pattern => "pattern",
            autocomplete => "autocomplete",
            spellcheck => "spellcheck",
            translate => "translate",
            contenteditable => "contenteditable",
            draggable => "draggable",
            enterkeyhint => "enterkeyhint",
            inputmode => "inputmode",
            slot => "slot",
            part => "part",
            is_attr => "is",
            nonce => "nonce",
            form => "form",
        );
    };
}

// ============================================================================
// Document structure
// ============================================================================

__component_html_define!(Html, html, "html");
__component_html_define!(Head, head, "head");
__component_html_define!(Title, title, "title");
__component_html_define!(Base, base, "base");
__component_html_define!(Link, link, "link");
__component_html_define!(Meta, meta, "meta");
__component_html_define!(Style, style, "style");
__component_html_define!(Body, body, "body");

// ============================================================================
// Sections
// ============================================================================

__component_html_define!(Article, article, "article");
__component_html_define!(Section, section, "section");
__component_html_define!(Nav, nav, "nav");
__component_html_define!(Aside, aside, "aside");
__component_html_define!(H1, h1, "h1");
__component_html_define!(H2, h2, "h2");
__component_html_define!(H3, h3, "h3");
__component_html_define!(H4, h4, "h4");
__component_html_define!(H5, h5, "h5");
__component_html_define!(H6, h6, "h6");
__component_html_define!(Header, header, "header");
__component_html_define!(Footer, footer, "footer");
__component_html_define!(Address, address, "address");
__component_html_define!(Main, main, "main");
__component_html_define!(Search, search, "search");
__component_html_define!(Hgroup, hgroup, "hgroup");

// ============================================================================
// Grouping
// ============================================================================

__component_html_define!(P, p, "p");
__component_html_define!(Hr, hr, "hr");
__component_html_define!(Pre, pre, "pre");
__component_html_define!(Blockquote, blockquote, "blockquote");
__component_html_define!(Ol, ol, "ol");
__component_html_define!(Ul, ul, "ul");
__component_html_define!(Menu, menu, "menu");
__component_html_define!(Li, li, "li");
__component_html_define!(Dl, dl, "dl");
__component_html_define!(Dt, dt, "dt");
__component_html_define!(Dd, dd, "dd");
__component_html_define!(Figure, figure, "figure");
__component_html_define!(Figcaption, figcaption, "figcaption");
__component_html_define!(Div, div, "div");

// ============================================================================
// Text-level
// ============================================================================

__component_html_define!(A, a, "a");
__component_html_define!(Em, em, "em");
__component_html_define!(Strong, strong, "strong");
__component_html_define!(Small, small, "small");
__component_html_define!(S, s, "s");
__component_html_define!(Cite, cite, "cite");
__component_html_define!(Q, q, "q");
__component_html_define!(Dfn, dfn, "dfn");
__component_html_define!(Abbr, abbr, "abbr");
__component_html_define!(Ruby, ruby, "ruby");
__component_html_define!(Rt, rt, "rt");
__component_html_define!(Rp, rp, "rp");
__component_html_define!(Data, data, "data");
__component_html_define!(Time, time, "time");
__component_html_define!(Code, code, "code");
__component_html_define!(Var, var, "var");
__component_html_define!(Samp, samp, "samp");
__component_html_define!(Kbd, kbd, "kbd");
__component_html_define!(Sub, sub, "sub");
__component_html_define!(Sup, sup, "sup");
__component_html_define!(I, i, "i");
__component_html_define!(B, b, "b");
__component_html_define!(U, u, "u");
__component_html_define!(Mark, mark, "mark");
__component_html_define!(Bdi, bdi, "bdi");
__component_html_define!(Bdo, bdo, "bdo");
__component_html_define!(Span, span, "span");
__component_html_define!(Br, br, "br");
__component_html_define!(Wbr, wbr, "wbr");

// ============================================================================
// Edits
// ============================================================================

__component_html_define!(Ins, ins, "ins");
__component_html_define!(Del, del, "del");

// ============================================================================
// Embedded
// ============================================================================

__component_html_define!(Picture, picture, "picture");
__component_html_define!(Source, source, "source");
__component_html_define!(Img, img, "img");
__component_html_define!(Iframe, iframe, "iframe");
__component_html_define!(Embed, embed, "embed");
__component_html_define!(Object, object, "object");
__component_html_define!(Video, video, "video");
__component_html_define!(Audio, audio, "audio");
__component_html_define!(Track, track, "track");
__component_html_define!(Map, map, "map");
__component_html_define!(Area, area, "area");

// ============================================================================
// Tables
// ============================================================================

__component_html_define!(Table, table, "table");
__component_html_define!(Caption, caption, "caption");
__component_html_define!(Colgroup, colgroup, "colgroup");
__component_html_define!(Col, col, "col");
__component_html_define!(Tbody, tbody, "tbody");
__component_html_define!(Thead, thead, "thead");
__component_html_define!(Tfoot, tfoot, "tfoot");
__component_html_define!(Tr, tr, "tr");
__component_html_define!(Td, td, "td");
__component_html_define!(Th, th, "th");

// ============================================================================
// Forms
// ============================================================================

__component_html_define!(Form, form, "form");
__component_html_define!(Label, label, "label");
__component_html_define!(Input, input, "input");
__component_html_define!(Button, button, "button");
__component_html_define!(Select, select, "select");
__component_html_define!(Datalist, datalist, "datalist");
__component_html_define!(Optgroup, optgroup, "optgroup");
__component_html_define!(Option, option, "option");
__component_html_define!(Textarea, textarea, "textarea");
__component_html_define!(Output, output, "output");
__component_html_define!(Progress, progress, "progress");
__component_html_define!(Meter, meter, "meter");
__component_html_define!(Fieldset, fieldset, "fieldset");
__component_html_define!(Legend, legend, "legend");

// ============================================================================
// Interactive
// ============================================================================

__component_html_define!(Details, details, "details");
__component_html_define!(Summary, summary, "summary");
__component_html_define!(Dialog, dialog, "dialog");
__component_html_define!(Script, script, "script");
__component_html_define!(Noscript, noscript, "noscript");
__component_html_define!(Template, template, "template");
__component_html_define!(Slot, slot, "slot");
__component_html_define!(Canvas, canvas, "canvas");

// ============================================================================
// Foreign
// ============================================================================

__component_html_define!(Math, math, "math");
__component_html_define!(Svg, svg, "svg");

// Factory for arbitrary tags via `el()`.
pub use crate::el;