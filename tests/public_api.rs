use mrk::*;

#[test]
fn end_to_end_builder() {
    let html = el("a")
        .attrs(vec![attr("href").value("/")])
        .children(nodes!["Home"])
        .render();
    assert_eq!(html, r#"<a href="/">Home</a>"#);
}

#[test]
fn nodes_macro_mixes_text_and_elements() {
    let html = div().children(nodes![
        "Hello, ",
        el("strong").children(nodes!["world"]),
    ]).render();
    assert_eq!(html, "<div>Hello, <strong>world</strong></div>");
}

#[test]
fn factories_at_crate_root() {
    assert_eq!(div().render(), "<div></div>");
    assert_eq!(br().render(), "<br>");
    assert_eq!(p().children(nodes!["hi"]).render(), "<p>hi</p>");
}

#[test]
fn renderable_trait_extensible() {
    struct Wrapper(&'static str);
    impl Renderable for Wrapper {
        fn render(&self) -> String {
            self.0.to_string()
        }
    }
    assert_eq!(render(Wrapper("hi")), "hi");
}

#[test]
fn display_impl_renders_html() {
    let e = div().children(nodes!["hi"]);
    assert_eq!(format!("{}", e), "<div>hi</div>");

    let n: Node = "hello".into();
    assert_eq!(format!("{}", n), "hello");
}

#[test]
fn owned_string_converts_to_node() {
    let owned = String::from("dynamic");
    let html = p().children(nodes![owned]).render();
    assert_eq!(html, "<p>dynamic</p>");
}
