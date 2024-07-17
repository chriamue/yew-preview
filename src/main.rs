use yew::prelude::*;
use yew_preview::create_component_item;
use yew_preview::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderCompProps {
    pub title: String,
}

#[function_component(HeaderComp)]
pub fn header_comp(props: &HeaderCompProps) -> Html {
    html! {
        <header>
            <h1>{ &props.title }</h1>
        </header>
    }
}

#[derive(Properties, PartialEq)]
pub struct ImageCompProps {
    pub src: String,
    pub size: u32,
}

#[function_component(ImageComp)]
pub fn image_comp(props: &ImageCompProps) -> Html {
    html! {
        <img src={ props.src.clone() } width={ format!("{}px", &props.size) } height={ format!("{}px", &props.size) } />
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let components: ComponentList = vec![
        create_component_item!(
            "Header",
            html! { <HeaderComp title={"Hello, Yew!"} /> },
            vec![("Title".to_string(), "Hello, Yew!".to_string())]
        ),
        create_component_item!(
            "Image",
            html! { <ImageComp src={"https://www.rust-lang.org/logos/rust-logo-512x512.png"} size={256} /> },
            vec![
                (
                    "Src".to_string(),
                    "https://www.rust-lang.org/logos/rust-logo-512x512.png".to_string()
                ),
                ("Size".to_string(), "256".to_string())
            ]
        ),
    ];

    html! {
        <div>
            <PreviewPage {components} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
