# YewPreview

**YewPreview** is a simple and flexible test framework for Yew components, inspired by Storybook. It allows you to preview and test Yew components in an organized and user-friendly manner.

## Purpose

The purpose of YewPreview is to provide a tool for developers to easily create, preview, and test Yew components with configurable parameters. By using this framework, you can quickly see how your components look and behave, making the development process more efficient.

## Features

- Display a list of available components
- Preview selected components in the center of the page
- Configure and test component parameters dynamically
- Simple setup with Trunk for building and serving the application

## Setup and Usage

### Prerequisites

Ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Trunk](https://trunkrs.dev/#install)
- [Yew](https://yew.rs/docs/getting-started/installation)

### Installation

1. Create a new Rust project:

    ```sh
    cargo new my-yew-preview --bin
    cd my-yew-preview
    ```

2. Add dependencies to `Cargo.toml`:

    ```toml
    [dependencies]
    yew = { version = "0.21", features = ["csr"] }
    yew-preview = { git = "https://github.com/chriamue/yew-preview" }
    ```

3. Create the `index.html` file in the `src` directory:

    ```html
   <!doctype html>
   <html lang="en">
   <head>
       <meta charset="utf-8" />
       <title>Yew Preview</title>
       <link rel="rust" href="src/main.rs" />
   </head>
   <body></body>
   </html>
    ```

4. Update `main.rs` in the `src` directory:

   ```rust
   use yew::prelude::*;
   use yew_preview::prelude::*;
   use yew_preview::create_component_item;
   
   #[derive(Properties, PartialEq, Clone)]
   pub struct HeaderCompProps {
       pub title: String,
   }
   
   #[function_component(HeaderComp)]
   pub fn header_comp(props: &HeaderCompProps) -> Html {
       html! {
           <header style="border-bottom: 1px solid #ccc; padding: 10px;">
               <h1>{ &props.title }</h1>
           </header>
       }
   }
   
   #[derive(Properties, PartialEq, Clone)]
   pub struct ImageCompProps {
       pub src: String,
       pub size: u32,
   }
   
   #[function_component(ImageComp)]
   pub fn image_comp(props: &ImageCompProps) -> Html {
       html! {
           <img src={ props.src.clone() } width={ format!("{}px", &props.size) } height={ format!("{}px", &props.size) } style="display: block; margin: 0 auto;" />
       }
   }
   
   #[function_component(App)]
   pub fn app() -> Html {
       let components: ComponentList = vec![
           create_component_item!(
               "Header",
               HeaderComp,
               vec![
                   (
                       "Hello".to_string(),
                       HeaderCompProps {
                           title: "Hello, World!".to_string()
                       }
                   ),
                   (
                       "Goodbye".to_string(),
                       HeaderCompProps {
                           title: "Goodbye, World!".to_string()
                       }
                   )
               ]
           ),
           create_component_item!(
               "Image",
               ImageComp,
               vec![
                   (
                       "256".to_string(),
                       ImageCompProps {
                           size: 256,
                           src: "https://www.rust-lang.org/logos/rust-logo-512x512.png".to_string()
                       }
                   ),
                   (
                       "512".to_string(),
                       ImageCompProps {
                           size: 512,
                           src: "https://www.rust-lang.org/logos/rust-logo-512x512.png".to_string()
                       }
                   )
               ]
           ),
       ];
   
       html! {
           <div style="font-family: Arial, sans-serif;">
               <h1 style="text-align: center;">{ "YewPreview Component Testing Framework" }</h1>
               <PreviewPage components={components} />
           </div>
       }
   }
   
   fn main() {
       yew::Renderer::<App>::new().render();
   }
   ```

### Running the Application

1. Build and serve the application using Trunk:

    ```sh
    trunk serve --open
    ```

2. Open your browser and navigate to `http://127.0.0.1:8080` to see YewPreview in action.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
```