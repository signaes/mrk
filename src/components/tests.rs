// Tests for the `components` module. Lives in a separate file inside the
// `components/` directory so the file stays focused on tests and the
// module's other implementation files stay focused on production code.

use std::borrow::Cow;
use std::collections::HashMap;

use crate::attributes::attr;
use crate::component;
use crate::components::{
    prop, Component, ComponentAttribute, ComponentElement, Expr, IntoExpr, MatchArm, Number,
    NumberKind, PropType, Props, RenderError, WrappedAttribute,
};
use crate::list;
use crate::nodes;

use crate::text;
use crate::element::{el, Element};
use crate::node::Node;
use crate::renderable::Renderable;

#[cfg(feature = "ir")]
use crate::ir::Mrk;

// =================== Props ===================

#[test]
fn props_new_is_empty() {
    let p = Props::new();
    assert!(p.is_empty());
    assert_eq!(p.len(), 0);
}

#[test]
fn props_insert_and_get() {
    let mut p = Props::new();
    p.insert("name", PropType::String("Alice".into()));
    p.insert("admin", PropType::Bool(true));
    assert_eq!(p.len(), 2);
    let is_string = matches!(p.get("name"), Some(PropType::String(_)));
    assert!(is_string);
    let is_missing = p.get("missing").is_none();
    assert!(is_missing);
}

#[test]
fn props_from_iter() {
    let p: Props = vec![
        (Cow::Borrowed("a"), PropType::String("x".into())),
        (Cow::Borrowed("b"), PropType::Bool(true)),
    ]
    .into_iter()
    .collect();
    assert_eq!(p.len(), 2);
}

#[test]
fn props_from_hashmap() {
    let mut m = HashMap::new();
    m.insert(Cow::Borrowed("a"), PropType::String("x".into()));
    let p: Props = m.into_iter().collect();
    assert_eq!(p.len(), 1);
}

#[test]
fn props_insert_and_get_variants() {
    let mut p = Props::new();
    p.insert("n", PropType::Number(Number::int("42")));
    p.insert("d", PropType::Dictionary(HashMap::new()));
    p.insert("l", PropType::List(vec![]));
    let is_num = matches!(p.get("n"), Some(PropType::Number(_)));
    let is_dict = matches!(p.get("d"), Some(PropType::Dictionary(_)));
    let is_list = matches!(p.get("l"), Some(PropType::List(_)));
    assert!(is_num);
    assert!(is_dict);
    assert!(is_list);
}

// =================== PropType / Number ===================

#[test]
fn number_int_and_float() {
    let i = Number::int("42");
    let f = Number::float("3.14");
    let s_i = format!("{:?}", i);
    let s_f = format!("{:?}", f);
    assert!(s_i.contains("42"));
    assert!(s_f.contains("3.14"));
}

#[test]
fn number_parse_i64() {
    let n: i64 = "123".parse().unwrap();
    assert_eq!(n, 123);
}

#[test]
fn number_parse_f64() {
    let n: f64 = "1.5".parse().unwrap();
    assert!((n - 1.5).abs() < 0.0001);
}

#[test]
fn prop_type_type_name() {
    assert_eq!(PropType::String("x".into()).type_name(), "string");
    assert_eq!(PropType::Bool(true).type_name(), "bool");
    assert_eq!(PropType::Number(Number::int("1")).type_name(), "number");
    assert_eq!(PropType::List(vec![]).type_name(), "list");
    assert_eq!(PropType::Dictionary(HashMap::new()).type_name(), "dictionary");
}

#[test]
fn prop_type_to_text_string() {
    let s = PropType::String("hi".into()).to_text();
    assert_eq!(s.as_ref(), "hi");
}

#[test]
fn prop_type_to_text_bool() {
    assert_eq!(PropType::Bool(true).to_text().as_ref(), "true");
    assert_eq!(PropType::Bool(false).to_text().as_ref(), "false");
}

#[test]
fn prop_type_to_text_number() {
    assert_eq!(PropType::Number(Number::int("42")).to_text().as_ref(), "42");
}

#[test]
fn prop_type_to_text_list() {
    let s = PropType::List(vec![PropType::String("a".into())]).to_text();
    assert_eq!(s.as_ref(), "a");
}

#[test]
fn prop_type_to_text_dictionary() {
    let mut m = HashMap::new();
    m.insert(Cow::Borrowed("k"), PropType::String("v".into()));
    let s = PropType::Dictionary(m).to_text();
    assert!(!s.is_empty());
}

// =================== IntoExpr ===================

#[test]
fn into_expr_expr() {
    let e: Expr = Expr::Prop("x".into()).into_expr();
    assert!(matches!(e, Expr::Prop(_)));
}

#[test]
fn into_expr_box() {
    let b: Box<Expr> = Box::new(Expr::Prop("x".into()));
    let e = b.into_expr();
    assert!(matches!(e, Expr::Prop(_)));
}

#[test]
fn into_expr_element() {
    let e: Expr = Element::new("div").into_expr();
    let is_lit = matches!(e, Expr::Literal(ref el) if el.name == "div");
    assert!(is_lit);
}

#[test]
fn into_expr_node() {
    let n = Node::Text("x".into());
    let e: Expr = n.into_expr();
    let is_lc = matches!(e, Expr::LiteralChildren(_));
    assert!(is_lc);
}

