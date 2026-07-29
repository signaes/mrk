/// Builds a `Vec<Node>` for use with `.children(...)`. Accepts `&'static str`,
/// `String`, and any element value (from `el(...)` or a factory).
///
/// # Example
///
/// ```
/// use mrk::*;
///
/// let tree = el("p").children(nodes!["Hello, ", "world"]);
/// assert_eq!(tree.children.len(), 2);
/// ```
#[macro_export]
macro_rules! nodes {
    () => {
        ::std::vec::Vec::<$crate::Node>::new()
    };
    ($($child:expr),+ $(,)?) => {{
        #[allow(clippy::vec_init_then_push)]
        {
            let mut v: ::std::vec::Vec<$crate::Node> = ::std::vec::Vec::new();
            $(
                v.push(<_ as ::std::convert::Into<$crate::Node>>::into($child));
            )+
            v
        }
    }};
}

/// Implementation detail of [`html!`](crate::html) and
/// [`svg!`](crate::svg). Expands one element (tag + attributes +
/// children) into its typed wrapper from the given factory module
/// (`$crate::html` or `$crate::svg`). Not public API.
///
/// Attribute keys may contain dashes (`data-value`, `aria-label`): each
/// key is matched as `$key $(- $krest)*` and rebuilt into a string with
/// the builtin `concat!`/`stringify!` macros, so no external crate is
/// needed. Attributes without a value (`disabled`, `checked`) are
/// emitted as boolean attributes.
#[doc(hidden)]
#[macro_export]
macro_rules! __mrk_markup {
    ($module:path, $tag:ident ( $($attrs:tt)* ) { $($children:tt)* }) => {{
        use $module as m;
        let e = m::$tag();
        let e = $crate::__mrk_markup_attrs!(e, $($attrs)*);
        let e = e.children({
            #[allow(unused_mut)] // empty children blocks emit no pushes
            let mut v: ::std::vec::Vec<$crate::Node> = ::std::vec::Vec::new();
            $crate::__mrk_markup_children!(v, $module, $($children)*);
            v
        });
        e
    }};
    ($module:path, $tag:ident ( $($attrs:tt)* )) => {{
        use $module as m;
        let e = m::$tag();
        let e = $crate::__mrk_markup_attrs!(e, $($attrs)*);
        e
    }};
}

/// Implementation detail of [`html!`](crate::html) and
/// [`svg!`](crate::svg). Munches the attribute token list, building up
/// the element’s attribute list. Distinguishes key-value attributes
/// (`key = "value"`) from boolean attributes (`key` alone) using arm
/// order: the key-value arm is tried first.
#[doc(hidden)]
#[macro_export]
macro_rules! __mrk_markup_attrs {
    ($e:ident,) => { $e };
    ($e:ident, $key:ident = $val:literal $($rest:tt)*) => {{
        let $e = $e.attr(::std::stringify!($key), $val);
        $crate::__mrk_markup_attrs!($e, $($rest)*)
    }};
    ($e:ident, $key:ident $(- $krest:ident)* = $val:literal $($rest:tt)*) => {{
        let key = ::std::concat!(
            ::std::stringify!($key) $(, "-", ::std::stringify!($krest))*
        );
        let $e = $e.attr(key, $val);
        $crate::__mrk_markup_attrs!($e, $($rest)*)
    }};
    ($e:ident, $key:ident $(- $krest:ident)* $($rest:tt)*) => {{
        let key = ::std::concat!(
            ::std::stringify!($key) $(, "-", ::std::stringify!($krest))*
        );
        let $e = $e.bool_attr(key);
        $crate::__mrk_markup_attrs!($e, $($rest)*)
    }};
}

/// Implementation detail of [`html!`](crate::html) and
/// [`svg!`](crate::svg). Munches a child token list, pushing each child
/// into the `Vec<Node>` accumulator `$v`. Not public API.
///
/// Arm order matters: braced element children must be tried before the
/// bare (leaf/void) element arm, which in turn precedes the text-literal
/// arm.
#[doc(hidden)]
#[macro_export]
macro_rules! __mrk_markup_children {
    ($v:ident, $module:path,) => {};
    ($v:ident, $module:path, $tag:ident ( $($attrs:tt)* ) { $($children:tt)* } $($rest:tt)*) => {{
        $v.push($crate::Node::from(
            $crate::__mrk_markup!($module, $tag ( $($attrs)* ) { $($children)* }),
        ));
        $crate::__mrk_markup_children!($v, $module, $($rest)*);
    }};
    ($v:ident, $module:path, $tag:ident ( $($attrs:tt)* ) $($rest:tt)*) => {{
        $v.push($crate::Node::from(
            $crate::__mrk_markup!($module, $tag ( $($attrs)* )),
        ));
        $crate::__mrk_markup_children!($v, $module, $($rest)*);
    }};
    ($v:ident, $module:path, $text:literal $($rest:tt)*) => {{
        $v.push($crate::Node::from($text));
        $crate::__mrk_markup_children!($v, $module, $($rest)*);
    }};
}

