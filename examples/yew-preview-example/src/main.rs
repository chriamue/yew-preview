use yew::prelude::*;
use yew_preview::create_component_group;
use yew_preview::prelude::*;

mod components;
mod pages;

use components::{
    badge::Badge, code_snippet::CodeSnippet, feature_card::FeatureCard, footer::FooterComp,
    header::HeaderComp, image::ImageComp, project::ProjectComp,
    prop_showcase::PropShowcase,
};
use pages::{
    doc_page::DocPage, getting_started_page::GettingStartedPage,
    interactive_page::InteractivePage, welcome_page::WelcomePage,
};

#[function_component(App)]
pub fn app() -> Html {
    let groups: ComponentList = vec![
        create_component_group!(
            "Overview",
            WelcomePage::preview(),
            FeatureCard::preview()
        ),
        create_component_group!(
            "Getting Started",
            GettingStartedPage::preview(),
            InteractivePage::preview(),
            CodeSnippet::preview()
        ),
        create_component_group!("Documentation", DocPage::preview()),
        create_component_group!(
            "Example Components",
            PropShowcase::preview(),
            Badge::preview(),
            HeaderComp::preview(),
            FooterComp::preview(),
            ImageComp::preview(),
            ProjectComp::preview()
        ),
    ];

    html! {
        <div style="font-family: Arial, sans-serif; height: 100vh; display: flex; flex-direction: column; overflow: hidden;">
            <div style="padding: 8px 20px; background: #24292e; flex-shrink: 0; display: flex; align-items: center; gap: 16px;">
                <span style="color: #fff; font-weight: 700; font-size: 1.1rem;">{ "YewPreview" }</span>
                <span style="color: #8b949e; font-size: 0.85rem;">{ "Interactive component browser for Yew" }</span>
            </div>
            <div style="flex: 1; overflow: hidden;">
                <PreviewPage groups={groups} />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
