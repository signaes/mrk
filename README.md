# mrk

A minimal HTML builder library for Rust.

`mrk` provides a fluent, type-safe API for constructing HTML. Create elements
with `el`, attach attributes with `attr`, add children with `text` or nested
elements, then render to a string.

## Installation

```toml
[dependencies]
mrk = "0.1"
```

## Quick start

```rust
use mrk::*;

let html = el("a")
    .attrs(vec![attr("href").value("/")])
    .children(vec![text("Home")])
    .render();

assert_eq!(html, r#"<a href="/">Home</a>"#);
```

## Factories

For common HTML tags, use the factory functions (`div`, `p`, `span`, `ul`,
`li`, `input`, ...):

```rust
use mrk::*;

let html = div().children(vec![
    text("Hello, "),
    node(el("strong").children(vec![text("world")])),
]).render();
// "<div>Hello, <strong>world</strong></div>"
```

## Nested elements

`text` and elements (from `el` or factories) compose inside
`.children(vec![...])`. Use `node(...)` to wrap each element child so the
vector is homogeneous:

```rust
use mrk::*;

let html = ul().children(vec![
    node(el("li").children(vec![text("first")])),
    node(el("li").children(vec![text("second")])),
]).render();
// "<ul><li>first</li><li>second</li></ul>"
```

## Boolean and key-value attributes

`attr(name)` produces a boolean attribute by default. Call `.value(...)` to
turn it into a key/value pair:

```rust
use mrk::*;

assert_eq!(attr("disabled").render(), "disabled");
assert_eq!(attr("href").value("/").render(), "href=\"/\"");
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
