use mrk::*;

#[test]
fn end_to_end_builder() {
    let html = el("a")
        .attrs(vec![attr("href").value("/")])
        .children(vec![text("Home")])
        .render();
    assert_eq!(html, "<a href=\"/\">Home</a>");
}

#[test]
fn factories_at_crate_root() {
    assert_eq!(div().render(), "<div></div>");
    assert_eq!(br().render(), "<br>");
    assert_eq!(p().children(vec![text("hi")]).render(), "<p>hi</p>");
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
