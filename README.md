# YewPreview

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Deploy](https://github.com/chriamue/yew-preview/workflows/GitHub%20Pages%20Deployment/badge.svg)](https://github.com/chriamue/yew-preview/actions/workflows/gh-pages.yml)

**YewPreview** is a lightweight Rust library for interactive component previews in Yew applications — like Storybook, but for Rust. Register static snapshot variants or live-editable args, then browse everything in a sidebar UI served by `trunk serve`.

> **Note:** This library is primarily built for personal use cases. It is not actively maintained and breaking changes may occur without notice. Use it as inspiration or a starting point rather than a production dependency.

## Features

- 🚀 **Fast Setup** — Add previews directly to your component files with minimal boilerplate
- 📦 **Macro-Powered** — `create_preview!` and `create_interactive_preview!` macros
- 🎛️ **Live Prop Editing** — Edit `Text`, `Bool`, `Int`, `IntRange` (slider), and `Float` args in the browser without recompiling
- 🎯 **Feature-Gated** — Preview code compiles out of production builds
- 🧪 **Testing** — Built-in matchers and `generate_component_test!` for SSR validation
- 🔄 **Quick Iteration** — Trunk hot reload during development

## Quick Start

### 1. Add Dependency

```toml
[dependencies]
yew = { version = "0.23", features = ["csr"] }
yew-preview = { git = "https://github.com/chriamue/yew-preview" }
```

### 2. Static Variants

Register named snapshot variants with `create_preview!`:

```rust
use yew::prelude::*;
use yew_preview::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub label: AttrValue,
    pub disabled: bool,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! { <button disabled={props.disabled}>{ &props.label }</button> }
}

yew_preview::create_preview!(
    Button,
    ButtonProps { label: AttrValue::from("Click me"), disabled: false },
    ("Disabled", ButtonProps { label: AttrValue::from("Can't click"), disabled: true }),
);
```

### 3. Interactive Previews

Use `create_interactive_preview!` to make props editable live in the browser:

```rust
yew_preview::create_interactive_preview!(
    Button,
    args: [
        ("label",    ArgValue::Text("Click me".to_string())),
        ("disabled", ArgValue::Bool(false)),
    ],
    |args| {
        let label    = get_text(args, "label");
        let disabled = get_bool(args, "disabled");
        html! {
            <Button label={AttrValue::from(label)} disabled={disabled} />
        }
    }
);
```

For a size slider, use `IntRange(value, min, max)`:

```rust
("size", ArgValue::IntRange(256, 24, 1024))
// read with: get_int(args, "size") as u32
```

### 4. Mix Static and Interactive

Set both `render` and `args` in a manual `Preview` impl to get static snapshot tabs alongside an **Interactive** tab:

```rust
impl Preview for ImageComp {
    fn preview() -> ComponentItem {
        ComponentItem {
            name: "ImageComp".to_string(),
            render: vec![
                ("256".to_string(), html! { <ImageComp src={SRC} size={256u32} /> }),
                ("512".to_string(), html! { <ImageComp src={SRC} size={512u32} /> }),
            ],
            args: Some(InteractiveArgs {
                values: vec![
                    ("src".to_string(),  ArgValue::Text(SRC.to_string())),
                    ("size".to_string(), ArgValue::IntRange(256, 24, 1024)),
                ],
                render_fn: Rc::new(|args| {
                    let src  = get_text(args, "src");
                    let size = get_int(args, "size") as u32;
                    html! { <ImageComp src={src} size={size} /> }
                }),
            }),
            test_cases: vec![],
        }
    }
}
```

### 5. Build the Preview App

```rust
use yew_preview::{create_component_group, prelude::*};

let groups: ComponentList = vec![
    create_component_group!("Buttons", Button::preview()),
];

html! { <PreviewPage groups={groups} /> }
```

```bash
trunk serve
```

Open `http://localhost:8080` to browse your previews.

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
| [Interactive Previews](docs/interactive.md) | `ArgValue` types, `create_interactive_preview!`, sliders, mixed mode |
| [Macros Reference](docs/macros.md) | All macros including `create_interactive_preview!` |
| [UI Components](docs/components.md) | `PreviewPage`, `ConfigPanel`, data types, state flow |
| [Testing](docs/testing.md) | Matchers, `TestCase`, `render_component` |
| [Architecture](docs/architecture.md) | Crate layout, feature flags, design decisions |
| [Examples](docs/examples.md) | Annotated walkthrough of the bundled example |

## Projects Using YewPreview

- [Konnektoren-Yew](https://github.com/konnektoren/konnektoren-yew) - Interactive German grammar lessons with component previews

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

Contributions welcome! Please submit pull requests.
