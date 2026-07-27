//! Code-generation helpers shared by the `html` and `svg` macro modules.
//!
//! Each helper is a `macro_rules!` declaration used by the per-feature
//! wrapper macros in `src/html/elements/macros.rs` and
//! `src/svg/elements/macros.rs`. The helper bodies expand into the
//! per-element files (`html/elements/document.rs`, `svg/elements/shapes.rs`,
//! etc.) where their runtime coverage is measured at 100%.
//!
//! This file itself cannot reach 100% line coverage under
//! `cargo-llvm-cov`: declarative macro arms are expansion targets,
//! not directly-executable code paths at the source line level.
//! The exclusion is therefore accepted and documented in
//! `scripts/coverage.sh` via `--ignore-filename-regex`. The script
//! is the canonical way to run coverage for this project; running
//! `cargo llvm-cov --all-features` directly will still report
//! `shared/macros.rs` as low coverage — that is a known limitation
//! of declarative macros, not a defect in the test suite.

/// Generates `pub struct $name(pub Element)` and a `Default` impl.
macro_rules! __define_struct {
    ($name:ident) => {
        /// Typed element wrapper.
        #[derive(Debug)]
        pub struct $name(pub crate::element::Element);

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}
pub(crate) use __define_struct;

/// Generates the `new()` constructor.
macro_rules! __new_method {
    ($tag:literal) => {
        /// Create a new empty element with the matching tag.
        pub fn new() -> Self {
            Self(crate::element::el($tag))
        }
    };
}
pub(crate) use __new_method;

/// Generates `attrs()`, `children()`, and `render()`.
macro_rules! __builder_methods {
    () => {
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
        /// Render to a string.
        pub fn render(&self) -> String {
            use crate::renderable::Renderable;
            self.0.render()
        }
    };
}
pub(crate) use __builder_methods;

/// Generates one element-specific attribute setter.
macro_rules! __emitted_custom_method {
    ($method:ident, $doc:literal, $attr_name_path:path) => {
        #[doc = $doc]
        pub fn $method(self, value: &'static str) -> Self {
            let attr_name = $attr_name_path(stringify!($method));
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr(attr_name).value(value)]),
            )
        }
    };
}
pub(crate) use __emitted_custom_method;

/// Generates `From` and `Renderable` impls.
macro_rules! __from_impls {
    ($name:ident) => {
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
pub(crate) use __from_impls;

/// Common HTML/SVG global attribute setters.
macro_rules! __common_globals_methods {
    () => {
        /// Unique identifier for the element.
        pub fn id(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("id").value(value)]),
            )
        }
        /// Space-separated list of CSS class names.
        pub fn class(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("class").value(value)]),
            )
        }
        /// Inline CSS styles.
        pub fn style(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("style").value(value)]),
            )
        }
        /// Tab navigation order (global `tabindex`).
        pub fn tabindex_global(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("tabindex").value(value)]),
            )
        }
        /// BCP 47 language tag (global `lang`).
        pub fn lang_global(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("lang").value(value)]),
            )
        }
        /// Text directionality.
        pub fn dir(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("dir").value(value)]),
            )
        }
        /// Hidden flag.
        pub fn hidden(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("hidden").value(value)]),
            )
        }
        /// Draggable hint.
        pub fn draggable(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("draggable").value(value)]),
            )
        }
        /// Spellcheck hint (global `spellcheck`).
        pub fn spellcheck_global(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("spellcheck").value(value)]),
            )
        }
        /// Advisory title (global `title`).
        pub fn title_global(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("title").value(value)]),
            )
        }
        /// Translation hint.
        pub fn translate(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("translate").value(value)]),
            )
        }
        /// Contenteditable mode.
        pub fn contenteditable(self, value: &'static str) -> Self {
            Self(self.0.attrs(vec![
                crate::attributes::attr("contenteditable").value(value),
            ]))
        }
        /// CSP nonce (global `nonce`).
        pub fn nonce_global(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("nonce").value(value)]),
            )
        }
        /// Shadow DOM slot name.
        pub fn slot(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("slot").value(value)]),
            )
        }
        /// Shadow DOM `::part()` name.
        pub fn part(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("part").value(value)]),
            )
        }
        /// Virtual keyboard hint.
        pub fn inputmode(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("inputmode").value(value)]),
            )
        }
        /// Enter key label hint.
        pub fn enterkeyhint(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("enterkeyhint").value(value)]),
            )
        }
        /// Popover API marker.
        pub fn popover(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("popover").value(value)]),
            )
        }
        /// Custom element slot identifier.
        pub fn is_content(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("is").value(value)]),
            )
        }
        /// Boolean focus-on-load flag (global `autofocus`).
        pub fn autofocus_global(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("autofocus").value(value)]),
            )
        }
        /// Form association (global `form`).
        pub fn form_global(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("form").value(value)]),
            )
        }
        /// Custom data attribute (`data-*`).
        pub fn data_x(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("data-x").value(value)]),
            )
        }
        /// Datalist reference (global `list`).
        pub fn list_global(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("list").value(value)]),
            )
        }
        /// Autofill hint (global `autocomplete`).
        pub fn autocomplete_global(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("autocomplete").value(value)]),
            )
        }
    };
}
pub(crate) use __common_globals_methods;

