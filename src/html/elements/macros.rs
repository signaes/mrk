//! Macro for generating typed HTML element wrappers.
//!
//! Each typed element wraps an [`Element`] and adds element-specific
//! attribute setters as methods. The macro generates:
//!
//! - A struct `pub struct $name(pub Element)`
//! - `new()` constructor
//! - `attrs()`, `children()`, `render()` builder methods
//! - One method per attribute listed in the invocation
//! - `From<$name> for Element` and `From<$name> for Node` impls
//! - `Renderable for $name` impl
//!
//! Method names map to HTML attribute names via the exhaustive [`attr_name`]
//! lookup table. Every attribute used in `define_html_element!` invocations
//! must have a corresponding match arm.
//!
//! [`Element`]: crate::element::Element

macro_rules! define_html_element {
    ($name:ident, $tag:literal $(, $($method:ident($doc:literal)),*)?) => {
        /// Typed HTML element wrapper.
        #[derive(Debug)]
        pub struct $name(pub crate::element::Element);

        impl $name {
            /// Create a new empty element with the matching tag.
            pub fn new() -> Self {
                $name(crate::element::el($tag))
            }

            /// Replace the element's attributes.
            pub fn attrs(mut self, attrs: Vec<crate::attributes::Attribute>) -> Self {
                self.0 = self.0.attrs(attrs);
                self
            }

            /// Replace the element's children.
            pub fn children(mut self, children: Vec<crate::node::Node>) -> Self {
                self.0 = self.0.children(children);
                self
            }

            /// Render to HTML string.
            pub fn render(&self) -> String {
                use crate::renderable::Renderable;
                self.0.render()
            }

            $(
                $(
                    #[doc = $doc]
                    pub fn $method(self, value: &'static str) -> Self {
                        let attr_name = crate::html::elements::macros::attr_name(stringify!($method));
                        $name(self.0.attrs(vec![
                            crate::attributes::attr(attr_name).value(value)
                        ]))
                    }
                )*
            )?
        }

        impl From<$name> for crate::element::Element {
            fn from(e: $name) -> crate::element::Element {
                e.0
            }
        }

        impl From<$name> for crate::node::Node {
            fn from(e: $name) -> crate::node::Node {
                crate::node::Node::Element(e.0)
            }
        }

        impl crate::renderable::Renderable for $name {
            fn render(&self) -> String {
                self.0.render()
            }
        }
    };
}

pub(crate) use define_html_element;

pub(crate) fn attr_name(ident: &str) -> &'static str {
    match ident {
        "abbr" => "abbr",
        "accept" => "accept",
        "accept_charset" => "accept-charset",
        "action" => "action",
        "allow" => "allow",
        "allowfullscreen" => "allowfullscreen",
        "allowpaymentrequest" => "allowpaymentrequest",
        "alt" => "alt",
        "as_attr" => "as",
        "async_attr" => "async",
        "autocomplete" => "autocomplete",
        "autofocus" => "autofocus",
        "autoplay" => "autoplay",
        "blocking" => "blocking",
        "charset" => "charset",
        "checked" => "checked",
        "cite" => "cite",
        "color" => "color",
        "cols" => "cols",
        "colspan" => "colspan",
        "content" => "content",
        "controls" => "controls",
        "coords" => "coords",
        "crossorigin" => "crossorigin",
        "credentialless" => "credentialless",
        "csp" => "csp",
        "data" => "data",
        "datalist" => "datalist",
        "datetime" => "datetime",
        "decoding" => "decoding",
        "default_attr" => "default",
        "defer_attr" => "defer",
        "disabled" => "disabled",
        "download" => "download",
        "enctype" => "enctype",
        "fetchpriority" => "fetchpriority",
        "for_attr" => "for",
        "form" => "form",
        "form_attr" => "form",
        "formaction" => "formaction",
        "formenctype" => "formenctype",
        "formmethod" => "formmethod",
        "formnovalidate" => "formnovalidate",
        "formtarget" => "formtarget",
        "headers" => "headers",
        "height" => "height",
        "high" => "high",
        "href" => "href",
        "hreflang" => "hreflang",
        "http_equiv" => "http-equiv",
        "imagesizes" => "imagesizes",
        "imagesrcset" => "imagesrcset",
        "integrity" => "integrity",
        "ismap" => "ismap",
        "kind" => "kind",
        "label" => "label",
        "list" => "list",
        "loading" => "loading",
        "longdesc" => "longdesc",
        "loop_attr" => "loop",
        "low" => "low",
        "max" => "max",
        "maxlength" => "maxlength",
        "media" => "media",
        "method" => "method",
        "min" => "min",
        "minlength" => "minlength",
        "multiple" => "multiple",
        "muted" => "muted",
        "name" => "name",
        "novalidate" => "novalidate",
        "nomodule" => "nomodule",
        "nonce" => "nonce",
        "open_attr" => "open",
        "optimum" => "optimum",
        "pattern" => "pattern",
        "ping" => "ping",
        "placeholder" => "placeholder",
        "playsinline" => "playsinline",
        "popovertarget" => "popovertarget",
        "popovertargetaction" => "popovertargetaction",
        "poster" => "poster",
        "preload" => "preload",
        "preserve_aspect_ratio" => "preserve-aspect-ratio",
        "readonly" => "readonly",
        "referrerpolicy" => "referrerpolicy",
        "rel" => "rel",
        "required" => "required",
        "reversed" => "reversed",
        "rowspan" => "rowspan",
        "rows" => "rows",
        "sandbox" => "sandbox",
        "scope" => "scope",
        "selected" => "selected",
        "shape" => "shape",
        "size" => "size",
        "sizes" => "sizes",
        "span" => "span",
        "spellcheck" => "spellcheck",
        "src" => "src",
        "srcdoc" => "srcdoc",
        "srcset" => "srcset",
        "srclang" => "srclang",
        "start" => "start",
        "step" => "step",
        "tabindex" => "tabindex",
        "target" => "target",
        "title" => "title",
        "title_attr" => "title",
        "type_attr" => "type",
        "typemustmatch" => "typemustmatch",
        "usemap" => "usemap",
        "value" => "value",
        "version" => "version",
        "view_box" => "view-box",
        "width" => "width",
        "wrap" => "wrap",
        "xmlns" => "xmlns",
        other => panic!("attr_name: unmapped identifier '{other}'"),
    }
}

