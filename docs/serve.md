---
title: Native Preview Server
tags: [serve, axum, ssr, dev-tools]
---

# Native Preview Server

← [[index]]

The `serve` feature starts an axum HTTP server that pre-renders every component variant via Yew SSR and generates the preview UI entirely in Rust — no trunk, no WASM, no files on disk.

Because it lives in `dev-dependencies`, axum and tokio are never included in your WASM production build.

## How it works

```
cargo run --example serve
  └─ serve_blocking(groups, port)
       └─ LocalSet: Yew SSR pre-renders each variant → String
            └─ axum serves generated HTML on every request
```

All HTML is built from the pre-rendered strings at request time. Navigating between components and variants is handled by query parameters (`?g=Group&c=Component&v=Variant`).

## Setup

### 1. Add `[lib]` target and `serve` dev-dependency

```toml
# Cargo.toml

[lib]
name = "my_components"
path = "src/lib.rs"

[dev-dependencies]
yew-preview = { ..., features = ["testing", "serve"] }
```

### 2. Export groups from `lib.rs`

The example binary needs to import your groups, so expose them from the library target:

```rust
// src/lib.rs
use yew_preview::prelude::*;
use yew_preview::create_component_group;

pub fn preview_groups() -> ComponentList {
    vec![
        create_component_group!("Buttons", Button::preview()),
        create_component_group!("Forms",   Input::preview()),
    ]
}
```

### 3. Add `examples/serve.rs`

```rust
// examples/serve.rs
fn main() {
    let port: u16 = std::env::var("PORT")
        .ok().and_then(|p| p.parse().ok()).unwrap_or(8080);
    yew_preview::serve_blocking(my_components::preview_groups(), port);
}
```

### 4. Run

```bash
cargo run --example serve
# yew-preview: http://localhost:8080

# custom port
PORT=9000 cargo run --example serve
```

## Comparison with trunk

| | `trunk serve` | `cargo run --example serve` |
|---|---|---|
| Output | WASM + JS bundle | Native binary |
| Hot reload | Yes | No |
| Build time | Slower (WASM compile) | Fast (native) |
| Dependencies | trunk, wasm-pack | none extra |
| Interactive args | Yes (browser JS) | Shows SSR snapshot only |
| Test results | Browser UI panel | Browser UI panel |

Use `trunk serve` for interactive development; use the native server for quick inspection, CI doc generation, or environments without a browser WASM runtime.
