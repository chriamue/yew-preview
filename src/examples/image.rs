use crate::{create_preview_with_tests, prelude::*};
use yew::prelude::*;

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

create_preview_with_tests!(
    component: ImageComp,
    default_props: ImageCompProps::default(),
    variants: [
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
    ],
    tests: [
        (
            "Has correct image source",
            Matcher::HasAttribute("src".to_string(), "https://www.rust-lang.org/logos/rust-logo-512x512.png".to_string())
        ),
        (
            "Has correct size",
            Matcher::HasAttribute("width".to_string(), "256px".to_string()),
            Matcher::HasAttribute("height".to_string(), "256px".to_string())
        )
    ]
);
