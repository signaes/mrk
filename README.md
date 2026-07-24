# mrk

A minimal HTML builder library for Rust.

`mrk` provides a fluent, type-safe API for constructing HTML. Create elements
with `el`, attach attributes with `attr`, build children lists with the `nodes!`
macro (mixing `&'static str`, `String`, and elements freely), then render to a
string.

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
    .children(nodes!["Home"])
    .render();

assert_eq!(html, r#"<a href="/">Home</a>"#);
```

## Factories

For common HTML tags, use the factory functions (`div`, `p`, `span`, `ul`,
`li`, `input`, ...):

```rust
use mrk::*;

let html = div().children(nodes![
    "Hello, ",
    el("strong").children(nodes!["world"]),
]).render();
// "<div>Hello, <strong>world</strong></div>"
```

## Nested elements

Strings and elements (from `el` or factories) compose freely inside
`children(nodes![...])`:

```rust
use mrk::*;

let html = ul().children(nodes![
    el("li").children(nodes!["first"]),
    el("li").children(nodes!["second"]),
]).render();
// "<ul><li>first</li><li>second</li></ul>"
```

## Runtime strings

`String` values work directly in `nodes!` — useful for user input or computed
content:

```rust
use mrk::*;

let name = String::from("World");
let html = p().children(nodes![format!("Hello, {}!", name)]).render();
// "<p>Hello, World!</p>"
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
