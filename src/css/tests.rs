use crate::css::properties::Value;
use crate::css::selector::{Selector, PseudoSelector};
use crate::css::values::{Color, CssString, Length};
use crate::css::{AtRule, Declaration, StyleSheet};
use crate::renderable::Renderable;

#[test]
fn empty_stylesheet_renders_to_empty_string() {
    let sheet = StyleSheet::new().build();
    assert_eq!(sheet.render(), "");
}

#[test]
fn empty_stylesheet_converts_to_node_raw() {
    let sheet = StyleSheet::new().build();
    let node: crate::node::Node = sheet.into();
    let debug = format!("{:?}", node);
    assert!(debug.contains("Raw"));
}

#[test]
fn single_rule_with_one_declaration() {
    let sheet = StyleSheet::new()
        .rule(|r| r.selector(Selector::class("btn")).color(Color::named("red")))
        .build();
    let css = sheet.render();
    assert_eq!(css, ".btn {\n    color: red;\n  }");
}

#[test]
fn single_rule_with_multiple_declarations() {
    let sheet = StyleSheet::new()
        .rule(|r| {
            r.selector(Selector::class("card"))
                .background_color(Color::named("white"))
                .padding(Length::px(16.0))
                .border_radius(Length::px(8.0))
        })
        .build();
    let css = sheet.render();
    assert_eq!(
        css,
        ".card {\n    background-color: white;\n    padding: 16px;\n    border-radius: 8px;\n  }"
    );
}

#[test]
fn rule_with_multiple_selectors() {
    let sheet = StyleSheet::new()
        .rule(|r| {
            r.selector(Selector::class("btn"))
                .selector(Selector::class("btn-primary"))
                .color(Color::named("blue"))
        })
        .build();
    let css = sheet.render();
    assert_eq!(css, ".btn, .btn-primary {\n    color: blue;\n  }");
}

#[test]
fn multiple_rules() {
    let sheet = StyleSheet::new()
        .rule(|r| r.selector(Selector::type_("h1")).font_size(Length::px(32.0)))
        .rule(|r| r.selector(Selector::type_("h2")).font_size(Length::px(24.0)))
        .build();
    let css = sheet.render();
    assert_eq!(
        css,
        "h1 {\n    font-size: 32px;\n  }\nh2 {\n    font-size: 24px;\n  }"
    );
}

#[test]
fn rule_with_nesting() {
    let sheet = StyleSheet::new()
        .rule(|r| {
            r.selector(Selector::class("card"))
                .color(Color::named("black"))
                .nest(|n| {
                    n.selector(Selector::Compound(vec![
                        Selector::NestingRef,
                        Selector::Pseudo(PseudoSelector::Class("hover".into())),
                    ]))
                    .color(Color::named("gray"))
                })
        })
        .build();
    let css = sheet.render();
    assert_eq!(
        css,
        ".card {\n    color: black;\n  \n    &:hover {\n      color: gray;\n    }\n}"
    );
}

#[test]
fn rule_with_nest_at_rule() {
    let sheet = StyleSheet::new()
        .rule(|r| {
            r.selector(Selector::class("responsive"))
                .nest_at_rule(AtRule::media("(max-width: 600px)").rule(|r| {
                    r.selector(Selector::class("&")).width(Length::pct(100.0))
                }).build())
        })
        .build();
    let css = sheet.render();
    assert!(css.contains("@media (max-width: 600px)"));
    assert!(css.contains("width: 100%"));
}

#[test]
fn media_at_rule_with_rules() {
    let sheet = StyleSheet::new()
        .at_rule(
            AtRule::media("(min-width: 800px)")
                .rule(|r| r.selector(Selector::class("container")).width(Length::px(750.0)))
                .build(),
        )
        .build();
    let css = sheet.render();
    assert_eq!(
        css,
        "@media (min-width: 800px) {\n  .container {\n    width: 750px;\n  }\n}"
    );
}

#[test]
fn font_face_at_rule() {
    let sheet = StyleSheet::new()
        .at_rule(
            AtRule::font_face()
                .property("font-family", CssString::from("Open Sans"))
                .property("font-weight", Value::Integer(400.into()))
                .property("src", Value::Raw("url('open-sans.woff2')".into()))
                .build(),
        )
        .build();
    let css = sheet.render();
    assert!(css.starts_with("@font-face {"));
    assert!(css.contains("font-family: \"Open Sans\""));
    assert!(css.contains("font-weight: 400"));
    assert!(css.contains("src: url('open-sans.woff2')"));
}

