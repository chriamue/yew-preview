use yew::prelude::*;
use yew_preview::prelude::*;
use yew_preview::{create_component_item, create_preview};

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

#[derive(Properties, PartialEq, Clone, Default)]
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

create_preview!(
    ImageComp,
    ImageCompProps::default(),
    (
        "256",
        ImageCompProps {
            size: 256,
            src: "https://www.rust-lang.org/logos/rust-logo-512x512.png".to_string()
        }
    ),
    (
        "512",
        ImageCompProps {
            size: 512,
            src: "https://www.rust-lang.org/logos/rust-logo-512x512.png".to_string()
        }
    )
);

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
        ImageComp::preview(),
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
