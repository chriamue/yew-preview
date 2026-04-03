pub mod components;
pub mod pages;

use yew_preview::create_component_group;
use yew_preview::prelude::*;

use components::{
    badge::Badge, button::Button, card::CardComp, code_snippet::CodeSnippet,
    feature_card::FeatureCard, footer::FooterComp, header::HeaderComp, image::ImageComp,
    project::ProjectComp, prop_showcase::PropShowcase,
};
use pages::{
    doc_page::DocPage, getting_started_page::GettingStartedPage,
    interactive_page::InteractivePage, serve_page::ServePage, welcome_page::WelcomePage,
};

pub fn preview_groups() -> ComponentList {
    vec![
        create_component_group!("Overview", WelcomePage::preview(), FeatureCard::preview()),
        create_component_group!(
            "Getting Started",
            GettingStartedPage::preview(),
            InteractivePage::preview(),
            CodeSnippet::preview()
        ),
        create_component_group!("Documentation", DocPage::preview()),
        create_component_group!("Native Server", ServePage::preview()),
        create_component_group!(
            "Example Components",
            PropShowcase::preview(),
            Badge::preview(),
            Button::preview(),
            HeaderComp::preview(),
            FooterComp::preview(),
            ImageComp::preview(),
            ProjectComp::preview()
        ),
        create_component_group!("Failing Tests Demo", CardComp::preview()),
    ]
}
