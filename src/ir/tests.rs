// Tests for the `ir` module. Lives in a separate file inside the
// `ir/` directory.
//
// As with the components tests, the `matches!` false-branch regions in
// assertions are uncatchable in passing tests, but per-file
// instrumentation still measures them. Acceptable for this project
// because exercised code is test-only.

use std::borrow::Cow;

use crate::attributes::attr;
use crate::components::Component;
use crate::element::el;
use crate::ir::{Mrk, ParseError};
use crate::node::Node;
use crate::nodes;
use crate::Element;

fn assert_round_trip(e: &Element) {
    let bytes = Mrk::bytes(e);
    let back = Mrk::from_bytes(&bytes).expect("decode failed");
    assert_eq!(e, &back, "round-trip mismatch");

    let s = Mrk::to_string(e);
    let back_s = Mrk::from_string(&s).expect("string decode failed");
    assert_eq!(e, &back_s, "string round-trip mismatch");
}

#[test]
fn empty_element() {
    let e = el("div");
    assert_round_trip(&e);
    let s = Mrk::to_string(&e);
    assert_eq!(s, "mrk1\nE 3:div\n");
}

#[test]
fn attribute_kv_only() {
    let e = el("a").attrs(vec![attr("href").value("/")]);
    assert_round_trip(&e);
}

#[test]
fn attribute_bool_only() {
    let e = el("input").attrs(vec![attr("disabled")]);
    assert_round_trip(&e);
}

#[test]
fn attribute_kv_empty_value() {
    let e = el("input").attrs(vec![attr("value").value("")]);
    assert_round_trip(&e);
    let back = Mrk::from_string(&Mrk::to_string(&e)).unwrap();
    let is_kv = matches!(
        back.attributes[0].attr,
        crate::AttributeType::KeyValue(_, _)
    );
    assert!(is_kv);
}

#[test]
fn text_children() {
    let e = el("p").children(nodes!["hello", " ", "world"]);
    assert_round_trip(&e);
}

#[test]
#[cfg(feature = "html")]
fn raw_children() {
    use crate::html::Raw;
    let e = el("p").children(vec![Raw::str("<em>x</em>")]);
    assert_round_trip(&e);
}

#[test]
#[cfg(feature = "html")]
fn mixed_children() {
    use crate::html::Raw;
    let e = el("div").children(vec![
        "text".into(),
        el("span").into(),
        Raw::str("<br/>"),
    ]);
    assert_round_trip(&e);
}

#[test]
fn nested_elements() {
    let e = el("div")
        .attrs(vec![attr("class").value("blue box")])
        .children(vec![
            "Hello".into(),
            el("p").children(vec!["world".into()]).into(),
        ]);
    assert_round_trip(&e);

    let s = Mrk::to_string(&e);
    assert!(s.starts_with("mrk1\nE 3:div\n"));
    assert!(s.contains("A 5:class 8:blue box\n"));
    assert!(s.contains("E 1:p\n"));
    assert!(s.contains("T 5:world\n"));
}

#[test]
fn text_with_control_bytes_and_unicode() {
    let e = el("p").children(vec!["héllo\t日本".into()]);
    assert_round_trip(&e);
}

#[test]
#[cfg(feature = "html")]
fn raw_with_html() {
    use crate::html::Raw;
    let raw_str = "<em class=\"ok\">x</em>\tnext";
    let e = el("div").children(vec![Raw::str(raw_str)]);
    assert_round_trip(&e);
}

#[test]
fn attribute_kv_with_colon_and_digits() {
    let e = el("a").attrs(vec![attr("href").value("https://example.com:8080/x?y=1")]);
    assert_round_trip(&e);
}

#[test]
fn deeply_nested() {
    let leaf = el("leaf").children(vec!["deep".into()]);
    let l3 = el("l3").children(vec![leaf.into()]);
    let l2 = el("l2").children(vec![l3.into()]);
    let l1 = el("l1").children(vec![l2.into()]);
    let root = el("root").children(vec![l1.into()]);
    assert_round_trip(&root);
}

#[test]
fn unknown_token_at_root() {
    let s = "mrk1\nZ 3:div\n";
    let err = Mrk::from_string(s).unwrap_err();
    let is_unknown = matches!(err, ParseError::UnknownToken { .. });
    assert!(is_unknown);
}

#[test]
fn missing_header() {
    let err = Mrk::from_string("").unwrap_err();
    assert_eq!(err, ParseError::MissingVersion);
}

#[test]
fn unknown_version() {
    let err = Mrk::from_string("mrk2\nE 3:div\n").unwrap_err();
    assert_eq!(err, ParseError::UnknownVersion(2));
}

#[test]
fn attribute_after_child_rejected() {
    let s = "mrk1\nE 3:div\n  T 5:hello\n  A 1:k 1:v\n";
    let err = Mrk::from_string(s).unwrap_err();
    let is_after = matches!(err, ParseError::AttributeAfterChild { .. });
    assert!(is_after);
}

#[test]
fn truncated_payload_rejected() {
    let s = "mrk1\nE 5:div\n";
    let err = Mrk::from_string(s).unwrap_err();
    let is_trunc = matches!(err, ParseError::TruncatedPayload { .. });
    assert!(is_trunc);
}

