# mrk

A minimal markup builder library for Rust.

`mrk` provides a fluent, type-safe API for building structured markup
trees. Compose elements with `el`, attach attributes with `attr`, and
build children lists with the `nodes!` macro.

## Installation

By default, `mrk` provides the data model and builder API only. Enable
a feature for built-in rendering:

```toml
[dependencies]
mrk = { version = "0.7.0", features = ["html"] }
```

## Quick start (with `html` feature)

```rust
use mrk::*;

let html = el("a")
    .attrs(vec![attr("href").value("/")])
    .children(nodes!["Home"])
    .render();

assert_eq!(html, r#"<a href="/">Home</a>"#);
```

## Features

| Feature | Default | Description |
|---|---|---|
| `html` | no | HTML rendering, 116 tag factories, void elements, escaping |

Without any feature, you can build trees but cannot render them. Implement
`Renderable` for your own renderer, or enable a feature.

## Building trees without rendering

```rust
use mrk::*;

let tree = el("custom-tag")
    .attrs(vec![attr("name").value("value")])
    .children(nodes!["data"]);

assert_eq!(tree.name, "custom-tag");
```

## Direct construction

All struct fields are public. You can construct directly via struct literals:

```rust
use mrk::*;

let div = Element {
    name: "div",
    attributes: vec![attr("class").value("container")],
    children: vec![],
};

let a = Attribute {
    key: "href",
    attr: AttributeType::KeyValue("href", "/"),
};
```

## Factories (with `html` feature)

For common HTML tags, use the factory functions:

```rust
use mrk::*;

let html = div().children(nodes![
    "Hello, ",
    el("strong").children(nodes!["world"]),
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
