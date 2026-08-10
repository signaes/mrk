# mrk

A minimal markup builder library for Rust.

`mrk` provides a fluent, type-safe API for building structured markup
trees. Compose elements with `el`, attach attributes with `attr`, and
build children lists with the `nodes!` macro. Layer on opt-in features
for declarative `html!` / `svg!` markup — without external dependencies
and with zero macros left enabled by default.

`mrk` is the data-model crate in a small ecosystem:

- [`mrk`](https://crates.io/crates/mrk) — data model, builder API, `html!` / `svg!` macros.
- [`mrk-components`](https://crates.io/crates/mrk-components) — templated `Component`s.
- [`mrk-css`](https://crates.io/crates/mrk-css) — type-safe CSS authoring (`css!` macro, `StyleSheet`).
- [`mrk-ir`](https://crates.io/crates/mrk-ir) — binary `.mrk` wire format codec.

Each companion crate depends on `mrk` for the `Renderable` trait and the
`Node` / `Element` data model.

## Installation

By default, `mrk` provides the data model, builder API, and built-in
rendering. Enable features for declarative authoring (`html!` / `svg!`
macros and typed tag factories):

```toml
[dependencies]
mrk = { version = "0.10.1", features = ["html"] }
```

Combine features freely:

```toml
mrk = { version = "0.10.1", features = ["html", "svg"] }
```

## Quick start

```rust
use mrk::*;

let html = el("a")
    .append_attrs(vec![attr("href").value("/")])
    .set_children(nodes!["Home"])
    .render();

assert_eq!(html, r#"<a href="/">Home</a>"#);
```

## Features

| Feature | Default | Description |
|---|---|---|
| *(none)*  | yes | Data model only: `el`, `attr`, `Node`, `Element`, `Renderable` |
| `html`    | no  | 114 HTML tag factories, `html!` declarative macro, void elements, escaping |
| `svg`     | no  | 67 SVG 2 tag factories, `svg!` declarative macro |

CSS authoring moved to the standalone
[`mrk-css`](https://github.com/signaes/mrk-css) crate (the former `css`
feature), templated components to
[`mrk-components`](https://github.com/signaes/mrk-components) (the
former `components` feature), and the `.mrk` wire format codec to
`mrk-ir` (the former `ir` feature). All depend on `mrk` for the data
model and the `Renderable` trait.

The core data model is always available: `el`, `attr`, `nodes!`,
`Node`, `Element`, and the `Renderable` trait — with built-in HTML
rendering for the data-model types. The `html` and `svg` features layer
declarative macros and typed tag factories on top.

## Declarative markup with `html!`

With the `html` feature, the `html!` macro builds `Element` trees with
markup-like syntax. Tag names resolve to the `html` module's factory
functions; attribute keys may contain dashes; values must be string
literals.

```rust
use mrk::*;

let tree = html! { div(class="a b c" id="container") {
    span(class="text") { "ok" }
    div() { "sibling" }
    div(data-value="true") { ul() { li() { "1" } li(class="second") { "2" } } }
} };

assert_eq!(
    tree.render(),
    r#"<div class="a b c" id="container"><span class="text">ok</span><div>sibling</div><div data-value="true"><ul><li>1</li><li class="second">2</li></ul></div></div>"#
);
```

Void elements (`img`, `br`, `input`, ...) may omit the braces. Boolean
attributes (`disabled`, `checked`, ...) are bare identifiers:

```rust
let cb = html! { input(type="checkbox" disabled) {} };
```

The macro evaluates to an `Element`, so the result composes with the
rest of the API.

## Declarative markup with `svg!`

The `svg` feature mirrors `html!` for SVG: snake_case tag names
(`linear_gradient`, `font_face`), camelCase attribute names verbatim,
and dashed attribute keys.

```rust
use mrk::*;

let icon = svg! { svg(viewBox="0 0 10 10") {
    circle(cx="5" cy="5" r="4")
    line(x1="0" y1="0" x2="10" y2="10" stroke-width="1")
} };
```

## CSS authoring

CSS authoring lives in the standalone
[`mrk-css`](https://github.com/signaes/mrk-css) crate (extracted from
what used to be the `css` feature of `mrk`). It provides `StyleSheet`,
`Rule`, `AtRule`, typed selectors/declarations/values, the CSS Color 4
parser and conversions, a pretty-printer, and the `css!` macro:

```rust
use mrk_css::{css, Renderable};

let sheet = css! {
    .btn {
        color: rebeccapurple;
        padding: 8px 16px;
        &:hover { color: blue; }
    }
};

let css = sheet.render();
```

A `StyleSheet` implements `mrk::Renderable` and converts into
`mrk::Node`, so stylesheets embed directly in `mrk` markup trees.

## Templated components

Templated components live in the standalone
[`mrk-components`](https://github.com/signaes/mrk-components) crate
(extracted from what used to be the `components` feature of `mrk`).
It provides `Component`, `Expr`, `Props`, typed HTML/SVG wrappers, and
the `component!` / `switch!` / `text!` macros:

```rust,ignore
use mrk_components::*;

component!(Card, div(class="card" id={prop("id")}) {
    span() { {prop("title")} }
    input(type="checkbox" disabled)
});
```

`Component::render(&Props)` returns `Result<Vec<mrk::Node>, RenderError>`.
See the crate's README for the full catalog.

## The `.mrk` wire format

The binary, length-prefixed `.mrk` codec (header `mrk1`, payloads
capped at 64 KiB, round-trip lossless) lives in the standalone
[`mrk-ir`](https://github.com/signaes/mrk-ir) crate, which builds on
`mrk` and `mrk-components`.

## Building trees without rendering

Without any feature, you can build `Element` trees and inspect them
programmatically:

```rust
use mrk::*;

let tree = el("custom-tag")
    .append_attrs(vec![attr("name").value("value")])
    .set_children(nodes!["data"]);

assert_eq!(tree.name, "custom-tag");
```

## Direct construction

All struct fields are public. You can construct directly via struct literals:

```rust
use mrk::*;
use std::borrow::Cow;

let div = Element {
    name: Cow::Borrowed("div"),
    attributes: vec![attr("class").value("container")],
    children: vec![],
};

let a = Attribute {
    key: Cow::Borrowed("href"),
    attr: AttributeType::KeyValue(Cow::Borrowed("href"), Cow::Borrowed("/")),
};
```

## Factories (with `html` feature)

For common HTML tags, use the factory functions:

```rust
use mrk::*;

let html = div().set_children(nodes![
    "Hello, ",
    el("strong").set_children(nodes!["world"]),
]).render();
```

## Implementing `Renderable`

Any type can be rendered by implementing `Renderable`:

```rust
use mrk::*;

struct Greeting(&'static str);

impl Renderable for Greeting {
    fn render(&self) -> String {
        format!("<p>Hello, {}!</p>", self.0)
    }
}

assert_eq!(render(Greeting("world")), "<p>Hello, world!</p>");
```

## License

Licensed under the [MIT License](LICENSE-MIT).

## Migration from 0.9

`mrk` 0.10.0 is a breaking release. The data model (`el`, `attr`,
`Node`, `Element`, `Renderable`) and the `html!` / `svg!` macros are
unchanged. The following capabilities were moved to dedicated crates:

| 0.9 feature   | New crate                                       | Notes |
|---------------|-------------------------------------------------|-------|
| `components`  | [`mrk-components`](https://crates.io/crates/mrk-components) | `Component`, `Expr`, `Props`, `component!` / `switch!` / `text!` macros |
| `css`         | [`mrk-css`](https://crates.io/crates/mrk-css)               | `StyleSheet`, `Rule`, `AtRule`, `css!` macro, CSS Color 4 |
| `ir`          | [`mrk-ir`](https://crates.io/crates/mrk-ir)                 | `.mrk` codec (depends on `mrk-components`) |

Update your `Cargo.toml`:

```toml
# Before (0.9)
mrk = { version = "0.9", features = ["html", "svg", "components", "css", "ir"] }

# After (0.10)
mrk          = { version = "0.10", features = ["html", "svg"] }
mrk-css        = "0.1.3"
mrk-components = "0.0.1"
mrk-ir         = "0.0.1"
```

Re-export paths changed: `mrk::components::*` is now
`mrk_components::*`; `mrk::css::*` is now `mrk_css::*`;
`mrk::ir::*` is now `mrk_ir::*`.