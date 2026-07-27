// Tests for the `components` module. Lives in a separate file inside the
// `components/` directory so the file stays focused on tests and the
// module's other implementation files stay focused on production code.
//
// The `matches!` false-branch regions in assertions below are uncatchable
// in passing tests; the per-file `#[cfg(coverage)]` instrumentation
// attached by `cargo llvm-cov` still measures them. That is acceptable
// for this project because the code being exercised is test-only.

use std::borrow::Cow;
use std::collections::HashMap;

use crate::attributes::attr;
use crate::components::{
    arm, component, either, list_expr, literal, map, match_on, maybe, prop, wrap, Component, Expr,
    ExprCtx, IntoExpr, MatchEntry, Number, NumberKind, Otherwise, PropType, Props, RenderError,
    WrappedAttribute,
};
use crate::components::list;
use crate::element::{el, Element};
use crate::node::Node;
use crate::renderable::Renderable;

// Round-trip tests (`Mrk::bytes_component`, `Mrk::from_bytes_component`,
// etc.) require the `ir` feature. Import `Mrk` only when those tests
// are eligible to compile.
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
fn props_from_hashmap() {
    let mut map = HashMap::new();
    map.insert(Cow::Borrowed("k"), PropType::String("v".into()));
    let p: Props = map.into();
    assert_eq!(p.len(), 1);
    let is_string = matches!(p.get("k"), Some(PropType::String(_)));
    assert!(is_string);
}

#[test]
fn props_from_iter() {
    let p: Props = vec![(Cow::Borrowed("a"), PropType::Bool(true))]
        .into_iter()
        .collect();
    assert_eq!(p.len(), 1);
    let is_bool = matches!(p.get("a"), Some(PropType::Bool(true)));
    assert!(is_bool);
}

// =================== PropType / Number ===================

#[test]
fn number_int_and_float() {
    let n_int = Number::int("42");
    let n_float = Number::float("3.14");
    assert_eq!(n_int.repr, "42");
    assert_eq!(n_int.kind, NumberKind::Int);
    assert_eq!(n_float.repr, "3.14");
    assert_eq!(n_float.kind, NumberKind::Float);
}

#[test]
fn number_parse_i64() {
    let n = Number::int("42");
    let parsed = n.parse_i64();
    assert_eq!(parsed, Some(42));
    let bad = Number::int("not-a-number");
    let none = bad.parse_i64();
    assert!(none.is_none());
}

#[test]
fn number_parse_f64() {
    let n = Number::float("3.14");
    let parsed = n.parse_f64();
    // Use `#[allow(clippy::approx_constant)]` on the literal `3.14`:
    // it's intentional (we want to round-trip a stringified f64), not
    // an accidental PI.
    #[allow(clippy::approx_constant)]
    let expected = 3.14_f64;
    assert_eq!(parsed, Some(expected));
    let bad = Number::float("nope");
    let none = bad.parse_f64();
    assert!(none.is_none());
}

#[test]
fn number_kind_tag_round_trip() {
    let cases = [(NumberKind::Int, "i"), (NumberKind::Float, "f")];
    for (kind, tag) in cases {
        assert_eq!(kind.tag(), tag);
        assert_eq!(NumberKind::from_tag(tag), Some(kind));
    }
    let none = NumberKind::from_tag("x");
    assert!(none.is_none());
}

#[test]
fn prop_type_to_text_string() {
    let p = PropType::String("hello".into());
    assert_eq!(p.to_text(), "hello");
}

#[test]
fn prop_type_to_text_number() {
    let p = PropType::Number(Number::int("42"));
    assert_eq!(p.to_text(), "42");
}

#[test]
fn prop_type_to_text_bool() {
    let p = PropType::Bool(true);
    let txt = p.to_text();
    assert_eq!(txt, "true");
    let p2 = PropType::Bool(false);
    let txt2 = p2.to_text();
    assert_eq!(txt2, "false");
}

#[test]
fn prop_type_to_text_list() {
    let p = PropType::List(vec![
        PropType::String("a".into()),
        PropType::String("b".into()),
    ]);
    assert_eq!(p.to_text(), "a,b");
}

#[test]
fn prop_type_to_text_dictionary() {
    let mut m = HashMap::new();
    m.insert(Cow::Borrowed("a"), PropType::String("x".into()));
    let p = PropType::Dictionary(m);
    let txt = p.to_text();
    assert!(txt.contains("1 keys"));
}

#[test]
fn prop_type_type_name() {
    let names = [
        (PropType::String("".into()), "string"),
        (PropType::Number(Number::int("0")), "number"),
        (PropType::Bool(true), "bool"),
        (PropType::List(vec![]), "list"),
    ];
    let mut d = HashMap::new();
    d.insert(Cow::Borrowed("k"), PropType::Bool(false));
    let d_name = PropType::Dictionary(d).type_name();
    assert_eq!(d_name, "dictionary");
    for (p, expected) in names {
        assert_eq!(p.type_name(), expected);
    }
}

// =================== IntoExpr ===================

#[test]
fn into_expr_element() {
    let e: Expr = Element::new("div").into_expr();
    let is_literal = matches!(e, Expr::Literal(_));
    assert!(is_literal);
}

#[test]
fn into_expr_expr() {
    let original = prop("k");
    let copy: Expr = original.clone().into_expr();
    assert_eq!(original, copy);
}

#[test]
fn into_expr_box() {
    let original = Box::new(prop("k"));
    let unboxed: Expr = original.into_expr();
    let is_prop = matches!(unboxed, Expr::Prop(_));
    assert!(is_prop);
}

#[test]
fn into_expr_node() {
    let n: Expr = Node::Text("hi".into()).into_expr();
    let is_lc = matches!(n, Expr::LiteralChildren(_));
    assert!(is_lc);
}

// =================== list! macro ===================

#[test]
fn list_macro_mixed() {
    let _e: Expr = list![
        Element::new("h1").children(vec!["Title".into()]),
        prop("name"),
        Box::new(prop("x")),
        Node::Text("static".into()),
    ];
}