#[test]
fn into_expr_for_node_ref() {
    let n = Node::Text("x".into());
    let n_ref: &Node = &n;
    let e: Expr = n_ref.into_expr();
    let is_lc = matches!(e, Expr::LiteralChildren(_));
    assert!(is_lc);
}

#[test]
fn into_expr_str() {
    let e: Expr = "hello".into_expr();
    let is_lc = matches!(e, Expr::LiteralChildren(_));
    assert!(is_lc);
}

#[test]
fn into_expr_component_element() {
    use crate::components::html::Div;
    let e: Expr = Div::new().into_expr();
    let is_wrap = matches!(e, Expr::Wrap { ref name, .. } if name == "div");
    assert!(is_wrap, "expected Wrap(div), got: {e:?}");
}

// =================== list! macro ===================

#[test]
fn list_macro_empty() {
    let e: Expr = list![];
    let is_empty = matches!(e, Expr::List(ref items) if items.is_empty());
    assert!(is_empty);
}

#[test]
fn list_macro_mixed() {
    let e: Expr = list![prop("a"), prop("b")];
    let is_list = matches!(e, Expr::List(ref items) if items.len() == 2);
    assert!(is_list);
}

// =================== prop() helper ===================

#[test]
fn prop_helper() {
    let e: Expr = prop("k");
    let is_prop = matches!(e, Expr::Prop(ref k) if k == "k");
    assert!(is_prop);
}

#[test]
fn prop_runtime_key() {
    let key = format!("dynamic-{}", 1);
    let e: Expr = prop(key);
    let is_prop = matches!(e, Expr::Prop(ref k) if k == "dynamic-1");
    assert!(is_prop);
}

// =================== ComponentElement ===================

#[test]
fn component_element_new() {
    let e = ComponentElement::new("div");
    assert_eq!(e.name, "div");
    assert!(e.attributes.is_empty());
    assert!(e.children.is_empty());
}

#[test]
fn component_element_attr_boolean() {
    let e = ComponentElement::new("div").attr("enabled");
    assert_eq!(e.attributes.len(), 1);
    let is_static = matches!(
        &e.attributes[0],
        ComponentAttribute::Static(a) if a.key == "enabled"
    );
    assert!(is_static);
}

#[test]
fn component_element_attr_dynamic() {
    let e = ComponentElement::new("div").attr_dynamic("class", prop("class"));
    assert_eq!(e.attributes.len(), 1);
    let is_dyn = matches!(
        &e.attributes[0],
        ComponentAttribute::Dynamic { key, .. } if key == "class"
    );
    assert!(is_dyn);
}

#[test]
fn component_element_children() {
    let e = ComponentElement::new("div").children(vec![Node::Text("hi".into())]);
    assert_eq!(e.children.len(), 1);
}

#[test]
fn component_element_into_expr() {
    let e: Expr = ComponentElement::new("div").into_expr();
    let is_wrap = matches!(e, Expr::Wrap { ref name, ref attrs, ref body }
        if name == "div" && attrs.is_empty() && body.is_empty());
    assert!(is_wrap, "expected Wrap(div, [], []), got: {e:?}");
}

// =================== component! macro ===================

#[test]
fn component_macro_ident_form() {
    let c = crate::component!(MyComp, { prop("name") });
    assert_eq!(c.name, "MyComp");
    let is_prop = matches!(c.expr, Expr::Prop(_));
    assert!(is_prop);
}

#[test]
fn component_macro_literal_form() {
    let c = crate::component!("my-comp", { prop("name") });
    assert_eq!(c.name, "my-comp");
    let is_prop = matches!(c.expr, Expr::Prop(_));
    assert!(is_prop);
}

// =================== HTML typed wrappers ===================

#[cfg(feature = "components")]
mod html_tests {
    use super::*;
    use crate::components::html::{div, span};

    #[test]
    fn div_new() {
        let d = div();
        assert_eq!(d.0.name, "div");
    }

