# YewPreview

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Actions Status](https://github.com/chriamue/yew-preview/workflows/GitHub%20Pages%20Deployment/badge.svg)](https://github.com/chriamue/yew-preview/actions)

**YewPreview** is a simple and flexible component explorer and test framework for [Yew](https://yew.rs/) applications, inspired by [Storybook](https://storybook.js.org/). It allows you to preview, configure, and test your Yew components in an isolated and organized environment.

## Purpose

YewPreview helps developers streamline the Yew component development workflow.  It provides a dedicated space to:

*   **Visualize:** See how your components render with different property combinations.
*   **Configure:** Dynamically adjust component properties and observe the changes.
*   **Test:**  Run automated tests against your components to ensure they meet your requirements.

This accelerates development, improves component reusability, and reduces visual regressions.

## Features

*   **Interactive Component Exploration:** Browse a categorized list of your Yew components.
*   **Live Preview:**  Render components with different property sets in a central preview area.
*   **Dynamic Configuration:** Easily modify component properties and see the results instantly.
*   **Test Automation:** Define and run tests directly within the YewPreview environment.
*   **Simple Setup:**  Quick integration with your existing Yew project using [Trunk](https://trunkrs.dev/).
*   **Macro-Powered:**  Uses macros for concise and declarative component definition.

## Getting Started

### Prerequisites

Make sure you have the following installed:

*   [Rust](https://www.rust-lang.org/tools/install) (stable channel recommended)
*   [Trunk](https://trunkrs.dev/#install) (for building and serving the application)
*   A code editor (e.g., VS Code with the Rust Analyzer extension)

### Installation and Usage

Here's how to integrate YewPreview into your Yew project:

1.  **Create a new Yew project (if you don't have one already):**

    ```sh
    cargo new my-yew-project --bin
    cd my-yew-project
    ```

2.  **Add `yew-preview` and `yew` as dependencies in your `Cargo.toml` file:**

    ```toml
    [dependencies]
    yew = { version = "0.21", features = ["csr"] } # Or the latest version
    yew-preview = { git = "https://github.com/chriamue/yew-preview" }
    ```

3.  **Create an `index.html` file in the `src` directory:**

    ```html
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="utf-8">
        <title>Yew Preview</title>
        <link data-trunk rel="rust" href="src/main.rs">
        <link data-trunk rel="scss" href="style.scss"> <!-- Optional: For styling -->
    </head>
    <body></body>
    </html>
    ```

4.  **Create a `style.scss` file for the `index.html` (optional):**

    ```scss
    body {
        margin: 0;
        padding: 0;
        font-family: sans-serif;
    }
    ```

5.  **Update `src/main.rs` with your components and the YewPreview setup:**

    ```rust
    use yew::prelude::*;
    use yew_preview::prelude::*;
    use yew_preview::create_component_group; // Import group macro
    use yew_preview::examples::{header::HeaderComp, image::ImageComp, footer::FooterComp}; // Import example components

    #[function_component(App)]
    pub fn app() -> Html {

        let groups = vec![
            create_component_group!(
                "General",
                HeaderComp::preview(),
                ImageComp::preview(),
                FooterComp::preview()
            ),
            // You can create more groups
        ];

        html! {
            <div style="font-family: sans-serif;">
                <h1>{ "YewPreview Demo" }</h1>
                <PreviewPage groups={groups} />
            </div>
        }
    }

    fn main() {
        yew::Renderer::<App>::new().render();
    }
    ```

6.  **Run the application using Trunk:**

    ```sh
    trunk serve --open
    ```

    This will typically open your browser to `http://127.0.0.1:8080`.

### Creating Component Previews

YewPreview uses the `Preview` trait and associated macros to define how your components are displayed and tested.  Here's a simple example:

```rust
use yew::prelude::*;
use yew_preview::prelude::*;

#[derive(Properties, PartialEq, Clone, Default)]
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

create_preview_with_tests!(
    component: MyComponent,
    default_props: MyComponentProps {
        text: "Hello".to_string(),
        color: "black".to_string(),
    },
    variants: [
        (
            "Red Text",
            MyComponentProps {
                text: "Red Hello".to_string(),
                color: "red".to_string(),
            }
        ),
        (
            "Blue Text",
            MyComponentProps {
                text: "Blue Hello".to_string(),
                color: "blue".to_string(),
            }
        ),
    ],
    tests: [
        (
            "Has correct text",
            has_text("Hello"),
        ),
        (
            "Has correct color",
            has_style("color", "black"),
        ),
    ]
);
```

**Explanation:**

*   `create_preview_with_tests!` macro:
    *   `component`: The Yew component type (`MyComponent`).
    *   `default_props`: The default properties used for the initial preview.
    *   `variants`:  A list of property sets that create different preview variations. Each variant has a name and the corresponding `props`.
    *   `tests`: A list of tests to verify the component's behavior.

### Component Grouping

Components can be grouped for better organization. Here's an example of how to use the `create_component_group!` macro:

```rust
use yew_preview::prelude::*;
use yew_preview::create_component_group;
use yew_preview::examples::{header::HeaderComp, image::ImageComp, footer::FooterComp};

let groups = vec![
    create_component_group!(
        "Layout",
        HeaderComp::preview(),
        FooterComp::preview()
    ),
    create_component_group!(
        "Media",
        ImageComp::preview()
    )
];
```

## Testing Components

YewPreview allows you to define tests for your components directly within the `create_preview_with_tests!` macro.  These tests use matchers to assert specific conditions about the rendered output.  See the example above for how to define the tests.

## Projects Using YewPreview

Here are some projects that use YewPreview to showcase and test their components:

### Konnektoren-Yew

A web application for learning German grammar, focused on connectors (Konnektoren) and conjunctions. Built with Yew, it offers interactive lessons across multiple language platforms.

- **Demo:** [https://konnektoren.github.io/konnektoren-yew/](https://konnektoren.github.io/konnektoren-yew/)
- **Repository:** [https://github.com/konnektoren/konnektoren-yew](https://github.com/konnektoren/konnektoren-yew)
- **Features:** Multi-language support, interactive lessons, progress tracking, achievements, and SSI credential badges

_Have a project using YewPreview? Submit a PR to add it here!_

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a pull request.

## Acknowledgements

Inspired by [Storybook](https://storybook.js.org/).
