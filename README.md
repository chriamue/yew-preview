# YewPreview

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Deploy](https://github.com/chriamue/yew-preview/workflows/GitHub%20Pages%20Deployment/badge.svg)](https://github.com/chriamue/yew-preview/actions/workflows/gh-pages.yml)

**YewPreview** is a lightweight Rust library for quickly generating interactive component previews in Yew applications. Add preview variants directly to your components with minimal boilerplate.

## Features

- 🚀 **Fast Setup** - Add previews directly to your component files
- 📦 **Macro-Powered** - Concise `create_preview!` macro syntax
- 🎯 **Feature-Gated** - Optional preview code with `yew-preview` feature
- 🧪 **Testing** - Built-in test support for component validation
- 📱 **Interactive UI** - Browse and test component states live
- 🔄 **Quick Iteration** - See changes instantly with Trunk hot reload

## Quick Start

### 1. Add Dependency

```toml
[dependencies]
yew = { version = "0.21", features = ["csr"] }
yew-preview = { git = "https://github.com/chriamue/yew-preview" }

[features]
default = []
yew-preview = []
```

### 2. Create Preview in Component File

Add a `preview` module to your component:

```rust
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MyComponentProps {
    pub text: String,
    pub color: String,
}

#[function_component(MyComponent)]
pub fn my_component(props: &MyComponentProps) -> Html {
    html! {
        <div style={format!("color: {}", props.color)}>
            {props.text.clone()}
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        MyComponent,
        MyComponentProps {
            text: "Default".to_string(),
            color: "black".to_string(),
        },
        (
            "Red Text",
            MyComponentProps {
                text: "Red".to_string(),
                color: "red".to_string(),
            }
        ),
        (
            "Blue Text",
            MyComponentProps {
                text: "Blue".to_string(),
                color: "blue".to_string(),
            }
        )
    );
}
```

### 3. Build Preview App

Create `src/main.rs`:

```rust
use yew::prelude::*;
use yew_preview::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let groups = vec![
        yew_preview::create_component_group!(
            "Components",
            MyComponent::preview()
        ),
    ];

    html! {
        <PreviewPage groups={groups} />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
```

### 4. Run

```bash
trunk serve
```

Open `http://localhost:8080` to browse your previews.

## Workflow

1. **Define your component** normally with `#[function_component]`
2. **Add preview variants** in a `#[cfg(feature = "yew-preview")]` module
3. **Use `create_preview!` macro** to register preview states
4. **Build with feature flag** to generate preview UI
5. **Iterate quickly** with Trunk hot reload

## Documentation

Full Obsidian-style notes live in [`docs/`](docs/):

| Note | Contents |
|---|---|
| [Getting Started](docs/getting-started.md) | Install, first preview, run |
| [Macros Reference](docs/macros.md) | `create_preview!`, `create_component_group!`, `generate_component_test!` |
| [UI Components](docs/components.md) | `PreviewPage`, `Sidebar`, `ConfigPanel`, data types |
| [Testing](docs/testing.md) | Matchers, `TestCase`, `render_component` |
| [Architecture](docs/architecture.md) | Crate layout, feature flags, design decisions |
| [Examples](docs/examples.md) | Annotated walkthrough of the bundled example |

## Projects Using YewPreview

- [Konnektoren-Yew](https://github.com/konnektoren/konnektoren-yew) - Interactive German grammar lessons with component previews

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

Contributions welcome! Please submit pull requests.
