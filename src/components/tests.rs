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
    arm, component, either, list_expr, literal, map, match_on, maybe, prop, wrap, Expr, IntoExpr,
    Number, NumberKind, PropType, Props, RenderError,
};
use crate::components::list;
use crate::element::{el, Element};
use crate::node::Node;

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
