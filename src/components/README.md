# `mrk::components`

Templated, fully-serializable markup components.

This directory contains:

| File         | Contents                                            |
|--------------|-----------------------------------------------------|
| `mod.rs`     | Module doc, [`Component`], [`RenderError`], the render engine, [`wrap`] handling of `<LiteralChildren>`. |
| `props.rs`   | [`Props`] bag, [`PropType`] enum, [`Number`]/[`NumberKind`]. |
| `expr.rs`    | [`Expr`] AST, [`MatchArm`], [`IntoExpr`] trait, [`list!`] macro, all the constructor helpers (`literal`, `prop`, `list_expr`, `either`, `maybe`, `map`, `match_on`, `arm`, `wrap`, `component`). |
| `tests.rs`   | Test module (compiled only under the `components` feature). |

## Feature flag

Everything here is gated behind the **`components`** Cargo feature
(off by default). Enable it like:

```toml
mrk = { version = "0.7.0", features = ["components"] }
```

The `ir` feature depends on `components` and adds the on-the-wire
codec — see `../ir/README.md`.

## What it is

A `Component` is a named expression tree. Render it with `Props` to
produce a `Vec<Node>`. There are no closures, no `Fn` types, no
opaque values: the whole tree is plain data, so it round-trips losslessly
through serialization.

```rust,ignore
use mrk::*;

let card = component(
    "card",
    wrap(
        el("div")
            .attrs(vec![attr("class").value("card")]),
        list![
            prop("title"),
            either("is_admin", prop("admin_tools"), prop("user_tools")),
        ],
    ),
);

let mut props = Props::new();
props.insert("title", PropType::String("Welcome".into()));
props.insert("is_admin", PropType::Bool(true));

let nodes = card.render(&props).unwrap();
```

## The expression tree

`Expr` has nine variants. Each maps cleanly to a render strategy:

| Variant          | Output                                            |
|------------------|---------------------------------------------------|
| `Literal(el)`    | A single `Node::Element(el)`                       |
| `Prop(key)`      | A `Node::Text` from the prop's text form          |
| `List(items)`    | Concatenation of each sub-expression's output      |
| `Match { ... }`  | First arm whose `value` matches the prop's string  |
| `Either { ... }` | `then` or `otherwise` based on a bool prop         |
| `Maybe { ... }`  | `then` when bool prop is true, else empty          |
| `Map { ... }`    | Body evaluated once per list item                 |
| `Wrap { ... }`   | A new element with rendered children               |
| `LiteralChildren(nodes)` | Pre-evaluated nodes, returned as-is        |

**Lenient vs strict:** `Prop` is lenient (missing or wrong type →
empty string). `Match`, `Either`, `Maybe`, `Map` are strict — type
mismatches return `RenderError::TypeMismatch`.

## Constructor helpers

Writing `Expr::Literal(el)` everywhere reads awkwardly, so we expose
short-named builders that produce the right variant:

| Helper      | Produces         |
|-------------|------------------|
| `literal`   | `Expr::Literal`  |
| `prop`      | `Expr::Prop`     |
| `list_expr` | `Expr::List`     |
| `either`    | `Expr::Either`   |
| `maybe`     | `Expr::Maybe`    |
| `map`       | `Expr::Map`      |
| `match_on`  | `Expr::Match`    |
| `arm`       | `MatchArm`       |
| `wrap`      | `Expr::Wrap` (special-cases the body's prepended `<el>`'s children). |
| `component` | The `Component` struct itself. |

`wrap(el, body)` splices any pre-existing children from `el` in front
of the new body via `LiteralChildren`, then sets up the `Wrap`
expression. This is how the static children of a template
`Element` become part of the rendered body without duplicate output.

The [`list!`] macro accepts `Element`, `Expr`, `Box<Expr>`, and
`Node` items directly so you don't have to spell out `Expr::Literal`
each time:

```rust,ignore
list![
    el("h1").children(nodes!["Title"]),
    prop("name"),
    Node::Text("static".into()),
]
```

The trait behind that convenience is [`IntoExpr`].

## Render

`Component::render(&props) -> Result<Vec<Node>, RenderError>`
runs the engine. The recursive core is private — see
[`render_expr`] in `mod.rs`. Two helpers handle the strict lookups:

- `require_bool` for `Either`/`Maybe` conditions
- `require_list` for `Map` inputs

A `Props` lookup that returns the wrong type yields
`RenderError::TypeMismatch { key, expected, found }`.

## Render engine details

The engine is small (≈150 lines in `mod.rs`). It's a straightforward
match on `Expr`, with the two strict-lookup helpers for typed
contexts. It performs **no** heap allocations beyond what the caller
already did, and **no** string allocation in the fast paths; the only
allocation happens when `PropType::to_text` materializes a string
from a non-string prop.

`Either` / `Maybe` evaluate their `then` once if needed; the
`otherwise` branch is only evaluated when the condition is false.

## Round-trip

Combined with the `ir` feature, a `Component` round-trips losslessly
through the `.mrk` wire format. `Mrk::bytes_component` /
`Mrk::from_bytes_component` are the entry points.

See `../ir/README.md` for the wire-format details and the
`#[cfg(test)] mod tests` block in `tests.rs` for the test set.

## Testing

The test file (`tests.rs`) is included by `lib.rs` only when both
the `components` feature and `cfg(test)` are active. It contains
≈70 tests covering props round-tripping, every `Expr` variant, and
each render-edge case. See it for examples of how each constructor
helper composes.

## Layout rationale

Three files:

- **`mod.rs`** — the render engine, the `Component`/`RenderError`
  types, and the public re-exports.
- **`props.rs`** — the value types. Pure data, no behavior.
- **`expr.rs`** — the AST, the `IntoExpr` trait, the `list!` macro,
  and the constructors. The only "active" code; everything else is
  matching on the AST.

`tests.rs` is the test module, kept separate so the production
files stay focused.
