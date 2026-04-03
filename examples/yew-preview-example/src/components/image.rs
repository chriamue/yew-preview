use yew::prelude::*;
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

impl Preview for ImageComp {
    fn preview() -> ComponentItem {
        use std::rc::Rc;
        use yew::html;

        let initial_args: Vec<(String, ArgValue)> = vec![
            ("src".to_string(), ArgValue::Text(RUST_LOGO.to_string())),
            ("size".to_string(), ArgValue::IntRange(256, 24, 1024)),
        ];
        let render_fn: Rc<dyn Fn(&[(String, ArgValue)]) -> Html> = Rc::new(|args| {
            let src = get_text(args, "src");
            let size = get_int(args, "size") as u32;
            html! { <ImageComp src={src} size={size} /> }
        });

        ComponentItem {
            name: "ImageComp".to_string(),
            render: vec![
                (
                    "256".to_string(),
                    html! { <ImageComp src={RUST_LOGO} size={256u32} /> },
                ),
                (
                    "512".to_string(),
                    html! { <ImageComp src={RUST_LOGO} size={512u32} /> },
                ),
            ],
            args: Some(InteractiveArgs {
                values: initial_args,
                render_fn,
            }),
            test_cases: vec![
                {
                    let mut tc = TestCase::new("Has correct image source");
                    tc.matchers.push(Matcher::HasAttribute(
                        "src".to_string(),
                        RUST_LOGO.to_string(),
                    ));
                    tc
                },
                {
                    let mut tc = TestCase::new("Has correct size");
                    tc.matchers
                        .push(Matcher::HasAttribute("width".to_string(), "256px".to_string()));
                    tc.matchers
                        .push(Matcher::HasAttribute("height".to_string(), "256px".to_string()));
                    tc
                },
            ],
        }
    }
}