    #[test]
    fn div_class_dynamic() {
        let c = component!("C", {
            div().class(prop("class"))
        });
        let mut p = Props::new();
        p.insert("class", PropType::String("my-class".into()));
        let nodes = c.render(&p).expect("render");
        let html: String = nodes.iter().map(|n| n.render()).collect();
        assert_eq!(html, r#"<div class="my-class"></div>"#);
    }

    #[test]
    fn div_class_static_string() {
        // String literals are valid IntoExpr via &str impl
        let c = component!("C", {
            div().class("static-class")
        });
        let nodes = c.render(&Props::new()).expect("render");
        let html: String = nodes.iter().map(|n| n.render()).collect();
        assert_eq!(html, r#"<div class="static-class"></div>"#);
    }

    #[test]
    fn div_id_dynamic() {
        let c = component!("C", {
            div().id(prop("id"))
        });
        let mut p = Props::new();
        p.insert("id", PropType::String("main".into()));
        let nodes = c.render(&p).expect("render");
        let html: String = nodes.iter().map(|n| n.render()).collect();
        assert_eq!(html, r#"<div id="main"></div>"#);
    }

    #[test]
    fn div_with_children() {
        let c = component!("C", {
            div().children(vec![Node::Text("hello".into())])
        });
        let nodes = c.render(&Props::new()).expect("render");
        let html: String = nodes.iter().map(|n| n.render()).collect();
        assert_eq!(html, "<div>hello</div>");
    }

    #[test]
    fn user_specified_syntax() {
        use crate::components::html::div as div_fn;

        let inner_el = el("custom");
        let inner_span_el = el("span");
        let c = component!(Card, {
            div_fn().class(prop("class")).children(vec![
                Node::Element(inner_el),
                Node::Element(inner_span_el),
            ])
        });
        let mut p = Props::new();
        p.insert("class", PropType::String("card-class".into()));
        let nodes = c.render(&p).expect("render");
        let html: String = nodes.iter().map(|n| n.render()).collect();
        assert_eq!(html, r#"<div class="card-class"><custom></custom><span></span></div>"#);
    }

    #[test]
    fn span_renders() {
        let c = component!("C", { span() });
        let nodes = c.render(&Props::new()).expect("render");
        let html: String = nodes.iter().map(|n| n.render()).collect();
        assert_eq!(html, "<span></span>");
    }

    #[test]
    fn h1_renders_with_text() {
        let c = component!("C", {
            crate::components::html::h1().children(vec![Node::Text("Title".into())])
        });
        let nodes = c.render(&Props::new()).expect("render");
        let html: String = nodes.iter().map(|n| n.render()).collect();
        assert_eq!(html, "<h1>Title</h1>");
    }
}

// =================== SVG typed wrappers ===================

#[cfg(feature = "components")]
mod svg_tests {
    use super::*;
    use crate::components::svg::{circle, rect};

    #[test]
    fn circle_renders_with_dynamic_attrs() {
        let c = component!("C", {
            circle().cx(prop("cx")).cy(prop("cy")).r(prop("r"))
        });
        let mut p = Props::new();
        p.insert("cx", PropType::String("50".into()));
        p.insert("cy", PropType::String("50".into()));
        p.insert("r", PropType::String("25".into()));
        let nodes = c.render(&p).expect("render");
        let html: String = nodes.iter().map(|n| n.render()).collect();
        assert_eq!(html, r#"<circle cx="50" cy="50" r="25"></circle>"#);
    }

    #[test]
    fn rect_renders_with_static_attrs() {
        let c = component!("C", {
            rect().width("100").height("50")
        });
        let nodes = c.render(&Props::new()).expect("render");
        let html: String = nodes.iter().map(|n| n.render()).collect();
        assert_eq!(html, r#"<rect width="100" height="50"></rect>"#);
    }
}

// =================== switch! macro ===================

#[cfg(feature = "components")]
mod switch_tests {
    use super::*;

    #[test]
    fn switch_macro_basic() {
        let e: Expr = crate::switch!("role", {
            "admin" => prop("a"),
            "user"  => prop("b"),
            _       => prop("c"),
        });
        let is_match = matches!(e, Expr::Match { ref key, ref arms, .. }
            if key == "role" && arms.len() == 2);
        assert!(is_match);
    }

    #[test]
    fn switch_macro_default_only() {
        let e: Expr = crate::switch!("role", {
            _ => prop("default"),
        });
        let is_match = matches!(e, Expr::Match { ref arms, .. } if arms.is_empty());
        assert!(is_match);
    }

    #[test]
    fn switch_macro_renders_arm() {
        use crate::components::html::div;
        let c = component!("C", {
            crate::switch!("status", {
                "ok"    => div().class("ok"),
                "error" => div().class("error"),
                _       => div().class("unknown"),
            })
        });
        let mut p = Props::new();
        p.insert("status", PropType::String("ok".into()));
        let nodes = c.render(&p).expect("render");
        let html: String = nodes.iter().map(|n| n.render()).collect();
        assert_eq!(html, r#"<div class="ok"></div>"#);
    }
}

// =================== text! macro ===================

#[cfg(feature = "components")]
mod text_tests {
    use super::*;

    #[test]
    fn text_macro_single() {
        let expr: Expr = crate::text!("hello");
        let is_list = matches!(expr, Expr::List(ref items) if items.len() == 1);
        assert!(is_list);
    }

    #[test]
    fn text_macro_multiple() {
        let expr: Expr = crate::text!(prop("first"), " ", prop("last"));
        let is_list = matches!(expr, Expr::List(ref items) if items.len() == 3);
        assert!(is_list);
    }

    #[test]
    fn text_macro_renders() {
        use crate::components::html::p;
        let c = component!("C", {
            p().children(vec![Node::Text("Hello, World!".into())])
        });
        let nodes = c.render(&Props::new()).expect("render");
        let html: String = nodes.iter().map(|n| n.render()).collect();
        assert_eq!(html, "<p>Hello, World!</p>");
    }
}

// =================== WrappedAttribute ===================

#[test]
fn wrapped_attribute_static_debug() {
    let a = attr("class").value("card");
    let wa = WrappedAttribute::Static(a);
    assert!(format!("{wa:?}").contains("Static"));
    assert!(format!("{wa:?}").contains("class"));
}

#[test]
fn wrapped_attribute_dynamic_debug() {
    let wa = WrappedAttribute::Dynamic("id".into(), Expr::Prop("x".into()));
    let dbg = format!("{wa:?}");
    assert!(dbg.contains("Dynamic"));
    assert!(dbg.contains("id"));
}

#[test]
fn wrapped_attribute_static_eq() {
    let a = WrappedAttribute::Static(attr("href").value("#"));
    let b = WrappedAttribute::Static(attr("href").value("#"));
    let c = WrappedAttribute::Static(attr("href").value("/other"));
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn wrapped_attribute_dynamic_eq() {
    let a = WrappedAttribute::Dynamic("k".into(), Expr::Prop("x".into()));
    let b = WrappedAttribute::Dynamic("k".into(), Expr::Prop("x".into()));
    let c = WrappedAttribute::Dynamic("k".into(), Expr::Prop("y".into()));
    assert_eq!(a, b);
    assert_ne!(a, c);
}

#[test]
fn wrapped_attribute_static_ne_dynamic() {
    let a = WrappedAttribute::Static(attr("k").value("v"));
    let b = WrappedAttribute::Dynamic(
        "k".into(),
        Expr::LiteralChildren(vec![Node::Text("v".into())]),
    );
    assert_ne!(a, b);
}

// =================== Render tests ===================

#[cfg(feature = "components")]
mod render_tests {
    use super::*;

    #[test]
    fn render_literal() {
        let c = Component {
            name: Cow::Borrowed("c"),
            expr: Expr::Literal(el("div")),
        };
        let nodes = c.render(&Props::new()).expect("render");
        assert_eq!(nodes.len(), 1);
    }

    #[test]
    fn render_prop_string() {
        let c = component!("c", { prop("name") });
        let mut p = Props::new();
        p.insert("name", PropType::String("Alice".into()));
        let nodes = c.render(&p).expect("render");
        assert_eq!(nodes.len(), 1);
    }

    #[test]
    fn render_either_true() {
        let c = component!("c", {
            Expr::Either {
                condition: "flag".into(),
                then: Box::new(Expr::Literal(el("yes"))),
                otherwise: Box::new(Expr::Literal(el("no"))),
            }
        });
        let mut p = Props::new();
        p.insert("flag", PropType::Bool(true));
        let nodes = c.render(&p).expect("render");
        let is_yes = matches!(&nodes[0], Node::Element(el) if el.name == "yes");
        assert!(is_yes);
    }

    #[test]
    fn render_either_false() {
        let c = component!("c", {
            Expr::Either {
                condition: "flag".into(),
                then: Box::new(Expr::Literal(el("yes"))),
                otherwise: Box::new(Expr::Literal(el("no"))),
            }
        });
        let mut p = Props::new();
        p.insert("flag", PropType::Bool(false));
        let nodes = c.render(&p).expect("render");
        let is_no = matches!(&nodes[0], Node::Element(el) if el.name == "no");
        assert!(is_no);
    }

    #[test]
    fn render_either_type_mismatch() {
        let c = component!("c", {
            Expr::Either {
                condition: "flag".into(),
                then: Box::new(prop("yes")),
                otherwise: Box::new(prop("no")),
            }
        });
        let mut p = Props::new();
        p.insert("flag", PropType::String("true".into()));
        let result = c.render(&p);
        let is_err = matches!(result, Err(RenderError::TypeMismatch { .. }));
        assert!(is_err);
    }

    #[test]
    fn render_maybe_true() {
        let c = component!("c", {
            Expr::Maybe {
                condition: "flag".into(),
                then: Box::new(Expr::Literal(el("present"))),
            }
        });
        let mut p = Props::new();
        p.insert("flag", PropType::Bool(true));
        let nodes = c.render(&p).expect("render");
        let is_present = matches!(&nodes[0], Node::Element(el) if el.name == "present");
        assert!(is_present);
    }

    #[test]
    fn render_maybe_false() {
        let c = component!("c", {
            Expr::Maybe {
                condition: "flag".into(),
                then: Box::new(prop("present")),
            }
        });
        let mut p = Props::new();
        p.insert("flag", PropType::Bool(false));
        let nodes = c.render(&p).expect("render");
        assert!(nodes.is_empty());
    }

    #[test]
    fn render_match_arm_hit() {
        let arms = vec![
            MatchArm {
                value: Cow::Borrowed("admin"),
                result: Box::new(Expr::Literal(el("a"))),
            },
            MatchArm {
                value: Cow::Borrowed("user"),
                result: Box::new(Expr::Literal(el("u"))),
            },
        ];
        let c = component!("c", {
            Expr::Match {
                key: "role".into(),
                arms,
                default: Box::new(Expr::Literal(el("default"))),
            }
        });
        let mut p = Props::new();
        p.insert("role", PropType::String("admin".into()));
        let nodes = c.render(&p).expect("render");
        let is_a = matches!(&nodes[0], Node::Element(el) if el.name == "a");
        assert!(is_a);
    }

    #[test]
    fn render_match_default() {
        let arms = vec![MatchArm {
            value: Cow::Borrowed("admin"),
            result: Box::new(Expr::Literal(el("a"))),
        }];
        let c = component!("c", {
            Expr::Match {
                key: "role".into(),
                arms,
                default: Box::new(Expr::Literal(el("default"))),
            }
        });
        let mut p = Props::new();
        p.insert("role", PropType::String("user".into()));
        let nodes = c.render(&p).expect("render");
        let is_default = matches!(&nodes[0], Node::Element(el) if el.name == "default");
        assert!(is_default);
    }

    #[test]
    fn render_match_type_mismatch() {
        let arms = vec![MatchArm {
            value: Cow::Borrowed("admin"),
            result: Box::new(Expr::Literal(el("a"))),
        }];
        let c = component!("c", {
            Expr::Match {
                key: "role".into(),
                arms,
                default: Box::new(Expr::Literal(el("default"))),
            }
        });
        let mut p = Props::new();
        p.insert("role", PropType::Bool(true));
        let result = c.render(&p);
        let is_err = matches!(result, Err(RenderError::TypeMismatch { .. }));
        assert!(is_err);
    }

    #[test]
    fn render_map_with_items() {
        let c = component!("c", {
            Expr::Map {
                input: "items".into(),
                body: Box::new(Expr::Literal(el("li"))),
            }
        });
        let mut p = Props::new();
        p.insert(
            "items",
            PropType::List(vec![
                PropType::String("a".into()),
                PropType::String("b".into()),
            ]),
        );
        let nodes = c.render(&p).expect("render");
        assert_eq!(nodes.len(), 2);
    }

    #[test]
    fn render_wrap_dynamic_attr_multi_node_fallback() {
        let c = component!("c", {
            Expr::Wrap {
                name: "div".into(),
                attrs: vec![WrappedAttribute::Dynamic(
                    "class".into(),
                    Expr::Match {
                        key: "missing".into(),
                        arms: vec![],
                        default: Box::new(Expr::Prop("x".into())),
                    },
                )],
                body: vec![],
            }
        });
        let err = c.render(&Props::new()).unwrap_err();
        assert!(matches!(err, RenderError::TypeMismatch { .. }));
    }
}

// =================== RenderError Display ===================

#[test]
fn render_error_display() {
    let err = RenderError::TypeMismatch {
        key: Cow::Borrowed("flag"),
        expected: "bool",
        found: Cow::Borrowed("string"),
    };
    let s = format!("{}", err);
    assert!(s.contains("flag"));
    assert!(s.contains("bool"));
    assert!(s.contains("string"));
}

#[test]
fn render_error_is_std_error() {
    let err = RenderError::TypeMismatch {
        key: Cow::Borrowed("k"),
        expected: "bool",
        found: Cow::Borrowed("string"),
    };
    let _: Box<dyn std::error::Error> = Box::new(err);
}

// =================== Round-trip via Mrk ===================

#[cfg(feature = "ir")]
mod ir_tests {
    use super::*;

    #[test]
    fn round_trip_simple_component() {
        let c = component!("greet", { prop("name") });
        let bytes = Mrk::bytes_component(&c);
        let back = Mrk::from_bytes_component(&bytes).expect("decode");
        assert_eq!(c, back);
    }

    #[test]
    fn round_trip_with_dynamic_attr() {
        use crate::components::html::div;
        let c = component!("c", { div().class(prop("class")) });
        let bytes = Mrk::bytes_component(&c);
        let back = Mrk::from_bytes_component(&bytes).expect("decode");
        assert_eq!(c, back);
    }

    #[test]
    fn round_trip_with_switch() {
        use crate::components::html::div;
        let c = component!("c", {
            crate::switch!("status", {
                "ok" => div().class("ok"),
                _     => div().class("other"),
            })
        });
        let bytes = Mrk::bytes_component(&c);
        let back = Mrk::from_bytes_component(&bytes).expect("decode");
        assert_eq!(c, back);
    }

    #[test]
    fn element_bytes_still_works() {
        let e = el("div");
        let bytes = Mrk::bytes(&e);
        let back = Mrk::from_bytes(&bytes).expect("decode");
        assert_eq!(e, back);
    }
}

// =================== HTML/SVG factories inside compose! (cross-feature test) ===================

#[cfg(feature = "html")]
#[test]
fn html_module_factories_still_work() {
    use crate::html::script;
    let out = script().src("app.js").render();
    assert_eq!(out, r#"<script src="app.js"></script>"#);
}

#[cfg(feature = "svg")]
#[test]
fn svg_module_factories_still_work() {
    use crate::svg::circle;
    let out = circle().cx("50").cy("50").r("25").render();
    assert_eq!(out, r#"<circle cx="50" cy="50" r="25"></circle>"#);
}

// =================== nodes! with typed wrappers ===================

#[test]
fn nodes_macro_accepts_typed_wrappers() {
    use crate::components::html::{div, span};
    let c = component!("Card", {
        div().class(prop("class")).children(nodes![
            span().children(nodes![text!(prop("title"))]),
        ])
    });
    let mut props = Props::new();
    props.insert("class", PropType::String("card".into()));
    props.insert("title", PropType::String("Hello".into()));
    let result = c.render(&props).expect("render");
    assert_eq!(result.len(), 1);
    let rendered = result[0].render();
    assert!(rendered.contains("card"));
    assert!(rendered.contains("Hello"));
}

#[test]
fn nodes_macro_mixes_wrappers_and_strings() {
    use crate::components::html::span;
    let c = component!("Mixed", {
        span().children(nodes!["static text", span()])
    });
    let props = Props::new();
    let result = c.render(&props).expect("render");
    assert_eq!(result.len(), 1);
    let rendered = result[0].render();
    assert!(rendered.contains("static text"));
}

#[test]
fn nodes_macro_with_expr_items() {
    use crate::components::html::div;
    let c = component!("WithExpr", {
        div().children(nodes![prop("x")])
    });
    let mut props = Props::new();
    props.insert("x", PropType::String("dynamic".into()));
    let result = c.render(&props).expect("render");
    assert_eq!(result.len(), 1);
    let rendered = result[0].render();
    assert!(rendered.contains("dynamic"));
}

// =================== Coverage: Props::defaults() ===================

#[test]
fn props_defaults_overlay() {
    let mut fallbacks = Props::new();
    fallbacks.insert("color", PropType::String("blue".into()));
    fallbacks.insert("size", PropType::Number(Number::int("12")));
    let mut caller = Props::new();
    caller.insert("color", PropType::String("red".into()));
    let merged = caller.defaults(&fallbacks);
    assert_eq!(merged.get("color"), Some(&PropType::String("red".into())));
    assert_eq!(merged.get("size"), Some(&PropType::Number(Number::int("12"))));
}

#[test]
fn props_defaults_empty() {
    let fallbacks = Props::new();
    let caller = Props::new();
    let merged = caller.defaults(&fallbacks);
    assert!(merged.is_empty());
}

// =================== Coverage: Props::from(HashMap) ===================

#[test]
fn props_from_hashmap_direct() {
    let mut m = HashMap::new();
    m.insert(Cow::Borrowed("x"), PropType::Bool(true));
    m.insert(Cow::Borrowed("y"), PropType::Number(Number::int("7")));
    let p = Props::from(m);
    assert_eq!(p.len(), 2);
    assert_eq!(p.get("x"), Some(&PropType::Bool(true)));
    assert_eq!(p.get("y"), Some(&PropType::Number(Number::int("7"))));
}

// =================== Coverage: Number parse methods ===================

#[test]
fn number_parse_i64_method() {
    let n = Number::int("42");
    assert_eq!(n.parse_i64(), Some(42));
    let neg = Number::int("-7");
    assert_eq!(neg.parse_i64(), Some(-7));
    let bad = Number::int("abc");
    assert_eq!(bad.parse_i64(), None);
    let float = Number::float("3.14");
    assert_eq!(float.parse_i64(), None);
}

#[test]
fn number_parse_f64_method() {
    let n = Number::float("3.14");
    assert!((n.parse_f64().unwrap() - 3.14).abs() < 0.0001);
    let neg = Number::float("-1.5");
    assert!((neg.parse_f64().unwrap() - (-1.5)).abs() < 0.0001);
    let bad = Number::float("abc");
    assert_eq!(bad.parse_f64(), None);
    let int = Number::int("42");
    assert!((int.parse_f64().unwrap() - 42.0).abs() < 0.0001);
}

// =================== Coverage: NumberKind::tag / from_tag ===================

#[test]
fn numberkind_tag_values() {
    assert_eq!(NumberKind::Int.tag(), "i");
    assert_eq!(NumberKind::Float.tag(), "f");
}

#[test]
fn numberkind_from_tag_roundtrip() {
    assert_eq!(NumberKind::from_tag("i"), Some(NumberKind::Int));
    assert_eq!(NumberKind::from_tag("f"), Some(NumberKind::Float));
    assert_eq!(NumberKind::from_tag("x"), None);
    assert_eq!(NumberKind::from_tag(""), None);
    assert_eq!(NumberKind::from_tag("I"), None);
}

// =================== Coverage: ComponentAttribute::Static into_expr ===================

#[test]
fn component_element_static_attr_into_expr() {
    let e = ComponentElement::new("div").attr("disabled");
    let expr: Expr = e.into_expr();
    let is_wrap_static = matches!(
        expr,
        Expr::Wrap { ref name, ref attrs, .. }
        if name == "div"
            && attrs.len() == 1
            && matches!(&attrs[0], WrappedAttribute::Static(_))
    );
    assert!(is_wrap_static, "expected Wrap with Static attr, got: {expr:?}");
}

#[test]
fn component_element_mixed_attrs_into_expr() {
    let e = ComponentElement::new("div")
        .attr("disabled")
        .attr_dynamic("class", prop("cls"));
    let expr: Expr = e.into_expr();
    if let Expr::Wrap { ref attrs, .. } = expr {
        assert_eq!(attrs.len(), 2);
        assert!(matches!(&attrs[0], WrappedAttribute::Static(_)));
        assert!(matches!(&attrs[1], WrappedAttribute::Dynamic(_, _)));
    } else {
        panic!("expected Wrap, got: {expr:?}");
    }
}

// =================== Coverage: render_expr uncovered branches ===================

#[cfg(feature = "components")]
mod coverage_render_branches {
    use super::*;

    #[test]
    fn render_wrap_static_kv_attr() {
        let c = Component {
            name: Cow::Borrowed("c"),
            expr: Expr::Wrap {
                name: "div".into(),
                attrs: vec![WrappedAttribute::Static(attr("class").value("card"))],
                body: vec![],
            },
        };
        let nodes = c.render(&Props::new()).expect("render");
        assert_eq!(nodes.len(), 1);
        let html = nodes[0].render();
        assert!(html.contains("class=\"card\""));
    }

    #[test]
    fn render_wrap_dynamic_attr_multi_node_fallback_path() {
        let c = Component {
            name: Cow::Borrowed("c"),
            expr: Expr::Wrap {
                name: "div".into(),
                attrs: vec![WrappedAttribute::Dynamic(
                    "class".into(),
                    Expr::List(vec![
                        Box::new(Expr::Prop("a".into())),
                        Box::new(Expr::Prop("b".into())),
                    ]),
                )],
                body: vec![],
            },
        };
        let mut p = Props::new();
        p.insert("a", PropType::String("x".into()));
        p.insert("b", PropType::String("y".into()));
        let nodes = c.render(&p).expect("render");
        assert_eq!(nodes.len(), 1);
        let html = nodes[0].render();
        assert!(html.contains("xy"));
    }

    #[test]
    fn render_prop_missing_returns_empty_string() {
        let c = Component {
            name: Cow::Borrowed("c"),
            expr: Expr::Prop("nonexistent".into()),
        };
        let nodes = c.render(&Props::new()).expect("render");
        assert_eq!(nodes.len(), 1);
        let text = nodes[0].render();
        assert_eq!(text, "");
    }

    #[test]
    fn render_either_missing_condition_key() {
        let c = Component {
            name: Cow::Borrowed("c"),
            expr: Expr::Either {
                condition: "flag".into(),
                then: Box::new(Expr::Literal(el("yes"))),
                otherwise: Box::new(Expr::Literal(el("no"))),
            },
        };
        let result = c.render(&Props::new());
        let err = result.unwrap_err();
        assert!(matches!(err, RenderError::TypeMismatch { ref key, .. } if key == "flag"));
    }

    #[test]
    fn render_maybe_missing_condition_key() {
        let c = Component {
            name: Cow::Borrowed("c"),
            expr: Expr::Maybe {
                condition: "flag".into(),
                then: Box::new(Expr::Literal(el("yes"))),
            },
        };
        let result = c.render(&Props::new());
        let err = result.unwrap_err();
        assert!(matches!(err, RenderError::TypeMismatch { ref key, .. } if key == "flag"));
    }

    #[test]
    fn render_map_wrong_prop_type() {
        let c = Component {
            name: Cow::Borrowed("c"),
            expr: Expr::Map {
                input: "items".into(),
                body: Box::new(Expr::Literal(el("li"))),
            },
        };
        let mut p = Props::new();
        p.insert("items", PropType::String("not a list".into()));
        let result = c.render(&p);
        let err = result.unwrap_err();
        assert!(matches!(err, RenderError::TypeMismatch { ref key, expected: "list", .. } if key == "items"));
    }

    #[test]
    fn render_map_missing_prop() {
        let c = Component {
            name: Cow::Borrowed("c"),
            expr: Expr::Map {
                input: "items".into(),
                body: Box::new(Expr::Literal(el("li"))),
            },
        };
        let result = c.render(&Props::new());
        let err = result.unwrap_err();
        assert!(matches!(err, RenderError::TypeMismatch { ref key, expected: "list", ref found, .. }
            if key == "items" && found == "<missing>"));
    }

    #[test]
    fn render_literal_children_with_expr_child() {
        let c = Component {
            name: Cow::Borrowed("c"),
            expr: Expr::LiteralChildren(vec![
                Node::Text("before".into()),
                Node::Expr(Expr::Prop("x".into())),
                Node::Text("after".into()),
            ]),
        };
        let mut p = Props::new();
        p.insert("x", PropType::String("middle".into()));
        let nodes = c.render(&p).expect("render");
        assert_eq!(nodes.len(), 3);
        let r0 = nodes[0].render();
        let r1 = nodes[1].render();
        let r2 = nodes[2].render();
        assert_eq!(r0, "before");
        assert_eq!(r1, "middle");
        assert_eq!(r2, "after");
    }

    #[test]
    #[should_panic(expected = "Node::Expr must be resolved during Component::render, not during HTML rendering")]
    fn node_expr_render_unreachable() {
        let n = Node::Expr(Expr::Prop("x".into()));
        let _ = n.render();
    }

    /// `Expr::List` body containing an expression that errors during
    /// render. Exercises the `?` propagation path in
    /// `render_expr` for `List`.
    #[test]
    fn render_list_body_error_propagates() {
        let c = Component {
            name: Cow::Borrowed("c"),
            expr: Expr::List(vec![
                Box::new(Expr::Prop("ok".into())),
                Box::new(Expr::Match {
                    key: "missing".into(),
                    arms: vec![],
                    default: Box::new(Expr::Literal(el("never"))),
                }),
            ]),
        };
        let mut p = Props::new();
        p.insert("ok", PropType::String("hi".into()));
        let err = c.render(&p).unwrap_err();
        assert!(matches!(err, RenderError::TypeMismatch { ref key, .. } if key == "missing"));
    }

    /// `Expr::Map` body containing an expression that errors during
    /// render. Exercises the `?` propagation path in
    /// `render_expr` for `Map`.
    #[test]
    fn render_map_body_error_propagates() {
        let c = Component {
            name: Cow::Borrowed("c"),
            expr: Expr::Map {
                input: "items".into(),
                body: Box::new(Expr::Match {
                    key: "missing".into(),
                    arms: vec![],
                    default: Box::new(Expr::Literal(el("never"))),
                }),
            },
        };
        let mut p = Props::new();
        p.insert(
            "items",
            PropType::List(vec![PropType::String("a".into()), PropType::String("b".into())]),
        );
        let err = c.render(&p).unwrap_err();
        assert!(matches!(err, RenderError::TypeMismatch { ref key, .. } if key == "missing"));
    }

    /// `Expr::Wrap` body containing an expression that errors during
    /// render. Exercises the `?` propagation path in
    /// `render_expr` for `Wrap`.
    #[test]
    fn render_wrap_body_error_propagates() {
        let c = Component {
            name: Cow::Borrowed("c"),
            expr: Expr::Wrap {
                name: "div".into(),
                attrs: vec![],
                body: vec![Box::new(Expr::Either {
                    condition: "missing".into(),
                    then: Box::new(Expr::Literal(el("never"))),
                    otherwise: Box::new(Expr::Literal(el("also_never"))),
                })],
            },
        };
        let err = c.render(&Props::new()).unwrap_err();
        assert!(matches!(err, RenderError::TypeMismatch { ref key, .. } if key == "missing"));
    }

    /// `Expr::LiteralChildren` containing a `Node::Expr` that errors
    /// during render. Exercises the `?` propagation path in
    /// `render_expr` for `LiteralChildren`.
    #[test]
    fn render_literal_children_expr_child_error_propagates() {
        let c = Component {
            name: Cow::Borrowed("c"),
            expr: Expr::LiteralChildren(vec![
                Node::Text("before".into()),
                Node::Expr(Expr::Maybe {
                    condition: "missing".into(),
                    then: Box::new(Expr::Literal(el("never"))),
                }),
                Node::Text("after".into()),
            ]),
        };
        let err = c.render(&Props::new()).unwrap_err();
        assert!(matches!(err, RenderError::TypeMismatch { ref key, .. } if key == "missing"));
    }
}

// =================== Coverage: Node::Expr Display unreachable ===================

#[cfg(feature = "ir")]
#[test]
#[should_panic(expected = "Node::Expr must be resolved during Component::render, not during Display")]
fn node_expr_display_unreachable() {
    let n = Node::Expr(Expr::Prop("x".into()));
    let _ = format!("{}", n);
}

// =================== Coverage: round-trip for missing Expr variants ===================

#[cfg(feature = "ir")]
mod ir_coverage_tests {
    use super::*;
    use crate::ParseError;

    fn assert_component_round_trip(c: &Component) {
        let bytes = Mrk::bytes_component(c);
        let back = Mrk::from_bytes_component(&bytes).expect("decode");
        assert_eq!(c, &back);
    }

    #[test]
    fn round_trip_literal_component() {
        let c = Component {
            name: Cow::Borrowed("lit"),
            expr: Expr::Literal(
                el("div").attrs(vec![attr("class").value("foo")])
            ),
        };
        assert_component_round_trip(&c);
    }

    #[test]
    fn round_trip_list_component() {
        let c = Component {
            name: Cow::Borrowed("lst"),
            expr: Expr::List(vec![
                Box::new(Expr::Prop("a".into())),
                Box::new(Expr::Prop("b".into())),
                Box::new(Expr::Literal(el("span"))),
            ]),
        };
        assert_component_round_trip(&c);
    }

    #[test]
    fn round_trip_either_component() {
        let c = Component {
            name: Cow::Borrowed("either"),
            expr: Expr::Either {
                condition: "flag".into(),
                then: Box::new(Expr::Literal(el("yes"))),
                otherwise: Box::new(Expr::Literal(el("no"))),
            },
        };
        assert_component_round_trip(&c);
    }

    #[test]
    fn round_trip_maybe_component() {
        let c = Component {
            name: Cow::Borrowed("maybe"),
            expr: Expr::Maybe {
                condition: "flag".into(),
                then: Box::new(Expr::Literal(el("present"))),
            },
        };
        assert_component_round_trip(&c);
    }

    #[test]
    fn round_trip_map_component() {
        let c = Component {
            name: Cow::Borrowed("mapper"),
            expr: Expr::Map {
                input: "items".into(),
                body: Box::new(Expr::Prop("item".into())),
            },
        };
        assert_component_round_trip(&c);
    }

    #[test]
    fn round_trip_wrap_static_kv_attr() {
        let c = Component {
            name: Cow::Borrowed("ws"),
            expr: Expr::Wrap {
                name: "div".into(),
                attrs: vec![WrappedAttribute::Static(
                    attr("class").value("card"),
                )],
                body: vec![Box::new(Expr::Prop("x".into()))],
            },
        };
        assert_component_round_trip(&c);
    }

    #[test]
    fn from_bytes_rejects_component_ir() {
        let c = Component {
            name: Cow::Borrowed("test"),
            expr: Expr::Prop("x".into()),
        };
        let ir = Mrk::bytes_component(&c);
        let err = Mrk::from_bytes(&ir).unwrap_err();
        assert!(
            matches!(err, ParseError::BadLengthPrefix { .. }),
            "expected BadLengthPrefix for Component IR passed to from_bytes, got: {err:?}"
        );
    }

    #[test]
    fn from_bytes_component_rejects_element_ir() {
        let ir = Mrk::bytes(&el("div"));
        let err = Mrk::from_bytes_component(&ir).unwrap_err();
        assert!(
            matches!(err, ParseError::BadLengthPrefix { .. }),
            "expected BadLengthPrefix for Element IR passed to from_bytes_component, got: {err:?}"
        );
    }

    #[test]
    #[should_panic(expected = "Node::Expr must be resolved during Component::render, not during IR encoding")]
    fn encode_node_in_literal_children_unreachable() {
        let c = Component {
            name: Cow::Borrowed("bad"),
            expr: Expr::LiteralChildren(vec![Node::Expr(Expr::Prop("x".into()))]),
        };
        let _ = Mrk::bytes_component(&c);
    }

    #[test]
    #[should_panic(expected = "Node::Expr must be resolved during Component::render, not during IR encoding")]
    fn encode_node_expr_unreachable() {
        let e = el("div").children(vec![Node::Expr(Expr::Prop("x".into()))]);
        let _ = Mrk::bytes(&e);
    }
}