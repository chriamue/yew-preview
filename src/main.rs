use yew::prelude::*;

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
    html! {
        <div>
            <HeaderComp title={"Hello, Yew!"} />
            <ImageComp src={"https://www.rust-lang.org/logos/rust-logo-512x512.png"} size={256} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