/// Event-handler attribute setters.
macro_rules! __event_handlers_methods {
    () => {
        /// Click handler.
        pub fn onclick(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onclick").value(value)]),
            )
        }
        /// Change handler.
        pub fn onchange(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onchange").value(value)]),
            )
        }
        /// Input handler.
        pub fn oninput(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("oninput").value(value)]),
            )
        }
        /// Submit handler.
        pub fn onsubmit(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onsubmit").value(value)]),
            )
        }
        /// Focus handler.
        pub fn onfocus(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onfocus").value(value)]),
            )
        }
        /// Blur handler.
        pub fn onblur(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onblur").value(value)]),
            )
        }
        /// Keydown handler.
        pub fn onkeydown(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onkeydown").value(value)]),
            )
        }
        /// Keyup handler.
        pub fn onkeyup(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onkeyup").value(value)]),
            )
        }
        /// Mousedown handler.
        pub fn onmousedown(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onmousedown").value(value)]),
            )
        }
        /// Mouseup handler.
        pub fn onmouseup(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onmouseup").value(value)]),
            )
        }
        /// Mouseover handler.
        pub fn onmouseover(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onmouseover").value(value)]),
            )
        }
        /// Mouseout handler.
        pub fn onmouseout(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onmouseout").value(value)]),
            )
        }
        /// Mousemove handler.
        pub fn onmousemove(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onmousemove").value(value)]),
            )
        }
        /// Load handler.
        pub fn onload(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("onload").value(value)]),
            )
        }
    };
}
pub(crate) use __event_handlers_methods;

/// Full ARIA attribute setters.
macro_rules! __aria_all_methods {
    () => {
        /// Accessible label.
        pub fn aria_label(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("aria-label").value(value)]),
            )
        }
        /// Accessibility hidden state.
        pub fn aria_hidden(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("aria-hidden").value(value)]),
            )
        }
        /// Element role hint.
        pub fn aria_role(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("role").value(value)]),
            )
        }
        /// ARIA live region politeness.
        pub fn aria_live(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("aria-live").value(value)]),
            )
        }
        /// Expanded state.
        pub fn aria_expanded(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("aria-expanded").value(value)]),
            )
        }
        /// Selected state.
        pub fn aria_selected(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("aria-selected").value(value)]),
            )
        }
    };
}
pub(crate) use __aria_all_methods;

/// Only the `aria-hidden` setter.
macro_rules! __aria_hidden_methods {
    () => {
        /// Accessibility hidden state.
        pub fn aria_hidden(self, value: &'static str) -> Self {
            Self(
                self.0
                    .attrs(vec![crate::attributes::attr("aria-hidden").value(value)]),
            )
        }
    };
}
pub(crate) use __aria_hidden_methods;