/// Builds an HTML [`Element`](crate::Element) tree with a declarative,
/// markup-like syntax. Available with the `html` feature.
///
/// Tag names resolve to the [`html`](crate::html) module's factory
/// functions, so an unknown tag is a compile error. The braces may be
/// omitted for leaf/void elements (`img(src="x.png")`, `br()`).
/// Attribute keys may contain dashes (`data-value`, `aria-label`);
/// values must be string literals.
///
/// The macro evaluates to an [`Element`](crate::Element), so the result
/// composes with the rest of the API (`.render()`, `.attrs(...)`,
/// [`nodes!`], ...).
///
/// # Example
///
/// ```
/// use mrk::*;
///
/// let tree = html! { div(class="a b c" id="container") {
///     span(class="text") { "ok" }
///     div() { "sibling" }
///     div(data-value="true") { ul() { li() { "1" } li(class="second") { "2" } } }
/// } };
///
/// assert_eq!(
///     tree.render(),
///     r#"<div class="a b c" id="container"><span class="text">ok</span><div>sibling</div><div data-value="true"><ul><li>1</li><li class="second">2</li></ul></div></div>"#
/// );
/// ```
#[cfg(feature = "html")]
#[macro_export]
macro_rules! html {
    ($tag:ident ( $($attrs:tt)* ) { $($children:tt)* }) => {
        $crate::Element::from(
            $crate::__mrk_markup!($crate::html, $tag ( $($attrs)* ) { $($children)* }),
        )
    };
    ($tag:ident ( $($attrs:tt)* )) => {
        $crate::Element::from(
            $crate::__mrk_markup!($crate::html, $tag ( $($attrs)* )),
        )
    };
}

/// Builds an SVG [`Element`](crate::Element) tree with a declarative,
/// markup-like syntax. Available with the `svg` feature.
///
/// Tag names resolve to the [`svg`](crate::svg) module's factory
/// functions (snake_case names such as `linear_gradient`, `font_face`),
/// so an unknown tag is a compile error. Attribute names are written
/// verbatim — camelCase idents like `viewBox` are valid as-is, and
/// dashed names like `stroke-width` are supported. Values must be
/// string literals. The braces may be omitted for leaf elements.
///
/// The macro evaluates to an [`Element`](crate::Element).
///
/// # Example
///
/// ```
/// use mrk::*;
///
/// let icon = svg! { svg(viewBox="0 0 10 10") {
///     circle(cx="5" cy="5" r="4")
///     line(x1="0" y1="0" x2="10" y2="10" stroke-width="1")
/// } };
///
/// assert_eq!(
///     icon.render(),
///     r#"<svg viewBox="0 0 10 10"><circle cx="5" cy="5" r="4"></circle><line x1="0" y1="0" x2="10" y2="10" stroke-width="1"></line></svg>"#
/// );
/// ```
#[cfg(feature = "svg")]
#[macro_export]
macro_rules! svg {
    ($tag:ident ( $($attrs:tt)* ) { $($children:tt)* }) => {
        $crate::Element::from(
            $crate::__mrk_markup!($crate::svg, $tag ( $($attrs)* ) { $($children)* }),
        )
    };
    ($tag:ident ( $($attrs:tt)* )) => {
        $crate::Element::from(
            $crate::__mrk_markup!($crate::svg, $tag ( $($attrs)* )),
        )
    };
}

#[cfg(all(test, feature = "html"))]
mod html_macro_tests {
    use crate::{Node, Renderable};

    #[test]
    fn builds_nested_tree_with_attributes() {
        let tree = crate::html! { div(class="a b c" id="container") {
            span(class="text") { "ok" }
            div() { "sibling" }
            div(data-value="true") { ul() { li() { "1" } li(class="second") { "2" } } }
        } };

        assert_eq!(tree.name, "div");
        assert_eq!(tree.attributes.len(), 2);
        assert_eq!(tree.attributes[0].key, "class");
        assert_eq!(tree.attributes[1].key, "id");
        assert_eq!(tree.children.len(), 3);
        assert_eq!(
            tree.render(),
            r#"<div class="a b c" id="container"><span class="text">ok</span><div>sibling</div><div data-value="true"><ul><li>1</li><li class="second">2</li></ul></div></div>"#
        );
    }

    #[test]
    fn dashed_attribute_keys() {
        let tree = crate::html! { button(aria-label="close" data-role="dismiss") { "x" } };
        assert_eq!(
            tree.render(),
            r#"<button aria-label="close" data-role="dismiss">x</button>"#
        );
    }