#[test]
fn keyframes_at_rule() {
    let sheet = StyleSheet::new()
        .at_rule(
            AtRule::keyframes("fade-in")
                .rule(|r| r.selector(Selector::class("from")).opacity(Value::Number(0.0.into())))
                .rule(|r| r.selector(Selector::class("to")).opacity(Value::Number(1.0.into())))
                .build(),
        )
        .build();
    let css = sheet.render();
    assert_eq!(
        css,
        "@keyframes fade-in {\n  .from {\n    opacity: 0;\n  }\n  .to {\n    opacity: 1;\n  }\n}"
    );
}

#[test]
fn supports_at_rule() {
    let sheet = StyleSheet::new()
        .at_rule(
            AtRule::supports("(display: grid)")
                .rule(|r| r.selector(Selector::class("grid")).display("grid"))
                .build(),
        )
        .build();
    let css = sheet.render();
    assert_eq!(
        css,
        "@supports (display: grid) {\n  .grid {\n    display: grid;\n  }\n}"
    );
}

#[test]
fn charset_at_rule() {
    let sheet = StyleSheet::new()
        .at_rule(AtRule::charset("UTF-8"))
        .build();
    assert_eq!(sheet.render(), "@charset \"UTF-8\";");
}

#[test]
fn namespace_at_rule() {
    let sheet = StyleSheet::new()
        .at_rule(AtRule::namespace("http://www.w3.org/2000/svg"))
        .build();
    assert_eq!(sheet.render(), "@namespace \"http://www.w3.org/2000/svg\";");
}

#[test]
fn import_at_rule() {
    let sheet = StyleSheet::new()
        .at_rule(AtRule::import("reset.css"))
        .build();
    assert_eq!(sheet.render(), "@import \"reset.css\";");
}

#[test]
fn mixed_rules_and_at_rules() {
    let sheet = StyleSheet::new()
        .rule(|r| r.selector(Selector::type_("body")).margin(Length::px(0.0)))
        .at_rule(
            AtRule::media("print")
                .rule(|r| r.selector(Selector::type_("body")).font_size(Length::pt(12.0)))
                .build(),
        )
        .build();
    let css = sheet.render();
    assert!(css.contains("body {\n    margin: 0px;"));
    assert!(css.contains("@media print"));
}

#[test]
fn chain_rules_with_all_property_types() {
    let sheet = StyleSheet::new()
        .rule(|r| {
            r.selector(Selector::class("demo"))
                .color(Color::named("red"))
                .background_color(Color::named("blue"))
                .font_size(Length::px(16.0))
                .margin(Length::px(10.0))
                .padding(Length::px(20.0))
                .display("flex")
                .position("relative")
                .opacity(0.5f32)
                .z_index(Value::Integer(100.into()))
        })
        .build();
    let css = sheet.render();
    assert!(css.contains("color: red"));
    assert!(css.contains("font-size: 16px"));
    assert!(css.contains("opacity: 0.5"));
    assert!(css.contains("z-index: 100"));
}

#[test]
fn rule_with_shorthand_and_longhand() {
    let sheet = StyleSheet::new()
        .rule(|r| {
            r.selector(Selector::class("box"))
                .margin(Length::px(10.0))
                .margin_top(Length::px(0.0))
        })
        .build();
    let css = sheet.render();
    assert!(css.contains("margin: 10px;"));
    assert!(css.contains("margin-top: 0px;"));
}

#[test]
fn using_decl_directly() {
    let sheet = StyleSheet::new()
        .rule(|r| {
            r.selector(Selector::class("custom"))
                .decl(Declaration::new("--my-var", Value::Number(42.0.into())))
        })
        .build();
    let css = sheet.render();
    assert!(css.contains("--my-var: 42"));
}

#[test]
fn nested_blocks_preserve_declarations() {
    let sheet = StyleSheet::new()
        .rule(|r| {
            r.selector(Selector::class("parent"))
                .color(Color::named("black"))
                .nest(|n| n.selector(Selector::class("& .child")).color(Color::named("gray")))
                .font_size(Length::px(14.0))
        })
        .build();
    let css = sheet.render();
    assert!(css.contains("color: black;"));
    assert!(css.contains("font-size: 14px;"));
    assert!(css.contains("& .child"));
    assert!(css.contains("color: gray;"));
}
