# `mrk::components`

Templated, fully-serializable markup components.

## Directory layout

| File         | Contents                                            |
|--------------|-----------------------------------------------------|
| `mod.rs`     | Module doc, `Component`, `RenderError`, render engine |
| `props.rs`   | `Props` bag, `PropType` enum, `Number`/`NumberKind` |
| `expr.rs`    | `Expr` AST, `MatchArm`, `IntoExpr` trait, `list!` macro, `prop()` |
| `element.rs` | `ComponentElement`, `ComponentAttribute`, typed-wrapper macro |
| `html/mod.rs`| 114 typed HTML wrappers (`Div`, `Span`, `H1`, etc.) |
| `svg/mod.rs` | 67+ typed SVG wrappers (`Circle`, `Rect`, `Path`, etc.) |
| `macros.rs`  | `component!`, `switch!`, `text!` macros |
| `tests.rs`   | Test module (≈490 tests) |

## Feature flag

Everything here is gated behind the **`components`** Cargo feature
(off by default):

```toml
mrk = { version = "0.9.0", features = ["components"] }
```

The `ir` feature depends on `components` and adds the on-the-wire
codec — see `../ir/README.md`.

## API overview

### `component!` — define a template

```rust,ignore
use mrk::*;

component!(Card, {
    div().class(prop("class")).children(vec![
        text!(prop("title")),
        text!(prop("body")),
    ])
});
```

`component!` generates a `Component` struct with a `render(&Props)` method.
Inside the body, use `prop("key")` for dynamic values and `text!` for
text concatenation.

### `switch!` — conditional branching

```rust,ignore
component!(Status, {
    switch!(prop("status"), {
        "ok"   => div().children(nodes!["All good"]),
        "warn" => div().children(nodes!["Warning"]),
        _      => div().children(nodes!["Unknown"]),
    })
});
```

### `text!` — text concatenation

```rust,ignore
text!(prop("greeting"), ", ", prop("name"), "!")
```

### Typed HTML/SVG wrappers

Each HTML/SVG tag has a typed wrapper with fluent setters:

```rust,ignore
use mrk::components::html::{div, span, h1, p};
use mrk::components::svg::{circle, rect};

// HTML
div().id("main").class("container").children(vec![
    h1().children(nodes!["Title"]),
    p().children(nodes!["Hello"]),
]);

// SVG
circle().cx(50).cy(50).r(25).fill("red");
rect().x(10).y(10).width(100).height(50);
```

All setters accept `impl IntoExpr`, so they work with both static
strings and `prop("key")` for dynamic values.

### `nodes!` macro

The `nodes!` macro builds `Vec<Node>` from mixed items:

```rust,ignore
nodes![
    div().children(nodes!["static"]),
    text!(prop("name")),
    Node::Element(el("custom")),
    "plain string",
]
```

Typed wrappers, `Expr`, `Element`, `&str`, and `String` all convert
into `Node` automatically.

## The expression tree

`Expr` has nine variants:

| Variant                  | Output                                           |
|--------------------------|--------------------------------------------------|
| `Literal(el)`           | A single `Node::Element(el)`                     |
| `Wrap { ... }`          | Element with rendered children                   |
| `List(items)`           | Concatenation of each sub-expression's output    |
| `Prop(key)`             | A `Node::Text` from the prop's text form         |
| `Match { ... }`         | First arm whose `value` matches the prop string  |
| `Either { ... }`        | `then` or `otherwise` based on a bool prop       |
| `Maybe { ... }`         | `then` when bool prop is true, else empty        |
| `Map { ... }`           | Body evaluated once per list item                |
| `LiteralChildren(nodes)`| Pre-evaluated nodes, returned as-is              |

## Constructor helpers

| Helper   | Produces        |
|----------|-----------------|
| `prop()` | `Expr::Prop`    |
| `list!`  | `Expr::List`    |

`IntoExpr` is implemented for `Element`, `ComponentElement`, `Expr`,
`Box<Expr>`, `String`, `&str`, `Node`, and all typed wrappers — so
you can use them directly everywhere.

## Render

`Component::render(&Props) -> Result<Vec<Node>, RenderError>`
runs the engine. Props lookups that return the wrong type yield
`RenderError::TypeMismatch { key, expected, found }`.

## Round-trip

Combined with the `ir` feature, a `Component` round-trips losslessly
through the `.mrk` wire format. See `../ir/README.md`.

## Testing

`tests.rs` contains ≈490 tests covering all `Expr` variants, typed
wrappers, `nodes!` mixing, `component!` generation, `switch!` branching,
and every render edge case.