#[test]
fn list_macro_empty() {
    let e: Expr = list![];
    let is_list = matches!(e, Expr::List(ref items) if items.is_empty());
    assert!(is_list);
}

// =================== Constructor helpers ===================

#[test]
fn literal_helper() {
    let e: Expr = literal(Element::new("div"));
    let is_literal = matches!(e, Expr::Literal(_));
    assert!(is_literal);
}

#[test]
fn prop_helper() {
    let e: Expr = prop("k");
    let is_prop = matches!(e, Expr::Prop(ref k) if k == "k");
    assert!(is_prop);
}

#[test]
fn list_expr_helper() {
    let e: Expr = list_expr(vec![Box::new(prop("a")), Box::new(prop("b"))]);
    let is_list = matches!(e, Expr::List(ref items) if items.len() == 2);
    assert!(is_list);
}

#[test]
fn either_helper() {
    let e: Expr = either("c", prop("a"), prop("b"));
    let is_either = matches!(e, Expr::Either { .. });
    assert!(is_either);
}

#[test]
fn maybe_helper() {
    let e: Expr = maybe("c", prop("a"));
    let is_maybe = matches!(e, Expr::Maybe { .. });
    assert!(is_maybe);
}

#[test]
fn map_helper() {
    let e: Expr = map("items", prop("a"));
    let is_map = matches!(e, Expr::Map { .. });
    assert!(is_map);
}

#[test]
fn match_on_helper() {
    let arms = vec![arm("a", prop("x"))];
    let e: Expr = match_on("role", arms, prop("default"));
    let is_match = matches!(e, Expr::Match { .. });
    assert!(is_match);
}

#[test]
fn arm_helper() {
    let a = arm("v", prop("r"));
    let v = &a.value;
    assert_eq!(v, "v");
    let is_prop = matches!(*a.result, Expr::Prop(_));
    assert!(is_prop);
}

#[test]
fn wrap_helper_with_no_children() {
    let e: Expr = wrap(Element::new("div"), prop("a"));
    match e {
        Expr::Wrap { name, attrs, body } => {
            assert_eq!(name, "div");
            assert!(attrs.is_empty());
            assert_eq!(body.len(), 1);
        }
        _ => panic!("expected Wrap"),
    }
}

#[test]
fn wrap_helper_with_initial_children() {
    let el = Element::new("div").children(vec![Node::Text("static".into())]);
    let e: Expr = wrap(el, prop("dyn"));
    match e {
        Expr::Wrap { name, body, .. } => {
            assert_eq!(name, "div");
            // body: [LiteralChildren([Text("static")]), Prop("dyn")]
            assert_eq!(body.len(), 2);
        }
        _ => panic!("expected Wrap"),
    }
}

#[test]
fn wrap_helper_with_attrs() {
    let el = Element::new("a").attrs(vec![attr("href").value("/")]);
    let e: Expr = wrap(el, prop("text"));
    match e {
        Expr::Wrap { name, attrs, body } => {
            assert_eq!(name, "a");
            assert_eq!(attrs.len(), 1);
            assert_eq!(body.len(), 1);
        }
        _ => panic!("expected Wrap"),
    }
}

#[test]
fn component_helper() {
    let c = component("greet", prop("name"));
    assert_eq!(c.name, "greet");
    let is_prop = matches!(c.expr, Expr::Prop(_));
    assert!(is_prop);
}

// =================== Render: Literal ===================

#[test]
fn render_literal() {
    let c = component("c", literal(el("div")));
    let props = Props::new();
    let result = c.render(&props);
    let is_ok = result.is_ok();
    assert!(is_ok);
    let nodes = result.expect("render should succeed");
    assert_eq!(nodes.len(), 1);
    let is_element = matches!(nodes[0], Node::Element(_));
    assert!(is_element);
}

// =================== Render: Prop ===================

#[test]
fn render_prop_string() {
    let c = component("c", prop("name"));
    let mut p = Props::new();
    p.insert("name", PropType::String("Alice".into()));
    let nodes = c.render(&p).expect("render");
    assert_eq!(nodes.len(), 1);
    let is_text_alice = matches!(&nodes[0], Node::Text(t) if t == "Alice");
    assert!(is_text_alice);
}

#[test]
fn render_prop_missing_is_empty() {
    let c = component("c", prop("name"));
    let p = Props::new();
    let nodes = c.render(&p).expect("render");
    assert_eq!(nodes.len(), 1);
    let is_text_empty = matches!(&nodes[0], Node::Text(t) if t.is_empty());
    assert!(is_text_empty);
}

#[test]
fn render_prop_number() {
    let c = component("c", prop("age"));
    let mut p = Props::new();
    p.insert("age", PropType::Number(Number::int("30")));
    let nodes = c.render(&p).expect("render");
    let is_text_30 = matches!(&nodes[0], Node::Text(t) if t == "30");
    assert!(is_text_30);
}

#[test]
fn render_prop_bool() {
    let c = component("c", prop("flag"));
    let mut p = Props::new();
    p.insert("flag", PropType::Bool(true));
    let nodes = c.render(&p).expect("render");
    let is_text_true = matches!(&nodes[0], Node::Text(t) if t == "true");
    assert!(is_text_true);
}

// =================== Render: List ===================

#[test]
fn render_list() {
    let c = component("c", list_expr(vec![Box::new(prop("a")), Box::new(prop("b"))]));
    let mut p = Props::new();
    p.insert("a", PropType::String("X".into()));
    p.insert("b", PropType::String("Y".into()));
    let nodes = c.render(&p).expect("render");
    assert_eq!(nodes.len(), 2);
    let is_x = matches!(&nodes[0], Node::Text(t) if t == "X");
    let is_y = matches!(&nodes[1], Node::Text(t) if t == "Y");
    assert!(is_x);
    assert!(is_y);
}

// =================== Render: Either ===================