macro_rules! factory {
    ($(#[$meta:meta])* $fn_name:ident, $type:ident) => {
        $(#[$meta])*
        pub fn $fn_name() -> $type {
            $type::new()
        }
    };
}

pub(crate) use factory;

#[cfg(test)]
mod tests {
    use crate::attributes::AttributeType;
    use crate::element::Element;
    use crate::node::Node;
    use crate::renderable::Renderable;

    super::define_html_element!(TestDiv, "div");
    super::define_html_element!(TestAnchor, "a", href("Hyperlink URL."), target("Frame target."));
    super::define_html_element!(TestInput, "input", type_attr("Input type."), name("Field name."));
    super::define_html_element!(TestSvg, "svg", view_box("SVG viewBox."), xmlns("XML namespace."));

    super::factory!(test_div, TestDiv);
    super::factory!(test_anchor, TestAnchor);

    #[test]
    fn new_creates_correct_tag() {
        assert_eq!(TestDiv::new().0.name, "div");
        assert_eq!(TestAnchor::new().0.name, "a");
        assert_eq!(TestInput::new().0.name, "input");
    }

    #[test]
    fn attrs_replaces_attributes() {
        let el = TestDiv::new()
            .attrs(vec![crate::attributes::attr("class").value("box")]);
        assert_eq!(el.0.attributes.len(), 1);
        assert_eq!(el.0.attributes[0].key, "class");
    }

    #[test]
    fn children_replaces_children() {
        let el = TestDiv::new().children(vec!["hello".into()]);
        assert_eq!(el.0.children.len(), 1);
    }

    #[test]
    fn attribute_setter_normal_name() {
        let el = TestAnchor::new().href("/");
        assert_eq!(el.0.attributes.len(), 1);
        assert_eq!(el.0.attributes[0].key, "href");
        assert!(matches!(el.0.attributes[0].attr, AttributeType::KeyValue("href", "/")));
    }

    #[test]
    fn attribute_setter_attr_suffix() {
        let el = TestInput::new().type_attr("text");
        assert_eq!(el.0.attributes.len(), 1);
        assert_eq!(el.0.attributes[0].key, "type");
        assert!(matches!(el.0.attributes[0].attr, AttributeType::KeyValue("type", "text")));
    }

    #[test]
    fn attribute_setter_underscore_to_dash() {
        let el = TestSvg::new().view_box("0 0 100 100");
        assert_eq!(el.0.attributes.len(), 1);
        assert_eq!(el.0.attributes[0].key, "view-box");
        assert!(matches!(el.0.attributes[0].attr, AttributeType::KeyValue("view-box", "0 0 100 100")));
    }

    #[test]
    fn attribute_setter_plain_name_no_conversion() {
        let el = TestAnchor::new().target("_blank");
        assert_eq!(el.0.attributes[0].key, "target");
    }

    #[test]
    fn attribute_setter_without_attr_suffix() {
        let el = TestInput::new().name("q");
        assert_eq!(el.0.attributes[0].key, "name");
    }

    #[test]
    fn attribute_setter_without_underscore() {
        let el = TestSvg::new().xmlns("http://www.w3.org/2000/svg");
        assert_eq!(el.0.attributes[0].key, "xmlns");
    }

    #[test]
    fn from_typed_into_element() {
        let typed = TestDiv::new();
        let elem: Element = typed.into();
        assert_eq!(elem.name, "div");
    }

    #[test]
    fn from_typed_into_node() {
        let typed = TestAnchor::new();
        let node: Node = typed.into();
        assert!(matches!(node, Node::Element(e) if e.name == "a"));
    }

    #[test]
    fn render_delegates_to_inner() {
        let el = TestDiv::new().children(vec!["hi".into()]);
        assert_eq!(Renderable::render(&el), "<div>hi</div>");
    }

    #[test]
    fn chained_setters_last_wins() {
        let el = TestAnchor::new()
            .href("/page")
            .target("_self");
        assert_eq!(el.0.attributes.len(), 1);
        assert_eq!(el.0.attributes[0].key, "target");
        assert_eq!(Renderable::render(&el), r#"<a target="_self"></a>"#);
    }

    #[test]
    fn factory_returns_correct_type() {
        let d = test_div();
        assert_eq!(d.0.name, "div");
        let a = test_anchor();
        assert_eq!(a.0.name, "a");
    }

    #[test]
    fn debug_format() {
        let _ = format!("{:?}", TestDiv::new());
    }

    #[test]
    #[should_panic(expected = "unmapped identifier")]
    fn attr_name_unmapped_identifier_panics() {
        super::attr_name("totally_unknown_identifier");
    }
}