#[test]
fn bad_length_prefix_rejected() {
    let s = "mrk1\nE foo:div\n";
    let err = Mrk::from_string(s).unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn bad_length_prefix_length_overflow() {
    let err = Mrk::from_string("mrk1\nE 99999999999999999999:div\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn bad_length_prefix_non_utf8_payload() {
    let err = Mrk::from_bytes(b"mrk1\nE 5:\xc3\x28\x80\x80\x80\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn payload_exceeds_cap() {
    let too_big = crate::ir::MAX_PAYLOAD + 1;
    let s = format!("mrk1\nE {}:x\n", too_big);
    let err = Mrk::from_string(&s).unwrap_err();
    let is_over = matches!(err, ParseError::LengthExceedsCap { .. });
    assert!(is_over);
}

#[test]
fn encoder_refuses_oversize_payload() {
    let big = "x".repeat(crate::ir::MAX_PAYLOAD + 1);
    let e = el("p").children(vec![big.into()]);
    let result = std::panic::catch_unwind(|| Mrk::bytes(&e));
    assert!(result.is_err());
}

#[test]
fn display_produces_ir_for_element() {
    let e = el("div");
    let s = format!("{}", e);
    assert!(s.starts_with("mrk1\n"));
}

#[test]
fn parse_matches_locked_example() {
    let input = "mrk1\nE 3:div\n  A 5:class 8:blue box\n  T 5:hello\n  E 1:p\n    T 2:hi\n";
    let e = Mrk::from_string(input).unwrap();
    let round = Mrk::to_string(&e);
    assert_eq!(round, input);
}

#[test]
fn parses_crlf_line_endings() {
    let input = "mrk1\r\nE 3:div\r\n";
    let e = Mrk::from_string(input).unwrap();
    assert_eq!(e.name, "div");
}

#[test]
fn parse_error_display_all_variants() {
    let cases: Vec<(ParseError, &str)> = vec![
        (ParseError::MissingVersion, "missing"),
        (ParseError::UnknownVersion(99), "mrk99"),
        (ParseError::UnknownToken { line: 7, got: b'Q' }, "line 7"),
        (ParseError::BadLengthPrefix { line: 3 }, "line 3"),
        (ParseError::LengthExceedsCap { line: 2, length: 99 }, "line 2"),
        (
            ParseError::TruncatedPayload {
                line: 4,
                expected: 5,
                got: 2,
            },
            "line 4",
        ),
        (ParseError::AttributeAfterChild { line: 6 }, "line 6"),
        (ParseError::UnexpectedEof, "end"),
        (ParseError::BadNesting { line: 1 }, "line 1"),
        (
            ParseError::MissingField {
                line: 8,
                expected: b'A',
            },
            "line 8",
        ),
    ];
    for (err, needle) in &cases {
        let s = format!("{}", err);
        assert!(s.contains(needle), "rendered {:?} missing {:?}", err, needle);
    }
    let _: Box<dyn std::error::Error> = Box::new(cases[0].0.clone());
}

#[test]
fn header_starts_with_mrk_but_unparseable_version() {
    let err = Mrk::from_string("mrkX\nE 3:div\n").unwrap_err();
    assert_eq!(err, ParseError::MissingVersion);
}

#[test]
fn header_does_not_start_with_mrk() {
    let err = Mrk::from_string("foo\nE 3:div\n").unwrap_err();
    assert_eq!(err, ParseError::MissingVersion);
}

#[test]
fn bad_nesting_root_not_at_indent_zero() {
    let err = Mrk::from_string("mrk1\n  E 3:div\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadNesting { .. });
    assert!(is_bad);
}

#[test]
fn bad_nesting_child_wrong_indent() {
    let err = Mrk::from_string("mrk1\nE 3:div\n   E 1:p\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadNesting { .. });
    assert!(is_bad);
}

#[test]
fn attribute_after_child_for_b() {
    let err =
        Mrk::from_string("mrk1\nE 3:div\n  T 5:hello\n  B 5:other\n").unwrap_err();
    let is_after = matches!(err, ParseError::AttributeAfterChild { .. });
    assert!(is_after);
}

#[test]
fn unknown_token_in_child_position() {
    let err = Mrk::from_string("mrk1\nE 3:div\n  Z 3:foo\n").unwrap_err();
    let is_unknown = matches!(err, ParseError::UnknownToken { .. });
    assert!(is_unknown);
}

#[test]
fn unexpected_eof_after_header() {
    let err = Mrk::from_string("mrk1\n").unwrap_err();
    assert_eq!(err, ParseError::UnexpectedEof);
}

#[test]
fn missing_field_a_without_value() {
    let err = Mrk::from_string("mrk1\nE 3:div\n  A 1:k\n").unwrap_err();
    let is_missing = matches!(err, ParseError::MissingField { expected: b'A', .. });
    assert!(is_missing);
}

#[test]
fn missing_field_e_without_name() {
    let err = Mrk::from_string("mrk1\nE\n").unwrap_err();
    let is_missing = matches!(err, ParseError::MissingField { expected: b'E', .. });
    assert!(is_missing);
}

#[test]
fn bad_length_prefix_no_colon() {
    let err = Mrk::from_string("mrk1\nE 3div\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn bad_length_prefix_a_second_field() {
    let err = Mrk::from_string("mrk1\nE 3:div\n  A 1:k abc:v\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn bad_length_prefix_a_first_field() {
    let err = Mrk::from_string("mrk1\nE 3:div\n  A abc:v\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn bad_length_prefix_a_key_non_utf8() {
    // First field Ok, second field Ok, key bytes invalid UTF-8.
    let err = Mrk::from_bytes(b"mrk1\nE 3:div\n  A 1:\xc3 1:v\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn bad_nested_element_recurses() {
    // A nested E with malformed content triggers parse_element Err.
    let err = Mrk::from_string("mrk1\nE 3:div\n  E abc:p\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn bad_length_prefix_b_first_field() {
    let err = Mrk::from_string("mrk1\nE 3:div\n  B abc\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn bad_length_prefix_t_first_field() {
    let err = Mrk::from_string("mrk1\nE 3:div\n  T abc\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn bad_length_prefix_r_first_field() {
    let err = Mrk::from_string("mrk1\nE 3:div\n  R abc\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn bad_length_prefix_a_value_non_utf8() {
    let err = Mrk::from_bytes(b"mrk1\nE 3:div\n  A 1:k 5:\xc3\x28\x80\x80\x80\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn bad_length_prefix_b_non_utf8() {
    let err = Mrk::from_bytes(b"mrk1\nE 3:div\n  B 5:\xc3\x28\x80\x80\x80\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn bad_length_prefix_t_non_utf8() {
    let err = Mrk::from_bytes(b"mrk1\nE 3:div\n  T 5:\xc3\x28\x80\x80\x80\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn bad_length_prefix_r_non_utf8() {
    let err = Mrk::from_bytes(b"mrk1\nE 3:div\n  R 5:\xc3\x28\x80\x80\x80\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn bad_length_prefix_invalid_trailing_byte() {
    let err = Mrk::from_string("mrk1\nE 3:divX\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn blank_line_at_eof_is_skipped() {
    let e = Mrk::from_string("mrk1\nE 3:div\n\n").unwrap();
    assert_eq!(e.name, "div");
}

#[test]
fn no_trailing_newline() {
    // Last line has no trailing \n — exercises eof=true in scan_line loop.
    let e = Mrk::from_string("mrk1\nE 3:div").unwrap();
    assert_eq!(e.name, "div");
}

#[test]
fn display_node_text() {
    let n: Node = "hello".into();
    assert_eq!(format!("{}", n), "hello");
}

#[test]
fn display_node_raw() {
    let n = Node::Raw("<br/>".into());
    assert_eq!(format!("{}", n), "<br/>");
}

#[test]
fn round_trip_raw_node() {
    // Construct a `Node::Raw` directly (without the `html::Raw`
    // helper) and verify it round-trips through the wire format.
    // Length 8: "<b>x</b>" is 8 bytes.
    let e = el("div").children(vec![Node::Raw("<b>x</b>".into())]);
    let s = Mrk::to_string(&e);
    assert!(s.contains("R 8:<b>x</b>"), "raw node not encoded: {s}");
    let bytes = Mrk::bytes(&e);
    let back = Mrk::from_bytes(&bytes).expect("decode failed");
    assert_eq!(e, back);
}

#[test]
fn parser_handles_raw_node_as_first_child() {
    // Exercises the `b'R' =>` parser arm at line 558 of parser.rs.
    // Length 8: "<p>x</p>" is 8 bytes.
    let bytes = b"mrk1\nE 3:div\n  R 8:<p>x</p>\n";
    let e = Mrk::from_bytes(bytes).expect("decode");
    assert_eq!(e.children.len(), 1);
    assert!(matches!(e.children[0], Node::Raw(_)));
}

#[test]
fn component_round_trip_with_raw_literal_children() {
    // Drives `encode_node` for the `Node::Raw` arm (encoder.rs:202),
    // the `b'R' =>` parser arms, and the `Expr::LiteralChildren`
    // encode/decode paths. Constructs a component containing
    // `Expr::LiteralChildren` with raw HTML — no `html::Raw` helper.
    use crate::components::{Component, Expr};

    let c = Component {
        name: Cow::Borrowed("rc"),
        expr: Expr::LiteralChildren(vec![Node::Raw("<b>x</b>".into())]),
    };
    let bytes = Mrk::bytes_component(&c);
    let back = Mrk::from_bytes_component(&bytes).expect("decode");
    assert_eq!(c, back);
}

#[test]
fn display_attribute_kv() {
    let a = attr("href").value("/");
    assert_eq!(format!("{}", a), "href=/");
}

#[test]
fn display_attribute_bool() {
    let a = attr("disabled");
    assert_eq!(format!("{}", a), "disabled");
}

#[test]
fn display_attribute_type_kv() {
    let at = crate::AttributeType::KeyValue(Cow::Borrowed("k"), Cow::Borrowed("v"));
    assert_eq!(format!("{}", at), "k=v");
}

#[test]
fn display_attribute_type_bool() {
    let at = crate::AttributeType::Bool(Cow::Borrowed("flag"));
    assert_eq!(format!("{}", at), "flag");
}

// =================================================================
// Component IR coverage (parser error branches).
// =================================================================

#[test]
fn blank_lines_with_only_spaces_are_skipped() {
    // Dispatch must tolerate whitespace-only lines between the header and
    // the first non-blank content line.
    let e = Mrk::from_string("mrk1\n   \nE 3:div\n").expect("decode");
    assert_eq!(e.name, "div");
}

#[test]
fn component_with_unknown_version_header() {
    let err = Mrk::from_string_component("mrk2\nC 5:greet\n  P 4:name\n").unwrap_err();
    assert_eq!(err, ParseError::UnknownVersion(2));
}

#[test]
fn component_with_unparseable_header() {
    let err = Mrk::from_string_component("mrkFoo\nC 5:greet\n  P 4:name\n").unwrap_err();
    assert_eq!(err, ParseError::MissingVersion);
}

#[test]
fn component_expr_line_wrong_indent() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\nP 4:name\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadNesting { .. });
    assert!(is_bad);
}

#[test]
fn component_missing_expr_line() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n").unwrap_err();
    assert_eq!(err, ParseError::UnexpectedEof);
}

#[test]
fn component_comp_line_no_name_field() {
    // `C` alone has no name field — forces the `?` Err branch on the
    // `field_payload` call inside `parse_component_root`.
    let err = Mrk::from_string_component("mrk1\nC\n  P 1:k\n").unwrap_err();
    assert_eq!(
        err,
        ParseError::MissingField {
            line: 2,
            expected: b'C'
        }
    );
}

#[test]
fn component_comp_line_name_non_utf8() {
    // Non-UTF-8 bytes in the name field — forces the `?` Err branch on
    // the `bytes_to_string` call inside `parse_component_root`.
    let err = Mrk::from_bytes_component(b"mrk1\nC 5:\xc3\x28\x80\x80\x80\n  P 1:k\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn component_expr_l_truncated() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  L\n").unwrap_err();
    assert_eq!(err, ParseError::UnexpectedEof);
}

#[test]
fn component_expr_l_element_wrong_indent() {
    let err =
        Mrk::from_string_component("mrk1\nC 5:greet\n  L\n E 3:div\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadNesting { .. });
    assert!(is_bad);
}

#[test]
fn component_expr_l_element_wrong_kind() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  L\n    Z 1:x\n").unwrap_err();
    let is_unknown = matches!(err, ParseError::UnknownToken { .. });
    assert!(is_unknown);
}

#[test]
fn component_expr_s_item_wrong_indent() {
    let err =
        Mrk::from_string_component("mrk1\nC 5:greet\n  S 1:1\n P 1:k\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadNesting { .. });
    assert!(is_bad);
}

#[test]
fn component_expr_m_value_wrong_indent() {
    let err = Mrk::from_string_component(
        "mrk1\nC 5:greet\n  M 1:k 1:1\n x\n    P 1:k\n",
    )
    .unwrap_err();
    let is_bad = matches!(err, ParseError::BadNesting { .. });
    assert!(is_bad);
}

#[test]
fn component_expr_m_arm_wrong_indent() {
    let err = Mrk::from_string_component(
        "mrk1\nC 5:greet\n  M 1:k 1:1\n    1:x\n P 1:k\n",
    )
    .unwrap_err();
    let is_bad = matches!(err, ParseError::BadNesting { .. });
    assert!(is_bad);
}

#[test]
fn component_expr_m_default_wrong_indent() {
    let err = Mrk::from_string_component(
        "mrk1\nC 5:greet\n  M 1:k 0:\n  P 1:k\n",
    )
    .unwrap_err();
    eprintln!("got: {:?}", err);
    let is_bad = matches!(err, ParseError::BadNesting { .. });
    assert!(is_bad);
}

#[test]
fn component_expr_m_value_no_colon() {
    let err = Mrk::from_string_component(
        "mrk1\nC 5:greet\n  M 1:k 1:1\n    x\n    P 1:k\n",
    )
    .unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn component_expr_i_then_wrong_indent() {
    let err = Mrk::from_string_component(
        "mrk1\nC 5:greet\n  I 1:c\n P 1:k\n    P 1:k\n",
    )
    .unwrap_err();
    let is_bad = matches!(err, ParseError::BadNesting { .. });
    assert!(is_bad);
}

#[test]
fn component_expr_i_else_wrong_indent() {
    let err = Mrk::from_string_component(
        "mrk1\nC 5:greet\n  I 1:c\n    P 1:k\n P 1:k\n",
    )
    .unwrap_err();
    let is_bad = matches!(err, ParseError::BadNesting { .. });
    assert!(is_bad);
}

#[test]
fn component_expr_o_then_wrong_indent() {
    let err =
        Mrk::from_string_component("mrk1\nC 5:greet\n  O 1:c\n P 1:k\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadNesting { .. });
    assert!(is_bad);
}

#[test]
fn component_expr_f_body_wrong_indent() {
    let err =
        Mrk::from_string_component("mrk1\nC 5:greet\n  F 1:i\n P 1:k\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadNesting { .. });
    assert!(is_bad);
}

#[test]
fn component_expr_w_attr_wrong_indent() {
    let err = Mrk::from_string_component(
        "mrk1\nC 5:greet\n  W 1:p 1:1 0:\n A 1:k 1:v\n",
    )
    .unwrap_err();
    eprintln!("got: {:?}", err);
    let is_bad = matches!(err, ParseError::BadNesting { .. });
    assert!(is_bad);
}

#[test]
fn debug_w_line() {
    let s = "mrk1\nC 1:c\n  W 1:p 1:1 0:\n A 1:k 1:v\n";
    let err = Mrk::from_string_component(s).unwrap_err();
    println!("got: {:?}, full expected: {:?}", err, s);
}

#[test]
fn component_expr_w_body_wrong_indent() {
    let err = Mrk::from_string_component(
        "mrk1\nC 5:greet\n  W 1:p 1:0 1:1\n P 1:k\n",
    )
    .unwrap_err();
    let is_bad = matches!(err, ParseError::BadNesting { .. });
    assert!(is_bad);
}

#[test]
fn component_expr_w_missing_body_count_field() {
    let err =
        Mrk::from_string_component("mrk1\nC 5:greet\n  W 1:p 1:0\n").unwrap_err();
    let is_missing = matches!(err, ParseError::MissingField { .. });
    assert!(is_missing);
}

#[test]
fn round_trip_w_with_bool_attribute() {
    use crate::components::{Expr, WrappedAttribute};
    let c = Component {
        name: Cow::Borrowed("c"),
        expr: Expr::Wrap {
            name: "div".into(),
            attrs: vec![WrappedAttribute::Static(attr("checked"))],
            body: vec![],
        },
    };
    let bytes = Mrk::bytes_component(&c);
    let back = Mrk::from_bytes_component(&bytes).expect("decode");
    assert_eq!(c, back);
}

#[test]
fn read_lp_value_no_colon() {
    // `name_b` here is a Match arm whose value line has no `:` to start
    // the length-prefixed payload. The parser must reject it as a
    // malformed length prefix.
    let err = Mrk::from_string_component(
        "mrk1\nC 1:c\n  M 4:role 1:1\n    X\n    P 1:k\n",
    )
    .unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn read_lp_value_non_digit_length_bytes() {
    // Length prefix `non_digit` is rejected because the bytes before
    // the colon are not all ASCII digits.
    let err = Mrk::from_string_component(
        "mrk1\nC 1:c\n  M 4:role 1:1\n    ab:1\n    P 1:k\n",
    )
    .unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn read_lp_value_length_overflow() {
    // A length prefix of `99999999999999999999` exceeds usize::MAX, so
    // `parse::<usize>()` fails and the `.map_err` closure in `read_lp_value`
    // fires.
    let err = Mrk::from_string_component(
        "mrk1\nC 1:c\n  M 4:role 1:1\n    99999999999999999999:x\n    P 1:k\n",
    )
    .unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn read_nth_field_missing_after_inner_loop() {
    let err =
        Mrk::from_string_component("mrk1\nC 5:greet\n  W 1:p\n").unwrap_err();
    eprintln!("got: {:?}", err);
    let is_missing = matches!(err, ParseError::MissingField { .. });
    assert!(is_missing);
}

#[test]
fn w_line_with_no_fields_after_kind() {
    // `W` followed by nothing — the parser must reject with
    // MissingField rather than panic.
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  W\n").unwrap_err();
    eprintln!("W alone: {:?}", err);
    let is_missing = matches!(err, ParseError::MissingField { .. });
    assert!(is_missing);
}

#[test]
fn w_line_with_only_name_no_attr_count() {
    // `W 1:p` — name present, but no attr-count field. The parser must
    // reject with MissingField at line 3.
    let err =
        Mrk::from_string_component("mrk1\nC 5:greet\n  W 1:p\n").unwrap_err();
    eprintln!("W only name: {:?}", err);
    let is_missing = matches!(err, ParseError::MissingField { .. });
    assert!(is_missing);
}

// =================================================================
// Coverage for Err-propagation in parse_expr variants.
// Each test forces a specific `?` operator to fire its Err branch.
// =================================================================

#[test]
fn parse_expr_l_prop_name_bad_length_prefix() {
    // `P abc:name` — field_payload fails on a non-digit length prefix.
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  P abc:name\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_l_prop_name_non_utf8() {
    // Prop name bytes are not valid UTF-8.
    let err = Mrk::from_bytes_component(b"mrk1\nC 5:greet\n  P 5:\xc3\x28\x80\x80\x80\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_l_element_name_bad_length_prefix() {
    // `L` is parsed; inside the nested element the name field is bad.
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  L\n    E abc:div\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_l_element_name_non_utf8() {
    let err = Mrk::from_bytes_component(b"mrk1\nC 5:greet\n  L\n    E 5:\xc3\x28\x80\x80\x80\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_l_element_attr_bad_length_prefix() {
    let err = Mrk::from_string_component(
        "mrk1\nC 5:greet\n  L\n    E 3:div\n      A 1:k abc:v\n",
    )
    .unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_s_count_bad_length_prefix() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  S abc:2\n    P 1:k\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_s_count_non_utf8() {
    let err = Mrk::from_bytes_component(b"mrk1\nC 5:greet\n  S 5:\xc3\x28\x80\x80\x80\n    P 1:k\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_s_count_unparseable() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  S 1:x\n    P 1:k\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_s_item_expr_fails() {
    // S has 1 item at the right indent, but the item itself fails to parse.
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  S 1:1\n    Z 1:x\n").unwrap_err();
    let is_unknown = matches!(err, ParseError::UnknownToken { .. });
    assert!(is_unknown);
}

#[test]
fn parse_expr_m_key_bad_length_prefix() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  M abc:k 0:\n    P 1:k\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_m_key_non_utf8() {
    let err = Mrk::from_bytes_component(b"mrk1\nC 5:greet\n  M 5:\xc3\x28\x80\x80\x80 0:\n    P 1:k\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_m_count_bad_length_prefix() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  M 1:k abc:2\n    P 1:k\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_m_count_non_utf8() {
    let err = Mrk::from_bytes_component(b"mrk1\nC 5:greet\n  M 1:k 5:\xc3\x28\x80\x80\x80\n    P 1:k\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_m_count_unparseable() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  M 1:k 1:x\n    P 1:k\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_m_value_non_utf8() {
    // `1:x` is fine as bytes; use a value with non-UTF8 payload.
    let err = Mrk::from_bytes_component(
        b"mrk1\nC 5:greet\n  M 1:k 1:1\n    5:\xc3\x28\x80\x80\x80\n    P 1:k\n",
    )
    .unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_m_arm_expr_fails() {
    // One arm with value but the expression after it is bad.
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  M 1:k 1:1\n    1:x\n    Z 1:x\n").unwrap_err();
    let is_unknown = matches!(err, ParseError::UnknownToken { .. });
    assert!(is_unknown);
}

#[test]
fn parse_expr_m_default_expr_fails() {
    // Default expression is bad.
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  M 1:k 0:\n    Z 1:x\n").unwrap_err();
    let is_unknown = matches!(err, ParseError::UnknownToken { .. });
    assert!(is_unknown);
}

#[test]
fn parse_expr_i_cond_bad_length_prefix() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  I abc:c\n    P 1:k\n    P 1:k\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_i_cond_non_utf8() {
    let err = Mrk::from_bytes_component(b"mrk1\nC 5:greet\n  I 5:\xc3\x28\x80\x80\x80\n    P 1:k\n    P 1:k\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_i_then_expr_fails() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  I 1:c\n    Z 1:x\n    P 1:k\n").unwrap_err();
    let is_unknown = matches!(err, ParseError::UnknownToken { .. });
    assert!(is_unknown);
}

#[test]
fn parse_expr_i_else_expr_fails() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  I 1:c\n    P 1:k\n    Z 1:x\n").unwrap_err();
    let is_unknown = matches!(err, ParseError::UnknownToken { .. });
    assert!(is_unknown);
}

#[test]
fn parse_expr_o_cond_bad_length_prefix() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  O abc:c\n    P 1:k\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_o_cond_non_utf8() {
    let err = Mrk::from_bytes_component(b"mrk1\nC 5:greet\n  O 5:\xc3\x28\x80\x80\x80\n    P 1:k\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_o_then_expr_fails() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  O 1:c\n    Z 1:x\n").unwrap_err();
    let is_unknown = matches!(err, ParseError::UnknownToken { .. });
    assert!(is_unknown);
}

#[test]
fn parse_expr_f_input_bad_length_prefix() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  F abc:i\n    P 1:k\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_f_input_non_utf8() {
    let err = Mrk::from_bytes_component(b"mrk1\nC 5:greet\n  F 5:\xc3\x28\x80\x80\x80\n    P 1:k\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_f_body_expr_fails() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  F 1:i\n    Z 1:x\n").unwrap_err();
    let is_unknown = matches!(err, ParseError::UnknownToken { .. });
    assert!(is_unknown);
}

#[test]
fn parse_expr_w_name_bad_length_prefix() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  W abc:p 0:0 0:\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_w_name_non_utf8() {
    let err = Mrk::from_bytes_component(b"mrk1\nC 5:greet\n  W 5:\xc3\x28\x80\x80\x80 0:0 0:\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_w_attr_count_bad_length_prefix() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  W 1:p abc:0 0:\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_w_attr_count_non_utf8() {
    let err = Mrk::from_bytes_component(b"mrk1\nC 5:greet\n  W 1:p 5:\xc3\x28\x80\x80\x80 0:\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_w_attr_count_unparseable() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  W 1:p 1:x 0:\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_w_body_count_bad_length_prefix() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  W 1:p 0:0 abc:0\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_w_body_count_non_utf8() {
    let err = Mrk::from_bytes_component(b"mrk1\nC 5:greet\n  W 1:p 1:0 5:\xc3\x28\x80\x80\x80\n").unwrap_err();
    eprintln!("got: {:?}", err);
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_w_body_count_unparseable() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  W 1:p 1:0 1:x\n").unwrap_err();
    eprintln!("got: {:?}", err);
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_w_attr_expr_fails() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  W 1:p 1:1 0:\n    Z 1:x\n").unwrap_err();
    let is_unknown = matches!(err, ParseError::UnknownToken { .. });
    assert!(is_unknown);
}

#[test]
fn parse_expr_w_body_expr_fails() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  W 1:p 1:0 1:1\n    Z 1:x\n").unwrap_err();
    let is_unknown = matches!(err, ParseError::UnknownToken { .. });
    assert!(is_unknown);
}

#[test]
fn parse_expr_n_count_bad_length_prefix() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  N abc:2\n    T 1:x\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_n_count_non_utf8() {
    let err = Mrk::from_bytes_component(b"mrk1\nC 5:greet\n  N 5:\xc3\x28\x80\x80\x80\n    T 1:x\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_n_count_unparseable() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  N 1:x\n    T 1:x\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_expr_n_node_expr_fails() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  N 1:1\n    Z 1:x\n").unwrap_err();
    let is_unknown = matches!(err, ParseError::UnknownToken { .. });
    assert!(is_unknown);
}

// =================================================================
// Loop truncation: scan_line returns None inside parse_expr loops.
// =================================================================

#[test]
fn parse_expr_s_loop_truncated() {
    // S declares 1 item but no item line follows.
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  S 1:1\n").unwrap_err();
    assert_eq!(err, ParseError::UnexpectedEof);
}

#[test]
fn parse_expr_m_value_loop_truncated() {
    // M declares 1 arm but no value line follows.
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  M 1:k 1:1\n").unwrap_err();
    assert_eq!(err, ParseError::UnexpectedEof);
}

#[test]
fn parse_expr_m_arm_loop_truncated() {
    // M declares 1 arm; value line is present but no arm expression follows.
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  M 1:k 1:1\n    1:x\n").unwrap_err();
    assert_eq!(err, ParseError::UnexpectedEof);
}

#[test]
fn parse_expr_m_default_truncated() {
    // M declares 1 arm; value+arm present but no default follows.
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  M 1:k 1:1\n    1:x\n    P 1:k\n").unwrap_err();
    assert_eq!(err, ParseError::UnexpectedEof);
}

#[test]
fn parse_expr_i_then_truncated() {
    // I has condition but no then line follows.
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  I 1:c\n").unwrap_err();
    assert_eq!(err, ParseError::UnexpectedEof);
}

#[test]
fn parse_expr_i_else_truncated() {
    // I has condition and then but no else follows.
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  I 1:c\n    P 1:k\n").unwrap_err();
    assert_eq!(err, ParseError::UnexpectedEof);
}

#[test]
fn parse_expr_o_then_truncated() {
    // O has condition but no then line follows.
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  O 1:c\n").unwrap_err();
    assert_eq!(err, ParseError::UnexpectedEof);
}

#[test]
fn parse_expr_f_body_truncated() {
    // F has input but no body line follows.
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  F 1:i\n").unwrap_err();
    assert_eq!(err, ParseError::UnexpectedEof);
}

#[test]
fn parse_expr_w_attr_loop_truncated() {
    // W declares 1 attr but no attr line follows.
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  W 1:p 1:1 0:\n").unwrap_err();
    assert_eq!(err, ParseError::UnexpectedEof);
}

#[test]
fn parse_expr_w_body_loop_truncated() {
    // W declares 1 body but no body line follows.
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  W 1:p 1:0 1:1\n").unwrap_err();
    assert_eq!(err, ParseError::UnexpectedEof);
}

#[test]
fn parse_attr_a_key_bad_length_prefix() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  W 1:p 1:1 0:\n    A abc:k 1:v\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_attr_a_key_non_utf8() {
    let err = Mrk::from_bytes_component(b"mrk1\nC 5:greet\n  W 1:p 1:1 0:\n    A 5:\xc3\x28\x80\x80\x80 1:v\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_attr_a_value_bad_length_prefix() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  W 1:p 1:1 0:\n    A 1:k abc:v\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_attr_a_value_non_utf8() {
    let err = Mrk::from_bytes_component(b"mrk1\nC 5:greet\n  W 1:p 1:1 0:\n    A 1:k 5:\xc3\x28\x80\x80\x80\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_attr_b_key_bad_length_prefix() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  W 1:p 1:1 0:\n    B abc:k\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_attr_b_key_non_utf8() {
    let err = Mrk::from_bytes_component(b"mrk1\nC 5:greet\n  W 1:p 1:1 0:\n    B 5:\xc3\x28\x80\x80\x80\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_node_t_text_bad_length_prefix() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  N 1:1\n    T abc:x\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_node_t_text_non_utf8() {
    let err = Mrk::from_bytes_component(b"mrk1\nC 5:greet\n  N 1:1\n    T 5:\xc3\x28\x80\x80\x80\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_node_r_text_bad_length_prefix() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  N 1:1\n    R abc:x\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_node_r_text_non_utf8() {
    let err = Mrk::from_bytes_component(b"mrk1\nC 5:greet\n  N 1:1\n    R 5:\xc3\x28\x80\x80\x80\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_node_e_name_bad_length_prefix() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  N 1:1\n    E abc:p\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn parse_node_e_name_non_utf8() {
    let err = Mrk::from_bytes_component(b"mrk1\nC 5:greet\n  N 1:1\n    E 5:\xc3\x28\x80\x80\x80\n").unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn read_lp_value_truncated_payload() {
    // `name_b` length prefix declares more bytes than the line contains.
    let err = Mrk::from_string_component(
        "mrk1\nC 1:c\n  M 4:role 1:1\n    10:abcxyz\n    P 1:k\n",
    )
    .unwrap_err();
    let is_trunc = matches!(err, ParseError::TruncatedPayload { .. });
    assert!(is_trunc);
}

#[test]
fn component_expr_n_node_wrong_indent() {
    let err = Mrk::from_string_component(
        "mrk1\nC 5:greet\n  N 1:1\n T 1:x\n",
    )
    .unwrap_err();
    let is_bad = matches!(err, ParseError::BadNesting { .. });
    assert!(is_bad);
}

#[test]
fn component_expr_n_truncated() {
    let err =
        Mrk::from_string_component("mrk1\nC 5:greet\n  N 1:1\n").unwrap_err();
    assert_eq!(err, ParseError::UnexpectedEof);
}

#[test]
fn component_expr_n_node_invalid_kind() {
    let err = Mrk::from_string_component(
        "mrk1\nC 5:greet\n  N 1:1\n    Z 1:x\n",
    )
    .unwrap_err();
    let is_unknown = matches!(err, ParseError::UnknownToken { .. });
    assert!(is_unknown);
}

#[test]
fn component_expr_unknown_kind() {
    let err = Mrk::from_string_component("mrk1\nC 5:greet\n  Z 1:x\n").unwrap_err();
    eprintln!("got: {:?}", err);
    let is_unknown = matches!(err, ParseError::UnknownToken { .. });
    assert!(is_unknown);
}

#[test]
fn component_attr_unknown_kind() {
    let err = Mrk::from_string_component(
        "mrk1\nC 5:greet\n  W 1:p 1:1 0:\n    Z 1:k\n",
    )
    .unwrap_err();
    eprintln!("got: {:?}", err);
    let is_unknown = matches!(err, ParseError::UnknownToken { .. });
    assert!(is_unknown);
}

// =================================================================
// D-wrapped-attr error paths in parse_wrapped_attr.
// =================================================================

#[test]
fn wrap_d_attr_bad_nesting() {
    // D at indent 4 → expression must be at indent 6.
    // Placing it at indent 2 triggers BadNesting.
    let err = Mrk::from_string_component(
        "mrk1\nC 5:greet\n  W 3:div 1:1 0:\n    D 5:class\n  P name\n",
    )
    .unwrap_err();
    let is_bad = matches!(err, ParseError::BadNesting { .. });
    assert!(is_bad);
}

// =================================================================
// Round-trip coverage for the encode_node Raw / Element variants.
// =================================================================

#[cfg(feature = "html")]
#[test]
fn component_literal_children_with_raw_and_element() {
    use crate::components::Expr;
    use crate::html::Raw;

    // Manually construct a LiteralChildren with Text, Raw, and Element
    // so all three `encode_node` arms run on round-trip.
    let raw_node = Raw::str("<em>x</em>");
    let el_node = Node::Element(el("p"));
    let text_node = Node::Text("hi".into());
    let c = Component {
        name: Cow::Borrowed("lc"),
        expr: Expr::LiteralChildren(vec![text_node, raw_node, el_node]),
    };
    let s = Mrk::to_string_component(&c);
    eprintln!("encoded:\n{}", s);
    let bytes = Mrk::bytes_component(&c);
    let back = Mrk::from_bytes_component(&bytes).expect("decode");
    assert_eq!(c, back);
}

#[cfg(not(feature = "html"))]
#[test]
fn component_literal_children_with_text_and_element() {
    use crate::components::Expr;

    let el_node = Node::Element(el("p"));
    let text_node = Node::Text("hi".into());
    let c = Component {
        name: Cow::Borrowed("lc"),
        expr: Expr::LiteralChildren(vec![text_node, el_node]),
    };
    let bytes = Mrk::bytes_component(&c);
    let back = Mrk::from_bytes_component(&bytes).expect("decode");
    assert_eq!(c, back);
}

#[test]
fn component_to_string_round_trip() {
    use crate::components::Expr;

    let c = Component {
        name: Cow::Borrowed("greet"),
        expr: Expr::Prop("name".into()),
    };
    let s = Mrk::to_string_component(&c);
    assert!(s.starts_with("mrk1\n"));
    let back = Mrk::from_string_component(&s).expect("decode");
    assert_eq!(c, back);
}

#[cfg(feature = "html")]
#[test]
fn round_trip_n_expr_with_raw_and_element() {
    use crate::components::Expr;
    use crate::html::Raw;

    // Empty Wrap containing a LiteralChildren with all node flavours.
    let body = Expr::LiteralChildren(vec![
        Node::Text("t".into()),
        Raw::str("<br/>"),
        Node::Element(el("span")),
    ]);
    let c = Component {
        name: Cow::Borrowed("wrap_with_n"),
        expr: Expr::Wrap {
        name: "div".into(),
        attrs: vec![],
        body: vec![Box::new(body)],
    },
    };
    let bytes = Mrk::bytes_component(&c);
    let back = Mrk::from_bytes_component(&bytes).expect("decode");
    assert_eq!(c, back);
}

#[test]
fn wrap_d_attr_field_payload_error() {
    let err = Mrk::from_string_component(
        "mrk1\nC 5:greet\n  W 3:div 1:1 0:\n    D bad\n",
    )
    .unwrap_err();
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn wrap_d_attr_bytes_to_string_non_utf8() {
    let mut ir = Vec::new();
    ir.extend_from_slice(b"mrk1\nC 4:test\n  W 3:div 1:1 0:\n    D 3:");
    ir.extend_from_slice(&[0xff, 0xfe, 0xfd]);
    ir.push(b'\n');
    let err = Mrk::from_bytes_component(&ir).unwrap_err();
    eprintln!("non_utf8 got: {:?}", err);
    let is_bad = matches!(err, ParseError::BadLengthPrefix { .. });
    assert!(is_bad);
}

#[test]
fn wrap_d_attr_eof_after_key() {
    let err = Mrk::from_string_component(
        "mrk1\nC 4:test\n  W 3:div 1:1 0:\n    D 3:foo\n",
    )
    .unwrap_err();
    eprintln!("got: {:?}", err);
    let is_eof = matches!(err, ParseError::UnexpectedEof);
    assert!(is_eof);
}

#[test]
fn wrap_d_attr_expr_parse_error() {
    let err = Mrk::from_string_component(
        "mrk1\nC 4:test\n  W 3:div 1:1 0:\n    D 3:foo\n      Z 3:bar\n",
    )
    .unwrap_err();
    let is_unknown = matches!(err, ParseError::UnknownToken { .. });
    assert!(is_unknown);
}

#[test]
fn parse_attr_unknown_token() {
    let mut parser = crate::ir::Parser::new(b"");
    let line = crate::ir::parse_line(b"Z 3:foo");
    let result = parser.parse_attr(line, 1);
    assert!(matches!(result, Err(ParseError::UnknownToken { .. })));
}