#[test]
fn render_either_true() {
    let c = component(
        "c",
        either("flag", literal(el("yes")), literal(el("no"))),
    );
    let mut p = Props::new();
    p.insert("flag", PropType::Bool(true));
    let nodes = c.render(&p).expect("render");
    let is_yes = matches!(&nodes[0], Node::Element(el) if el.name == "yes");
    assert!(is_yes);
}

#[test]
fn render_either_false() {
    let c = component(
        "c",
        either("flag", literal(el("yes")), literal(el("no"))),
    );
    let mut p = Props::new();
    p.insert("flag", PropType::Bool(false));
    let nodes = c.render(&p).expect("render");
    let is_no = matches!(&nodes[0], Node::Element(el) if el.name == "no");
    assert!(is_no);
}

#[test]
fn render_either_type_mismatch() {
    let c = component("c", either("flag", prop("yes"), prop("no")));
    let mut p = Props::new();
    p.insert("flag", PropType::String("true".into()));
    let result = c.render(&p);
    let is_err = matches!(result, Err(RenderError::TypeMismatch { .. }));
    assert!(is_err);
}

#[test]
fn render_either_missing() {
    let c = component("c", either("flag", prop("yes"), prop("no")));
    let p = Props::new();
    let result = c.render(&p);
    let is_err = matches!(result, Err(RenderError::TypeMismatch { .. }));
    assert!(is_err);
}

// =================== Render: Maybe ===================

#[test]
fn render_maybe_true() {
    let c = component("c", maybe("flag", literal(el("present"))));
    let mut p = Props::new();
    p.insert("flag", PropType::Bool(true));
    let nodes = c.render(&p).expect("render");
    let is_present = matches!(&nodes[0], Node::Element(el) if el.name == "present");
    assert!(is_present);
}

#[test]
fn render_maybe_false() {
    let c = component("c", maybe("flag", prop("present")));
    let mut p = Props::new();
    p.insert("flag", PropType::Bool(false));
    let nodes = c.render(&p).expect("render");
    assert!(nodes.is_empty());
}

#[test]
fn render_maybe_type_mismatch() {
    let c = component("c", maybe("flag", prop("present")));
    let mut p = Props::new();
    p.insert("flag", PropType::Number(Number::int("1")));
    let result = c.render(&p);
    let is_err = matches!(result, Err(RenderError::TypeMismatch { .. }));
    assert!(is_err);
}

// =================== Render: Match ===================

#[test]
fn render_match_arm_hit() {
    let arms = vec![arm("admin", literal(el("a"))), arm("user", literal(el("u")))];
    let c = component("c", match_on("role", arms, literal(el("default"))));
    let mut p = Props::new();
    p.insert("role", PropType::String("admin".into()));
    let nodes = c.render(&p).expect("render");
    let is_a = matches!(&nodes[0], Node::Element(el) if el.name == "a");
    assert!(is_a);
}

#[test]
fn render_match_default() {
    let arms = vec![arm("admin", literal(el("a")))];
    let c = component("c", match_on("role", arms, literal(el("default"))));
    let mut p = Props::new();
    p.insert("role", PropType::String("user".into()));
    let nodes = c.render(&p).expect("render");
    let is_default = matches!(&nodes[0], Node::Element(el) if el.name == "default");
    assert!(is_default);
}

#[test]
fn render_match_type_mismatch() {
    let arms = vec![arm("admin", literal(el("a")))];
    let c = component("c", match_on("role", arms, literal(el("default"))));
    let mut p = Props::new();
    p.insert("role", PropType::Bool(true));
    let result = c.render(&p);
    let is_err = matches!(result, Err(RenderError::TypeMismatch { .. }));
    assert!(is_err);
}

#[test]
fn render_match_missing() {
    let arms = vec![arm("admin", literal(el("a")))];
    let c = component("c", match_on("role", arms, literal(el("default"))));
    let p = Props::new();
    let result = c.render(&p);
    let is_err = matches!(result, Err(RenderError::TypeMismatch { .. }));
    assert!(is_err);
}

// =================== Render: Map ===================

#[test]
fn render_map_empty() {
    let c = component("c", map("items", literal(el("li"))));
    let mut p = Props::new();
    p.insert("items", PropType::List(vec![]));
    let nodes = c.render(&p).expect("render");
    assert!(nodes.is_empty());
}

#[test]
fn render_map_with_items() {
    let c = component(
        "c",
        map("items", literal(el("li"))),
    );
    let mut p = Props::new();
    p.insert(
        "items",
        PropType::List(vec![
            PropType::String("a".into()),
            PropType::String("b".into()),
            PropType::String("c".into()),
        ]),
    );
    let nodes = c.render(&p).expect("render");
    // One element per item.
    assert_eq!(nodes.len(), 3);
}

#[test]
fn render_map_with_index() {
    let c = component(
        "c",
        map("items", prop("index")),
    );
    let mut p = Props::new();
    p.insert(
        "items",
        PropType::List(vec![
            PropType::String("a".into()),
            PropType::String("b".into()),
        ]),
    );
    let nodes = c.render(&p).expect("render");
    // Each iteration produces a Text node with the index.
    let first_index = match &nodes[0] {
        Node::Text(t) => t.to_string(),
        _ => panic!("expected Text"),
    };
    let second_index = match &nodes[1] {
        Node::Text(t) => t.to_string(),
        _ => panic!("expected Text"),
    };
    assert_eq!(first_index, "0");
    assert_eq!(second_index, "1");
}

#[test]
fn render_map_type_mismatch() {
    let c = component("c", map("items", prop("x")));
    let mut p = Props::new();
    p.insert("items", PropType::String("not-a-list".into()));
    let result = c.render(&p);
    let is_err = matches!(result, Err(RenderError::TypeMismatch { .. }));
    assert!(is_err);
}

#[test]
fn render_map_missing_key() {
    let c = component("c", map("items", prop("x")));
    let p = Props::new();
    let result = c.render(&p);
    let is_err = matches!(result, Err(RenderError::TypeMismatch { .. }));
    assert!(is_err);
}

