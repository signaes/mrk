//! HTML tag factory functions.
//!
//! Each function returns an empty [`Element`] with the matching tag name.
//! Use the builder methods `.attrs(...)` and `.children(...)` to populate.

use crate::element::Element;

macro_rules! factory {
    ($fn_name:ident, $tag:literal) => {
        pub fn $fn_name() -> Element {
            $crate::element::el($tag)
        }
    };
}

macro_rules! factories {
    () => {};
    ($($name:ident, $tag:literal);+ $(;)?) => {
        $( factory!($name, $tag); )+
    };
}

factories! {
    html, "html";
    head, "head";
    body, "body";
    title, "title";
    base, "base";
    link, "link";
    meta, "meta";
    style, "style";

    address, "address";
    article, "article";
    aside, "aside";
    footer, "footer";
    header, "header";
    h1, "h1";
    h2, "h2";
    h3, "h3";
    h4, "h4";
    h5, "h5";
    h6, "h6";
    main, "main";
    nav, "nav";
    section, "section";

    blockquote, "blockquote";
    dd, "dd";
    div, "div";
    dl, "dl";
    dt, "dt";
    figcaption, "figcaption";
    figure, "figure";
    hr, "hr";
    li, "li";
    ol, "ol";
    p, "p";
    pre, "pre";
    ul, "ul";

    a, "a";
    abbr, "abbr";
    b, "b";
    bdi, "bdi";
    bdo, "bdo";
    br, "br";
    cite, "cite";
    code, "code";
    data, "data";
    dfn, "dfn";
    em, "em";
    i, "i";
    kbd, "kbd";
    mark, "mark";
    q, "q";
    rb, "rb";
    rp, "rp";
    rt, "rt";
    rtc, "rtc";
    ruby, "ruby";
    s, "s";
    samp, "samp";
    small, "small";
    span, "span";
    strong, "strong";
    sub, "sub";
    sup, "sup";
    time, "time";
    u, "u";
    var, "var";
    wbr, "wbr";

    del, "del";
    ins, "ins";

    area, "area";
    audio, "audio";
    embed, "embed";
    iframe, "iframe";
    img, "img";
    map, "map";
    object, "object";
    picture, "picture";
    portal, "portal";
    source, "source";
    track, "track";
    video, "video";

    math, "math";
    svg, "svg";

    caption, "caption";
    col, "col";
    colgroup, "colgroup";
    table, "table";
    tbody, "tbody";
    td, "td";
    tfoot, "tfoot";
    th, "th";
    thead, "thead";
    tr, "tr";

    button, "button";
    datalist, "datalist";
    fieldset, "fieldset";
    form, "form";
    input, "input";
    label, "label";
    legend, "legend";
    meter, "meter";
    optgroup, "optgroup";
    option, "option";
    output, "output";
    param, "param";
    progress, "progress";
    select, "select";
    textarea, "textarea";

    details, "details";
    dialog, "dialog";
    menu, "menu";
    summary, "summary";

    canvas, "canvas";
    noscript, "noscript";
    script, "script";
    template, "template";
    slot, "slot";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::element::Element;
    use crate::renderable::Renderable;

    #[test]
    fn render_table() {
        let cases: Vec<(&str, fn() -> Element, &str)> = vec![
            ("html", html, "<html></html>"),
            ("head", head, "<head></head>"),
            ("body", body, "<body></body>"),
            ("title", title, "<title></title>"),
            ("base", base, "<base>"),
            ("link", link, "<link>"),
            ("meta", meta, "<meta>"),
            ("style", style, "<style></style>"),
            ("address", address, "<address></address>"),
            ("article", article, "<article></article>"),
            ("aside", aside, "<aside></aside>"),
            ("footer", footer, "<footer></footer>"),
            ("header", header, "<header></header>"),
            ("h1", h1, "<h1></h1>"),
            ("h2", h2, "<h2></h2>"),
            ("h3", h3, "<h3></h3>"),
            ("h4", h4, "<h4></h4>"),
            ("h5", h5, "<h5></h5>"),
            ("h6", h6, "<h6></h6>"),
            ("main", main, "<main></main>"),
            ("nav", nav, "<nav></nav>"),
            ("section", section, "<section></section>"),
            ("blockquote", blockquote, "<blockquote></blockquote>"),
            ("dd", dd, "<dd></dd>"),
            ("div", div, "<div></div>"),
            ("dl", dl, "<dl></dl>"),
            ("dt", dt, "<dt></dt>"),
            ("figcaption", figcaption, "<figcaption></figcaption>"),
            ("figure", figure, "<figure></figure>"),
            ("hr", hr, "<hr>"),
            ("li", li, "<li></li>"),
            ("ol", ol, "<ol></ol>"),
            ("p", p, "<p></p>"),
            ("pre", pre, "<pre></pre>"),
            ("ul", ul, "<ul></ul>"),
            ("a", a, "<a></a>"),
            ("abbr", abbr, "<abbr></abbr>"),
            ("b", b, "<b></b>"),
            ("bdi", bdi, "<bdi></bdi>"),
            ("bdo", bdo, "<bdo></bdo>"),
            ("br", br, "<br>"),
            ("cite", cite, "<cite></cite>"),
            ("code", code, "<code></code>"),
            ("data", data, "<data></data>"),
            ("dfn", dfn, "<dfn></dfn>"),
            ("em", em, "<em></em>"),
            ("i", i, "<i></i>"),
            ("kbd", kbd, "<kbd></kbd>"),
            ("mark", mark, "<mark></mark>"),
            ("q", q, "<q></q>"),
            ("rb", rb, "<rb></rb>"),
            ("rp", rp, "<rp></rp>"),
            ("rt", rt, "<rt></rt>"),
            ("rtc", rtc, "<rtc></rtc>"),
            ("ruby", ruby, "<ruby></ruby>"),
            ("s", s, "<s></s>"),
            ("samp", samp, "<samp></samp>"),
            ("small", small, "<small></small>"),
            ("span", span, "<span></span>"),
            ("strong", strong, "<strong></strong>"),
            ("sub", sub, "<sub></sub>"),
            ("sup", sup, "<sup></sup>"),
            ("time", time, "<time></time>"),
            ("u", u, "<u></u>"),
            ("var", var, "<var></var>"),
            ("wbr", wbr, "<wbr>"),
            ("del", del, "<del></del>"),
            ("ins", ins, "<ins></ins>"),
            ("area", area, "<area>"),
            ("audio", audio, "<audio></audio>"),
            ("embed", embed, "<embed>"),
            ("iframe", iframe, "<iframe></iframe>"),
            ("img", img, "<img>"),
            ("map", map, "<map></map>"),
            ("object", object, "<object></object>"),
            ("picture", picture, "<picture></picture>"),
            ("portal", portal, "<portal></portal>"),
            ("source", source, "<source>"),
            ("track", track, "<track>"),
            ("video", video, "<video></video>"),
            ("math", math, "<math></math>"),
            ("svg", svg, "<svg></svg>"),
            ("caption", caption, "<caption></caption>"),
            ("col", col, "<col>"),
            ("colgroup", colgroup, "<colgroup></colgroup>"),
            ("table", table, "<table></table>"),
            ("tbody", tbody, "<tbody></tbody>"),
            ("td", td, "<td></td>"),
            ("tfoot", tfoot, "<tfoot></tfoot>"),
            ("th", th, "<th></th>"),
            ("thead", thead, "<thead></thead>"),
            ("tr", tr, "<tr></tr>"),
            ("button", button, "<button></button>"),
            ("datalist", datalist, "<datalist></datalist>"),
            ("fieldset", fieldset, "<fieldset></fieldset>"),
            ("form", form, "<form></form>"),
            ("input", input, "<input>"),
            ("label", label, "<label></label>"),
            ("legend", legend, "<legend></legend>"),
            ("meter", meter, "<meter></meter>"),
            ("optgroup", optgroup, "<optgroup></optgroup>"),
            ("option", option, "<option></option>"),
            ("output", output, "<output></output>"),
            ("param", param, "<param>"),
            ("progress", progress, "<progress></progress>"),
            ("select", select, "<select></select>"),
            ("textarea", textarea, "<textarea></textarea>"),
            ("details", details, "<details></details>"),
            ("dialog", dialog, "<dialog></dialog>"),
            ("menu", menu, "<menu></menu>"),
            ("summary", summary, "<summary></summary>"),
            ("canvas", canvas, "<canvas></canvas>"),
            ("noscript", noscript, "<noscript></noscript>"),
            ("script", script, "<script></script>"),
            ("template", template, "<template></template>"),
            ("slot", slot, "<slot></slot>"),
        ];

        for (name, factory, expected) in &cases {
            assert_eq!(factory().render(), *expected, "case: {name}");
        }
    }
}
