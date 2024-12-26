use yew::prelude::*;
use yew_preview::examples::{
    footer::FooterComp,
    header::{HeaderComp, HeaderCompProps},
    image::ImageComp,
};
use yew_preview::prelude::*;
use yew_preview::{create_component_group, create_component_item};

#[function_component(App)]
pub fn app() -> Html {
    let groups: ComponentList = vec![
        create_component_group!(
            "Layout Components",
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
            FooterComp::preview()
        ),
        create_component_group!("Media Components", ImageComp::preview()),
    ];

    html! {
        html! {
            <div style="
                font-family: Arial, sans-serif;
                height: 100vh;
                display: flex;
                flex-direction: column;
                overflow: hidden;
            ">
                <div style="
                    padding: 10px;
                    background-color: #f8f8f8;
                    border-bottom: 1px solid #ccc;
                    flex-shrink: 0;
                ">
                    <h1 style="text-align: center;">
                        { "YewPreview Component Testing Framework" }
                    </h1>
                </div>
                <div style="flex: 1; overflow: hidden;">
                    <PreviewPage {groups} />
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