#[test]
fn into_expr_for_node_ref() {
    let n = Node::Text("x".into());
    let n_ref: &Node = &n;
    let e: Expr = n_ref.into_expr();
    let is_lc = matches!(e, Expr::LiteralChildren(_));
    assert!(is_lc);
}

// =================== Render: Wrap ===================

#[test]
fn render_wrap_basic() {
    let c = component("c", wrap(Element::new("div"), prop("a")));
    let mut p = Props::new();
    p.insert("a", PropType::String("X".into()));
    let nodes = c.render(&p).expect("render");
    assert_eq!(nodes.len(), 1);
    match &nodes[0] {
        Node::Element(el) => {
            assert_eq!(el.name, "div");
            assert_eq!(el.children.len(), 1);
        }
        _ => panic!("expected Element"),
    }
}

#[test]
fn render_wrap_with_initial_children() {
    let el = Element::new("div").children(vec![Node::Text("static".into())]);
    let c = component("c", wrap(el, prop("dyn")));
    let mut p = Props::new();
    p.insert("dyn", PropType::String("D".into()));
    let nodes = c.render(&p).expect("render");
    assert_eq!(nodes.len(), 1);
    match &nodes[0] {
        Node::Element(el) => {
            // "static" + "D"
            assert_eq!(el.children.len(), 2);
        }
        _ => panic!("expected Element"),
    }
}

// =================== Render: LiteralChildren ===================

#[test]
fn render_literal_children() {
    let e: Expr = Expr::LiteralChildren(vec![
        Node::Text("a".into()),
        Node::Text("b".into()),
    ]);
    let c = component("c", e);
    let p = Props::new();
    let nodes = c.render(&p).expect("render");
    assert_eq!(nodes.len(), 2);
}

// =================== Round-trip via Mrk ===================

/// Round-trip + dispatch tests live under the `ir` feature because they
/// need [`crate::ir::Mrk`].
#[cfg(feature = "ir")]
mod ir_tests {
    use super::*;

    #[test]
    fn round_trip_simple_component() {
        let c = component("greet", wrap(Element::new("div"), prop("name")));
        let bytes = Mrk::bytes_component(&c);
        let back = Mrk::from_bytes_component(&bytes).expect("decode");
        assert_eq!(c, back);
    }

    #[test]
    fn round_trip_with_all_variants() {
        let c = component(
            "complex",
            wrap(
                Element::new("div").attrs(vec![attr("class").value("x")]),
                list![
                    literal(el("h1").children(vec!["Title".into()])),
                    prop("name"),
                    either("admin", prop("a"), prop("b")),
                    maybe("show", prop("c")),
                    match_on(
                        "role",
                        vec![arm("vip", prop("d"))],
                        prop("def"),
                    ),
                    map("items", prop("e")),
                    Node::Text("static".into()),
                ],
            ),
        );
        let bytes = Mrk::bytes_component(&c);
        let back = Mrk::from_bytes_component(&bytes).expect("decode");
        assert_eq!(c, back);
    }

    #[test]
    fn round_trip_component_string_form() {
        let c = component("c", wrap(Element::new("p"), prop("msg")));
        let s = Mrk::to_string_component(&c);
        assert!(s.starts_with("mrk1\nC "));
        let back = Mrk::from_string_component(&s).expect("decode");
        assert_eq!(c, back);
    }

    #[test]
    fn round_trip_component_with_initial_children() {
        let el = Element::new("div").children(vec![Node::Text("static".into())]);
        let c = component("c", wrap(el, prop("dyn")));
        let bytes = Mrk::bytes_component(&c);
        let back = Mrk::from_bytes_component(&bytes).expect("decode");
        assert_eq!(c, back);
    }

    // =================== Element dispatch ===================

    #[test]
    fn element_bytes_still_works() {
        let e = el("div");
        let bytes = Mrk::bytes(&e);
        let back = Mrk::from_bytes(&bytes).expect("decode");
        assert_eq!(e, back);
    }

