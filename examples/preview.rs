use yew::prelude::*;
use yew_preview::prelude::*;
use yew_preview::create_component_group;

mod components;
use components::{
    footer::FooterComp,
    header::HeaderComp,
    image::ImageComp,
    project::ProjectComp,
};

#[function_component(App)]
pub fn app() -> Html {
    let groups: ComponentList = vec![
        create_component_group!(
            "Layout Components",
            HeaderComp::preview(),
            FooterComp::preview()
        ),
        create_component_group!("Media Components", ImageComp::preview()),
        create_component_group!("Projects", ProjectComp::preview()),
    ];

    html! {
        <div class="yew-preview" style="font-family: Arial, sans-serif; height: 100vh; display: flex; flex-direction: column; overflow: hidden;">
            <div class="yew-preview-header" style="padding: 10px; background-color: #f8f8f8; border-bottom: 1px solid #ccc; flex-shrink: 0;">
                <h1 style="text-align: center;">
                    { "YewPreview Component Testing Framework" }
                </h1>
            </div>
            <div class="yew-preview-content" style="flex: 1; overflow: hidden;">
                <PreviewPage groups={groups} />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
