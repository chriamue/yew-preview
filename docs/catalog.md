---
title: Static HTML Catalog
tags: [catalog, ssr, html, export]
---

# Static HTML Catalog

← [[index]]

The `catalog` feature renders every component variant via Yew SSR and writes a
single self-contained HTML file — a long-scroll catalog of all components, like
a design-system reference sheet.

Because it only needs tokio and Yew SSR (no axum, no wasm), it is lighter than
the `serve` feature. `serve` implies `catalog`, so enabling `serve` gives you
both automatically.

## How it works

```
cargo run --example catalog
  └─ generate_catalog_blocking(groups, options)
       └─ LocalSet: Yew SSR pre-renders each variant → String
            └─ single HTML file written to disk
```

All variants of every component are written to one page, grouped by
`ComponentGroup`. A sticky header provides jump-links to each group. Test cases
are shown as collapsible `<details>` sections beneath each component card.

## Setup

### 1. Add the `catalog` feature to dev-dependencies

```toml
# Cargo.toml

[lib]
name = "my_components"
path = "src/lib.rs"

[dev-dependencies]
yew-preview = { ..., features = ["testing", "catalog"] }
```

> If you already have `features = ["testing", "serve"]`, the `catalog` feature
> is already included because `serve` depends on `catalog`.

### 2. Export groups from `lib.rs`

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

### 3. Add `examples/catalog.rs`

```rust
// examples/catalog.rs
fn main() {
    let css = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("my-styles.css");

    // Optional: override output path via environment variable
    let output = std::env::var("OUTPUT")
        .ok()
        .map(std::path::PathBuf::from);

    let mut options = yew_preview::CatalogOptions::new("my-components")
        .css_file(css);

    if let Some(out) = output {
        options = options.output(out);
    }

    yew_preview::generate_catalog_blocking(my_components::preview_groups(), options);
}
```

### 4. Run

```bash
# Output: my-components-yew-preview.html (current directory)
cargo run --example catalog

# Custom output path
OUTPUT=dist/catalog.html cargo run --example catalog
```

## `CatalogOptions`

| Field | Type | Default | Description |
|---|---|---|---|
| `project_name` | `String` | — | Page title and default filename stem |
| `css_file` | `Option<PathBuf>` | `None` | CSS file injected after scaffold styles |
| `output` | `Option<PathBuf>` | `{project_name}-yew-preview.html` | Output path |

Builder methods mirror the fields:

```rust
CatalogOptions::new("my-app")
    .css_file("styles/components.css")   // inject component CSS
    .output("dist/catalog.html")         // custom output path
```

## Output format

The generated file is fully self-contained — no external resources, no
JavaScript. Components are rendered to static HTML via Yew SSR.

```
<sticky header with group jump-links>
  Group 1
    ComponentA  [badge: 2 tests]
      Default variant preview
      Other variant preview
      ▶ 2 test cases  (collapsible)
    ComponentB  [badge: interactive]
      (interactive — use trunk serve for live editing)
  Group 2
    ...
```

## Comparison with the native server

| | `cargo run --example serve` | `cargo run --example catalog` |
|---|---|---|
| Output | Live HTTP server | Static `.html` file |
| Navigation | Sidebar + query params | Sticky nav + anchor links |
| Interactive args | Shown as SSR snapshot | Shown as SSR snapshot |
| Usable offline | No | Yes |
| CI / artefact upload | No | Yes — commit or publish the file |
| Dependencies | axum, tokio | tokio only |

Use the catalog when you want a shareable snapshot — e.g. attached to a pull
request, published to GitHub Pages, or included in generated documentation.