    #[test]
    fn dispatch_to_element_for_e_header() {
        let e = el("div");
        let bytes = Mrk::bytes(&e);
        let result = Mrk::from_bytes_component(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn dispatch_to_component_for_c_header() {
        let c = component("c", prop("x"));
        let bytes = Mrk::bytes_component(&c);
        let result = Mrk::from_bytes(&bytes);
        assert!(result.is_err());
    }

    #[test]
    fn dispatch_unknown_token() {
        let bytes = b"mrk1\nZ 3:foo\n";
        let result = Mrk::from_bytes(bytes);
        assert!(result.is_err());
    }

    #[test]
    fn dispatch_empty_input() {
        let result = Mrk::from_bytes(b"");
        assert!(result.is_err());
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

// =================================================================
// Coverage for the `?` Err branches inside List/Map/Wrap recursive renders.
// =================================================================

#[test]
fn render_list_inner_type_mismatch() {
    // List body recursion propagates Err from `render_expr`. We force a
    // type mismatch by including an `Either` that expects a bool.
    let c = component(
        "c",
        Expr::List(vec![Box::new(either("flag", prop("yes"), prop("no")))]),
    );
    let mut p = Props::new();
    p.insert("flag", PropType::String("not-a-bool".into()));
    let result = c.render(&p);
    let is_err = matches!(result, Err(RenderError::TypeMismatch { .. }));
    assert!(is_err);
}

#[test]
fn render_map_inner_type_mismatch() {
    // Map body recursion propagates Err.
    let c = component(
        "c",
        Expr::Map {
            input: "items".into(),
            body: Box::new(either("flag", prop("yes"), prop("no"))),
        },
    );
    let mut p = Props::new();
    p.insert("items", PropType::List(vec![PropType::String("a".into())]));
    p.insert("flag", PropType::String("not-a-bool".into()));
    let result = c.render(&p);
    let is_err = matches!(result, Err(RenderError::TypeMismatch { .. }));
    assert!(is_err);
}

#[test]
fn render_wrap_inner_type_mismatch() {
    // Wrap body recursion propagates Err.
    let c = component(
        "c",
        Expr::Wrap {
            name: "div".into(),
            attrs: vec![],
            body: vec![Box::new(either("flag", prop("yes"), prop("no")))],
        },
    );
    let mut p = Props::new();
    p.insert("flag", PropType::String("not-a-bool".into()));
    let result = c.render(&p);
    let is_err = matches!(result, Err(RenderError::TypeMismatch { .. }));
    assert!(is_err);
}

// =====================================================================
// Closure DSL — ExprCtx / Component::build
// =====================================================================

#[test]
fn build_basic_prop() {
    let c = Component::build("c", |ctx| ctx.prop("x"));
    assert_eq!(c.name, "c");
    assert_eq!(c.expr, Expr::Prop("x".into()));
}

#[test]
fn build_returns_same_as_helper() {
    let closure_c = Component::build("c", |ctx| {
        ctx.wrap(
            Element::new("div"),
            list![ctx.prop("a"), ctx.literal(el("span"))],
        )
    });
    let helper_c = component(
        "c",
        wrap(
            Element::new("div"),
            list![prop("a"), literal(el("span"))],
        ),
    );
    assert_eq!(closure_c, helper_c);
}

#[test]
fn build_prop_render() {
    let c = Component::build("c", |ctx| ctx.prop("name"));
    let mut p = Props::new();
    p.insert("name", PropType::String("Bob".into()));
    let nodes = c.render(&p).expect("render");
    assert_eq!(nodes.len(), 1);
    let is_bob = matches!(&nodes[0], Node::Text(t) if t == "Bob");
    assert!(is_bob);
}

#[test]
fn build_literal_render() {
    let c = Component::build("c", |ctx| ctx.literal(el("br")));
    let p = Props::new();
    let nodes = c.render(&p).expect("render");
    assert_eq!(nodes.len(), 1);
    let is_br = matches!(&nodes[0], Node::Element(e) if e.name == "br");
    assert!(is_br);
}

#[test]
fn build_wrap_render() {
    let c = Component::build("c", |ctx| {
        ctx.wrap(
            Element::new("div").attrs(vec![attr("class").value("box")]),
            list![ctx.prop("msg")],
        )
    });
    let mut p = Props::new();
    p.insert("msg", PropType::String("hi".into()));
    let nodes = c.render(&p).expect("render");
    assert_eq!(nodes.len(), 1);
    match &nodes[0] {
        Node::Element(el) => {
            assert_eq!(el.name, "div");
            assert_eq!(el.attributes.len(), 1);
            assert_eq!(el.children.len(), 1);
        }
        _ => panic!("expected Element"),
    }
}

#[test]
fn build_either_render() {
    let c = Component::build("c", |ctx| {
        ctx.either("flag", (ctx.literal(el("yes")), ctx.literal(el("no"))))
    });
    let mut p = Props::new();
    p.insert("flag", PropType::Bool(true));
    let nodes = c.render(&p).expect("render");
    let is_yes = matches!(&nodes[0], Node::Element(e) if e.name == "yes");
    assert!(is_yes);

    p.insert("flag", PropType::Bool(false));
    let nodes = c.render(&p).expect("render");
    let is_no = matches!(&nodes[0], Node::Element(e) if e.name == "no");
    assert!(is_no);
}

#[test]
fn build_maybe_render() {
    let c = Component::build("c", |ctx| ctx.maybe("flag", ctx.literal(el("shown"))));
    let mut p = Props::new();
    p.insert("flag", PropType::Bool(true));
    let nodes = c.render(&p).expect("render");
    assert_eq!(nodes.len(), 1);

    p.insert("flag", PropType::Bool(false));
    let nodes = c.render(&p).expect("render");
    assert!(nodes.is_empty());
}

#[test]
fn build_map_render() {
    let c = Component::build("c", |ctx| ctx.map("items", ctx.prop("item")));
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
fn build_match_basic() {
    let c = Component::build("c", |ctx| {
        ctx.match_on("role", |otherwise| {
            vec![
                ("admin", ctx.literal(el("badge-admin"))).into(),
                ("dev", ctx.literal(el("badge-dev"))).into(),
                (otherwise, ctx.literal(el("badge-guest"))).into(),
            ]
        })
    });
    let mut p = Props::new();
    p.insert("role", PropType::String("admin".into()));
    let nodes = c.render(&p).expect("render");
    let is_admin = matches!(&nodes[0], Node::Element(e) if e.name == "badge-admin");
    assert!(is_admin);

    p.insert("role", PropType::String("unknown".into()));
    let nodes = c.render(&p).expect("render");
    let is_guest = matches!(&nodes[0], Node::Element(e) if e.name == "badge-guest");
    assert!(is_guest);
}

#[test]
fn build_match_default_at_start() {
    let c = Component::build("c", |ctx| {
        ctx.match_on("x", |otherwise| {
            vec![
                (otherwise, ctx.prop("fallback")).into(),
                ("a", ctx.prop("av")).into(),
            ]
        })
    });
    let mut p = Props::new();
    p.insert("x", PropType::String("a".into()));
    p.insert("av", PropType::String("a_val".into()));
    p.insert("fallback", PropType::String("fb_val".into()));
    let nodes = c.render(&p).expect("render");
    let is_av = matches!(&nodes[0], Node::Text(t) if t == "a_val");
    assert!(is_av);

    p.insert("x", PropType::String("z".into()));
    let nodes = c.render(&p).expect("render");
    let is_fb = matches!(&nodes[0], Node::Text(t) if t == "fb_val");
    assert!(is_fb);
}

#[test]
fn build_match_default_in_middle() {
    let c = Component::build("c", |ctx| {
        ctx.match_on("x", |otherwise| {
            vec![
                ("a", ctx.prop("av")).into(),
                (otherwise, ctx.prop("fallback")).into(),
                ("b", ctx.prop("bv")).into(),
            ]
        })
    });
    let mut p = Props::new();
    p.insert("x", PropType::String("a".into()));
    p.insert("av", PropType::String("a_val".into()));
    p.insert("fallback", PropType::String("fb_val".into()));
    p.insert("bv", PropType::String("b_val".into()));
    let nodes = c.render(&p).expect("render");
    let is_av = matches!(&nodes[0], Node::Text(t) if t == "a_val");
    assert!(is_av);

    p.insert("x", PropType::String("c".into()));
    let nodes = c.render(&p).expect("render");
    let is_fb = matches!(&nodes[0], Node::Text(t) if t == "fb_val");
    assert!(is_fb);
}

#[test]
fn build_match_no_arms_only_default() {
    let c = Component::build("c", |ctx| {
        ctx.match_on("x", |otherwise| vec![(otherwise, ctx.prop("fallback")).into()])
    });
    let mut p = Props::new();
    p.insert("x", PropType::String("anything".into()));
    p.insert("fallback", PropType::String("fb_val".into()));
    let nodes = c.render(&p).expect("render");
    let is_fb = matches!(&nodes[0], Node::Text(t) if t == "fb_val");
    assert!(is_fb);
}

#[test]
#[should_panic(expected = "exactly one default arm")]
fn build_match_panics_with_no_default() {
    let _c = Component::build("c", |ctx| {
        ctx.match_on("x", |_: &Otherwise| vec![("a", ctx.prop("av")).into()])
    });
}

#[test]
#[should_panic(expected = "exactly one default arm")]
fn build_match_panics_with_multiple_defaults() {
    let _c = Component::build("c", |ctx| {
        ctx.match_on("x", |otherwise| {
            vec![
                (otherwise, ctx.prop("a")).into(),
                (otherwise, ctx.prop("b")).into(),
            ]
        })
    });
}

#[test]
fn build_match_round_trips_through_mrk() {
    #[cfg(feature = "ir")]
    {
        use crate::ir::Mrk;
        let c = Component::build("c", |ctx| {
            ctx.match_on("role", |otherwise| {
                vec![
                    ("admin", ctx.prop("a")).into(),
                    (otherwise, ctx.prop("fallback")).into(),
                ]
            })
        });
        let bytes = Mrk::bytes_component(&c);
        let back = Mrk::from_bytes_component(&bytes).expect("decode");
        assert_eq!(c, back);
    }
}

#[test]
fn build_match_equivalent_to_match_on_helper() {
    let closure_c = Component::build("c", |ctx| {
        ctx.match_on("role", |otherwise| {
            vec![
                ("a", ctx.prop("av")).into(),
                (otherwise, ctx.prop("fallback")).into(),
            ]
        })
    });
    let helper_c = component(
        "c",
        match_on("role", vec![arm("a", prop("av"))], prop("fallback")),
    );
    assert_eq!(closure_c, helper_c);
}

#[test]
fn build_match_tuple_arm_into() {
    let entry: MatchEntry = (
        Cow::Borrowed("foo") as Cow<'static, str>,
        Expr::Prop("bar".into()),
    )
        .into();
    let is_arm = matches!(entry, MatchEntry::Arm(ref v, _) if v == "foo");
    assert!(is_arm);
}

#[test]
fn build_match_otherwise_into() {
    let entry: MatchEntry = (Otherwise, Expr::Prop("fb".into())).into();
    let is_default = matches!(entry, MatchEntry::Default(_));
    assert!(is_default);
}

#[test]
fn build_nested_closures() {
    let c = Component::build("outer", |ctx| {
        ctx.wrap(
            Element::new("div"),
            list![
                ctx.prop("title"),
                ctx.either(
                    "show_detail",
                    (ctx.prop("detail"), ctx.literal(el("span"))),
                ),
            ],
        )
    });
    let mut p = Props::new();
    p.insert("title", PropType::String("T".into()));
    p.insert("show_detail", PropType::Bool(false));
    let nodes = c.render(&p).expect("render");
    assert_eq!(nodes.len(), 1);
}

#[test]
fn build_element_into_expr() {
    let c = Component::build("c", |ctx| ctx.wrap(el("section"), ctx.literal(el("p"))));
    let p = Props::new();
    let nodes = c.render(&p).expect("render");
    assert_eq!(nodes.len(), 1);
}

#[test]
fn otherwise_is_zero_sized() {
    assert_eq!(std::mem::size_of::<Otherwise>(), 0);
}

#[test]
fn expr_ctx_new() {
    let ctx = ExprCtx::new();
    let _ = ctx.prop("k");
}

#[test]
fn build_round_trip_simple() {
    #[cfg(feature = "ir")]
    {
        use crate::ir::Mrk;
        let c = Component::build("c", |ctx| {
            ctx.wrap(Element::new("p"), list![ctx.prop("name")])
        });
        let bytes = Mrk::bytes_component(&c);
        let back = Mrk::from_bytes_component(&bytes).expect("decode");
        assert_eq!(c, back);
    }
}

#[test]
fn build_round_trip_with_all_variants() {
    #[cfg(feature = "ir")]
    {
        use crate::ir::Mrk;
        let c = Component::build("complex", |ctx| {
            ctx.wrap(
                Element::new("div").attrs(vec![attr("class").value("x")]),
                list![
                    ctx.literal(el("h1").children(vec!["Title".into()])),
                    ctx.prop("name"),
                    ctx.either("admin", (ctx.prop("a"), ctx.prop("b"))),
                    ctx.maybe("show", ctx.prop("c")),
                    ctx.match_on("role", |otherwise| {
                        vec![
                            ("vip", ctx.prop("d")).into(),
                            (otherwise, ctx.prop("def")).into(),
                        ]
                    }),
                    ctx.map("items", ctx.prop("e")),
                    Node::Text("static".into()),
                ],
            )
        });
        let bytes = Mrk::bytes_component(&c);
        let back = Mrk::from_bytes_component(&bytes).expect("decode");
        assert_eq!(c, back);
    }
}

// =====================================================================
// Props::defaults
// =====================================================================

#[test]
fn props_defaults_caller_wins_on_collision() {
    let mut fallbacks = Props::new();
    fallbacks.insert("color", PropType::String("blue".into()));
    let mut caller = Props::new();
    caller.insert("color", PropType::String("red".into()));
    let merged = caller.defaults(&fallbacks);
    assert_eq!(
        merged.get("color"),
        Some(&PropType::String("red".into())),
    );
}

#[test]
fn props_defaults_fills_missing_keys() {
    let mut fallbacks = Props::new();
    fallbacks.insert("size", PropType::Number(Number::int("12")));
    let caller = Props::new();
    let merged = caller.defaults(&fallbacks);
    assert_eq!(
        merged.get("size"),
        Some(&PropType::Number(Number::int("12"))),
    );
}

#[test]
fn props_defaults_does_not_mutate_inputs() {
    let mut fallbacks = Props::new();
    fallbacks.insert("k", PropType::String("fb".into()));
    let mut caller = Props::new();
    caller.insert("k", PropType::String("caller".into()));
    let _merged = caller.defaults(&fallbacks);
    assert_eq!(
        fallbacks.get("k"),
        Some(&PropType::String("fb".into())),
    );
    assert_eq!(
        caller.get("k"),
        Some(&PropType::String("caller".into())),
    );
}

#[test]
fn props_defaults_with_empty_caller() {
    let mut fallbacks = Props::new();
    fallbacks.insert("a", PropType::Bool(true));
    let caller = Props::new();
    let merged = caller.defaults(&fallbacks);
    assert_eq!(merged.len(), 1);
    let is_bool = matches!(merged.get("a"), Some(PropType::Bool(true)));
    assert!(is_bool);
}

#[test]
fn props_defaults_with_empty_defaults() {
    let mut caller = Props::new();
    caller.insert("x", PropType::String("val".into()));
    let fallbacks = Props::new();
    let merged = caller.defaults(&fallbacks);
    assert_eq!(merged.len(), 1);
    assert_eq!(
        merged.get("x"),
        Some(&PropType::String("val".into())),
    );
}

#[test]
fn props_defaults_with_both_empty() {
    let caller = Props::new();
    let fallbacks = Props::new();
    let merged = caller.defaults(&fallbacks);
    assert!(merged.is_empty());
}

#[test]
fn props_defaults_render_uses_fallback() {
    let mut fallbacks = Props::new();
    fallbacks.insert("msg", PropType::String("default msg".into()));
    let caller = Props::new();
    let merged = caller.defaults(&fallbacks);
    let c = component("c", prop("msg"));
    let nodes = c.render(&merged).expect("render");
    let is_default = matches!(&nodes[0], Node::Text(t) if t == "default msg");
    assert!(is_default);
}

// =====================================================================
// Coexistence: old free-function vs new Component::build
// =====================================================================

#[test]
fn coexist_old_and_new_forms_produce_different_structures() {
    let old_form = component("c", prop("x"));
    let new_form = Component::build("c", |ctx| ctx.prop("x"));
    assert_eq!(old_form, new_form);
}

#[test]
fn coexist_old_form_renders() {
    let c = component("c", prop("name"));
    let mut p = Props::new();
    p.insert("name", PropType::String("old".into()));
    let nodes = c.render(&p).expect("render");
    let is_old = matches!(&nodes[0], Node::Text(t) if t == "old");
    assert!(is_old);
}

#[test]
fn coexist_new_form_renders() {
    let c = Component::build("c", |ctx| ctx.prop("name"));
    let mut p = Props::new();
    p.insert("name", PropType::String("new".into()));
    let nodes = c.render(&p).expect("render");
    let is_new = matches!(&nodes[0], Node::Text(t) if t == "new");
    assert!(is_new);
}

#[test]
fn build_wrap_with_literal_children() {
    let el_with_children =
        Element::new("div").children(vec![Node::Text("static".into())]);
    let c = Component::build("c", |ctx| ctx.wrap(el_with_children, ctx.prop("dyn")));
    let p = Props::new();
    let nodes = c.render(&p).expect("render");
    assert_eq!(nodes.len(), 1);
    match &nodes[0] {
        Node::Element(e) => {
            assert_eq!(e.name, "div");
            assert_eq!(e.children.len(), 2);
        }
        _ => panic!("expected Element"),
    }
}

#[test]
fn build_ctx_component_method() {
    let c = ExprCtx::new();
    let comp = c.component("inner", c.prop("x"));
    assert_eq!(comp.name, "inner");
    let is_prop = matches!(comp.expr, Expr::Prop(ref k) if k == "x");
    assert!(is_prop);
}

// =====================================================================
// comp! and text! macros

#[test]
fn comp_macro_bare_tag() {
    let expr = crate::comp!(br);
    let is_wrap = matches!(
        expr,
        Expr::Wrap {
            ref name,
            ref attrs,
            ref body,
        } if name == "br" && attrs.is_empty() && body.is_empty()
    );
    assert!(is_wrap, "expected bare Wrap, got: {expr:?}");
}

#[test]
fn comp_macro_single_child() {
    let expr = crate::comp!(h1, { crate::prop("title") });
    let is_wrap = matches!(
        expr,
        Expr::Wrap {
            ref name,
            ref attrs,
            ref body,
        } if name == "h1" && attrs.is_empty() && body.len() == 1
    );
    assert!(is_wrap, "expected Wrap with one child, got: {expr:?}");
}

#[test]
fn comp_macro_multiple_children() {
    let expr = crate::comp!(div, [crate::comp!(p, { crate::prop("a") }), crate::comp!(p, { crate::prop("b") })]);
    let is_wrap = matches!(
        expr,
        Expr::Wrap {
            ref name,
            ref attrs,
            ref body,
        } if name == "div" && attrs.is_empty() && body.len() == 2
    );
    assert!(is_wrap, "expected Wrap with two children, got: {expr:?}");
}

#[test]
fn comp_macro_attrs_single_child() {
    let expr = crate::comp!(div, class="card", id="main", { crate::prop("title") });
    match expr {
        Expr::Wrap { name, attrs, body } => {
            assert_eq!(name, "div");
            assert_eq!(attrs.len(), 2);
            assert_eq!(body.len(), 1);
            assert!(matches!(
                &attrs[0],
                WrappedAttribute::Dynamic(k, _) if k == "class"
            ));
            assert!(matches!(
                &attrs[1],
                WrappedAttribute::Dynamic(k, _) if k == "id"
            ));
        }
        _ => panic!("expected Wrap, got: {expr:?}"),
    }
}

#[test]
fn comp_macro_attrs_multiple_children() {
    let expr = crate::comp!(ul, class="list", [crate::comp!(li, { crate::prop("item") })]);
    match expr {
        Expr::Wrap { name, attrs, body } => {
            assert_eq!(name, "ul");
            assert_eq!(attrs.len(), 1);
            assert_eq!(body.len(), 1);
            assert!(matches!(
                &attrs[0],
                WrappedAttribute::Dynamic(k, _) if k == "class"
            ));
        }
        _ => panic!("expected Wrap, got: {expr:?}"),
    }
}

#[test]
fn text_macro_single() {
    let expr = crate::text!("hello");
    let is_list = matches!(expr, Expr::List(ref items) if items.len() == 1);
    assert!(is_list, "expected List with one item, got: {expr:?}");
}

#[test]
fn text_macro_multiple() {
    let expr = crate::text!(crate::prop("first"), " ", crate::prop("last"));
    let is_list = matches!(expr, Expr::List(ref items) if items.len() == 3);
    assert!(is_list, "expected List with three items, got: {expr:?}");
}

#[test]
fn comp_macro_render_bare() {
    let c = component("c", crate::comp!(br));
    let p = Props::new();
    let nodes = c.render(&p).expect("render");
    let html: String = nodes.iter().map(|n| n.render()).collect();
    assert_eq!(html, "<br>");
}

#[test]
fn comp_macro_render_with_dynamic_attr() {
    let c = component("c", crate::comp!(div, class="card", { crate::prop("text") }));
    let mut p = Props::new();
    p.insert("text", PropType::String("hello".into()));
    let nodes = c.render(&p).expect("render");
    let html: String = nodes.iter().map(|n| n.render()).collect();
    assert_eq!(html, "<div class=\"card\">hello</div>");
}

#[test]
fn comp_macro_render_nested() {
    let c = component(
        "c",
        crate::comp!(div, [
            crate::comp!(h1, { crate::prop("title") }),
            crate::comp!(p, { crate::prop("body") }),
        ]),
    );
    let mut p = Props::new();
    p.insert("title", PropType::String("Hi".into()));
    p.insert("body", PropType::String("World".into()));
    let nodes = c.render(&p).expect("render");
    let html: String = nodes.iter().map(|n| n.render()).collect();
    assert_eq!(
        html,
        "<div><h1>Hi</h1><p>World</p></div>"
    );
}

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
    let b = WrappedAttribute::Dynamic("k".into(), Expr::LiteralChildren(vec![Node::Text("v".into())]));
    assert_ne!(a, b);
}

#[test]
fn into_expr_str_in_comp_macro() {
    let expr = crate::comp!(p, { "static text" });
    match expr {
        Expr::Wrap { name, attrs, body } => {
            assert_eq!(name, "p");
            assert!(attrs.is_empty());
            assert_eq!(body.len(), 1);
            let is_literal = matches!(*body[0], Expr::LiteralChildren(_));
            assert!(is_literal, "expected LiteralChildren, got: {:?}", *body[0]);
        }
        _ => panic!("expected Wrap, got: {expr:?}"),
    }
}

#[test]
fn comp_macro_dynamic_attr_multi_node_fallback() {
    let c = component(
        "c",
        Expr::Wrap {
            name: "div".into(),
            attrs: vec![WrappedAttribute::Dynamic(
                "class".into(),
                Expr::LiteralChildren(vec![
                    Node::Text("a".into()),
                    Node::Element(crate::element::el("span")),
                ]),
            )],
            body: vec![],
        },
    );
    let p = Props::new();
    let nodes = c.render(&p).expect("render");
    let html: String = nodes.iter().map(|n| n.render()).collect();
    assert_eq!(html, r#"<div class="a&lt;span&gt;&lt;/span&gt;"></div>"#);
}

#[test]
fn ir_round_trip_dynamic_attr() {
    use crate::ir::Mrk;
    let c = component(
        "c",
        Expr::Wrap {
            name: "div".into(),
            attrs: vec![WrappedAttribute::Dynamic(
                "class".into(),
                Expr::Prop("cls".into()),
            )],
            body: vec![],
        },
    );
    let bytes = Mrk::bytes_component(&c);
    let back = Mrk::from_bytes_component(&bytes).expect("decode");
    assert_eq!(c, back);
}

#[test]
fn ir_round_trip_mixed_attrs() {
    use crate::ir::Mrk;
    let c = component(
        "c",
        Expr::Wrap {
            name: "div".into(),
            attrs: vec![
                WrappedAttribute::Static(attr("id").value("main")),
                WrappedAttribute::Dynamic("class".into(), Expr::Prop("cls".into())),
            ],
            body: vec![],
        },
    );
    let bytes = Mrk::bytes_component(&c);
    let back = Mrk::from_bytes_component(&bytes).expect("decode");
    assert_eq!(c, back);
}

#[test]
fn wrap_dynamic_attr_render_expr_error() {
    let c = Component {
        name: "test".into(),
        expr: Expr::Wrap {
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
        },
    };
    let err = c.render(&Props::new()).unwrap_err();
    assert!(matches!(err, RenderError::TypeMismatch { .. }));
}