    #[test]
    fn void_elements_without_braces() {
        assert_eq!(crate::html! { img(src="x.png") }.render(), r#"<img src="x.png">"#);
        assert_eq!(crate::html! { br() }.render(), "<br>");
    }

    #[test]
    fn empty_braces_are_allowed() {
        assert_eq!(crate::html! { div() {} }.render(), "<div></div>");
    }

    #[test]
    fn text_is_escaped() {
        let tree = crate::html! { p() { "a < b & c" } };
        assert_eq!(tree.render(), "<p>a &lt; b &amp; c</p>");
    }

    #[test]
    fn mixed_text_and_element_children() {
        let tree = crate::html! { p() { "hello " strong() { "world" } "!" } };
        assert_eq!(tree.children.len(), 3);
        // Iterate over a mix of children so each matches! line is exercised
        // with both true and false: Text and Element.
        let mut results: Vec<(bool, bool)> = Vec::new();
        for child in &tree.children {
            let is_text = matches!(child, Node::Text(_));
            let is_element = matches!(child, Node::Element(_));
            results.push((is_text, is_element));
        }
        assert_eq!(results, vec![(true, false), (false, true), (true, false)]);
        assert_eq!(tree.render(), "<p>hello <strong>world</strong>!</p>");
    }

    #[test]
    fn mixed_children_other_types_exercise_false_branches() {
        // A Raw child (built via the Element API since the html! macro
        // doesn't yet accept nodes! as a child) hits the false branch
        // of the matches! in the previous test for both Text and Element.
        let tree = crate::el("p").children(crate::nodes![crate::html::Raw::str("x")]);
        assert_eq!(tree.children.len(), 1);
        let first_is_text = matches!(tree.children[0], Node::Text(_));
        let first_is_element = matches!(tree.children[0], Node::Element(_));
        assert!(!first_is_text);
        assert!(!first_is_element);
    }

    #[test]
    fn result_composes_with_element_api() {
        let page = crate::el("section").children(crate::nodes![
            crate::html! { h1() { "Title" } },
            "tail",
        ]);
        assert_eq!(page.render(), "<section><h1>Title</h1>tail</section>");
    }

    #[test]
    fn boolean_attribute_renders_without_value() {
        // `disabled` with no `=` is a boolean attribute; renders as just
        // the key, e.g. `<input disabled>`.
        let tree = crate::html! { input(type="checkbox" disabled) {} };
        assert_eq!(tree.render(), r#"<input type="checkbox" disabled>"#);
    }

    #[test]
    fn boolean_attribute_mixed_with_key_value() {
        // Boolean attributes can be interleaved with key-value ones.
        let tree = crate::html! {
            input(type="text" required disabled class="a" autofocus)
        };
        assert_eq!(
            tree.render(),
            r#"<input type="text" required disabled class="a" autofocus>"#
        );
    }

    #[test]
    fn boolean_attribute_inside_children() {
        // Boolean attributes work on elements with children too.
        let tree = crate::html! {
            button(disabled class="btn") { "Can't click" }
        };
        assert_eq!(
            tree.render(),
            r#"<button disabled class="btn">Can't click</button>"#
        );
    }

    #[test]
    fn boolean_attribute_with_dashed_key() {
        // Dashed boolean attributes work too, e.g. SVG `focusable`.
        let tree = crate::html! { input(focusable="false" hidden) {} };
        assert_eq!(tree.render(), r#"<input focusable="false" hidden>"#);
    }
}

#[cfg(all(test, feature = "svg"))]
mod svg_macro_tests {
    use crate::Renderable;

    #[test]
    fn builds_svg_tree() {
        let icon = crate::svg! { svg(viewBox="0 0 10 10") {
            circle(cx="5" cy="5" r="4")
            line(x1="0" y1="0" x2="10" y2="10" stroke-width="1")
        } };

        assert_eq!(icon.name, "svg");
        assert_eq!(icon.children.len(), 2);
        assert_eq!(
            icon.render(),
            r#"<svg viewBox="0 0 10 10"><circle cx="5" cy="5" r="4"></circle><line x1="0" y1="0" x2="10" y2="10" stroke-width="1"></line></svg>"#
        );
    }

    #[test]
    fn svg_text_child() {
        let label = crate::svg! { text(x="1" y="2") { "hi" } };
        assert_eq!(label.render(), r#"<text x="1" y="2">hi</text>"#);
    }

    #[test]
    fn snake_case_factory_names() {
        let defs = crate::svg! { defs() {
            linear_gradient(id="g") { stop(offset="0" stop-color="red") }
        } };
        assert_eq!(
            defs.render(),
            r#"<defs><linearGradient id="g"><stop offset="0" stop-color="red"></stop></linearGradient></defs>"#
        );
    }

    #[test]
    fn svg_result_is_element() {
        let el = crate::svg! { rect(width="10" height="20") };
        assert_eq!(el.name, "rect");
        assert_eq!(el.attributes.len(), 2);
        assert_eq!(el.render(), r#"<rect width="10" height="20"></rect>"#);
    }

    #[test]
    fn svg_boolean_attributes_render_without_value() {
        // SVG attributes such as `noValidate` work as boolean flags.
        let tree = crate::svg! { circle(cx="5" cy="5" r="4" noValidate) };
        assert_eq!(tree.render(), r#"<circle cx="5" cy="5" r="4" noValidate></circle>"#);
    }
}
