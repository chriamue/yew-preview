# YewPreview

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Deploy](https://github.com/chriamue/yew-preview/workflows/GitHub%20Pages%20Deployment/badge.svg)](https://github.com/chriamue/yew-preview/actions/workflows/gh-pages.yml)

**YewPreview** is a lightweight Rust library for interactive component previews in Yew applications — like Storybook, but for Rust. Register static snapshot variants, live-editable args, and SSR test cases, then browse everything in a sidebar UI served by `trunk serve`.

> **Note:** This library is primarily built for personal use cases. It is not actively maintained and breaking changes may occur without notice. Use it as inspiration or a starting point rather than a production dependency.

## Features

- **Fast Setup** — Add previews and tests directly to your component files with minimal boilerplate
- **Macro-Powered** — `create_preview_with_tests!` covers static variants + SSR tests in one call
- **Live Prop Editing** — Edit `Text`, `Bool`, `Int`, `IntRange` (slider), and `Float` args in the browser without recompiling
- **SSR Testing** — Built-in matchers run against server-rendered HTML; failures appear in both `cargo test` and the browser UI
- **Failing-test workflow** — Enable the `tests-ignored` feature to mark all generated tests as `#[ignore]` so a broken component never blocks CI

## Quick Start

### 1. Add Dependency

```toml
[dependencies]
yew = { version = "0.23", features = ["csr"] }
yew-preview = { git = "https://github.com/chriamue/yew-preview" }

[dev-dependencies]
yew-preview = { git = "https://github.com/chriamue/yew-preview", features = ["testing"] }
```

### 2. Static Variants + Tests

`create_preview_with_tests!` registers named snapshot variants and SSR test cases in one macro call. The macro generates an `impl Preview` and a `#[tokio::test]` that runs every test against every variant.

```rust
use yew::prelude::*;
use yew_preview::prelude::*;
use yew_preview::{create_preview_with_tests, test_utils::exists};

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    pub label: String,
    pub disabled: bool,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! { <button disabled={props.disabled}>{ &props.label }</button> }
}

create_preview_with_tests!(
    component: Button,
    default_props: ButtonProps { label: "Click me".to_string(), disabled: false },
    variants: [
        ("Disabled", ButtonProps { label: "Disabled".to_string(), disabled: true }),
    ],
    tests: [
        ("Has button element", exists("button")),
    ]
);
```

Running `cargo test` shows a result per component:

```
test components::button::button_preview::default ... ok
```

### 3. Interactive Previews

Use `create_interactive_preview!` to make props editable live in the browser:

```rust
use yew_preview::prelude::*;

yew_preview::create_interactive_preview!(
    Button,
    args: [
        ("label",    ArgValue::Text("Click me".to_string())),
        ("disabled", ArgValue::Bool(false)),
    ],
    |args| {
        let label    = get_text(args, "label");
        let disabled = get_bool(args, "disabled");
        html! { <Button label={label} disabled={disabled} /> }
    }
);
```

For a size slider use `IntRange(value, min, max)`:

```rust
("size", ArgValue::IntRange(256, 24, 1024))
// read with: get_int(args, "size") as u32
```

### 4. Build the Preview App

```rust
use yew_preview::{create_component_group, prelude::*};

fn get_groups() -> ComponentList {
    vec![
        create_component_group!("Buttons", Button::preview()),
    ]
}

html! { <PreviewPage groups={get_groups()} /> }
```

```bash
trunk serve
```

Open `http://localhost:8080` to browse your previews.

### 5. Native Preview Server (no trunk)

Add `examples/serve.rs` to your project for a zero-dependency preview server:

```toml
# Cargo.toml — serve feature only in dev-dependencies, never in the WASM build
[lib]
name = "my_components"
path = "src/lib.rs"

[dev-dependencies]
yew-preview = { ..., features = ["testing", "serve"] }
```

```rust
// src/lib.rs
pub fn preview_groups() -> ComponentList { ... }

// examples/serve.rs
fn main() {
    yew_preview::serve_blocking(my_components::preview_groups(), 8080);
}
```

```bash
cargo run --example serve
# yew-preview: http://localhost:8080
```

All HTML is generated via Yew SSR at startup — no files, no WASM, no trunk required.

### 6. Integration Test for All Components

Call `run_groups_tests` from a single `#[tokio::test]` to run every component's SSR tests at once:

```rust
#[cfg(test)]
mod tests {
    use super::get_groups;
    use yew_preview::test_utils::run_groups_tests;

    #[tokio::test]
    async fn test_all_components() {
        run_groups_tests(&get_groups()).await;
    }
}
```

## Failing Tests and `tests-ignored`

When a component has a failing test, it shows as failing in both the browser UI (test panel) and in `cargo test`:

```
test components::card::card_comp_preview::default ... FAILED

---- components::card::card_comp_preview::default stdout ----
  [CardComp] [Default] ✓ Has card element
  [CardComp] [Default] ✓ Has heading
thread '...' panicked at src/components/card.rs:30:1:
[CardComp] [Default] Test 'Has close button' failed:
    ✗ element exists: button
```

To keep CI green while a test is known-failing, enable the `tests-ignored` feature on `yew-preview`. All macro-generated tests become `#[ignore]`:

```toml
[dev-dependencies]
yew-preview = { git = "...", features = ["testing", "tests-ignored"] }
```

```
test components::card::card_comp_preview::default ... ignored, tests-ignored feature enabled
test result: ok. 0 passed; 0 failed; 7 ignored
```

Run the ignored tests explicitly when you want to see them:

```bash
cargo test -- --ignored
```

## `ArgValue` Types

| Variant | Control | Accessor |
|---|---|---|
| `ArgValue::Text(String)` | text input | `get_text(args, "key")` |
| `ArgValue::Bool(bool)` | checkbox | `get_bool(args, "key")` |
| `ArgValue::Int(i64)` | number input | `get_int(args, "key")` |
| `ArgValue::IntRange(val, min, max)` | slider | `get_int(args, "key")` |
| `ArgValue::Float(f64)` | number input | `get_float(args, "key")` |

## Documentation

Full notes live in [`docs/`](docs/):

| Note | Contents |
|---|---|
| [Getting Started](docs/getting-started.md) | Install, first preview, run |
| [Interactive Previews](docs/interactive.md) | `ArgValue` types, `create_interactive_preview!`, sliders |
| [Macros Reference](docs/macros.md) | All macros |
| [UI Components](docs/components.md) | `PreviewPage`, `ConfigPanel`, data types, state flow |
| [Testing](docs/testing.md) | Matchers, `TestCase`, `render_component`, `tests-ignored` |
| [Native Server](docs/serve.md) | `serve_blocking`, axum SSR server, `examples/serve.rs` pattern |
| [Architecture](docs/architecture.md) | Crate layout, feature flags, design decisions |
| [Examples](docs/examples.md) | Annotated walkthrough of the bundled example |

## Projects Using YewPreview

- [Konnektoren-Yew](https://github.com/konnektoren/konnektoren-yew) - Interactive German grammar lessons with component previews

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

Contributions welcome! Please submit pull requests.
