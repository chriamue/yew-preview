use yew::prelude::*;
use yew_preview::create_preview_with_tests;
use yew_preview::prelude::*;

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

const RUST_LOGO: &str = "https://www.rust-lang.org/logos/rust-logo-512x512.png";

create_preview_with_tests!(
    component: ImageComp,
    default_props: ImageCompProps { src: RUST_LOGO.to_string(), size: 256 },
    variants: [
        ("512", ImageCompProps { src: RUST_LOGO.to_string(), size: 512 }),
    ],
    tests: [
        ("Has img element",     Matcher::Exists("img".to_string())),
        ("Has image source",    Matcher::HasAttribute("src".to_string(), RUST_LOGO.to_string())),
        ("Has size attributes", Matcher::Contains("width=\"".to_string()),
                                Matcher::Contains("height=\"".to_string())),
    ]
);
