use crate::components::feature_card::{FeatureCard, FeatureCardProps};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct WelcomePageProps {
    pub source_url: AttrValue,
}

#[function_component(WelcomePage)]
pub fn welcome_page(props: &WelcomePageProps) -> Html {
    let features = vec![
        FeatureCardProps {
            icon: AttrValue::from("🚀"),
            title: AttrValue::from("Fast Setup"),
            description: AttrValue::from("Add interactive previews to your Yew components in minutes with zero configuration."),
        },
        FeatureCardProps {
            icon: AttrValue::from("📦"),
            title: AttrValue::from("Macro-Powered"),
            description: AttrValue::from("Concise create_preview! syntax keeps component files clean and self-documenting."),
        },
        FeatureCardProps {
            icon: AttrValue::from("🎯"),
            title: AttrValue::from("Feature-Gated"),
            description: AttrValue::from("Preview code compiles only with the yew-preview feature flag — zero cost in production."),
        },
        FeatureCardProps {
            icon: AttrValue::from("🧪"),
            title: AttrValue::from("Built-in Testing"),
            description: AttrValue::from("Validate component states with integrated matchers via create_preview_with_tests!."),
        },
        FeatureCardProps {
            icon: AttrValue::from("📱"),
            title: AttrValue::from("Interactive UI"),
            description: AttrValue::from("Browse all component variants and states live in an interactive sidebar browser."),
        },
        FeatureCardProps {
            icon: AttrValue::from("🔄"),
            title: AttrValue::from("Quick Iteration"),
            description: AttrValue::from("See changes instantly with Trunk hot reload during development."),
        },
    ];

    html! {
        <div style="padding: 40px 20px; max-width: 900px; margin: 0 auto; font-family: Arial, sans-serif;">
            <div style="text-align: center; margin-bottom: 48px;">
                <h1 style="font-size: 2.5rem; color: #24292e; margin: 0 0 16px 0;">
                    { "YewPreview" }
                </h1>
                <p style="font-size: 1.15rem; color: #586069; max-width: 600px; margin: 0 auto 28px auto; line-height: 1.6;">
                    { "A lightweight Rust library for interactive component previews in Yew — like Storybook, but for Rust." }
                </p>
                <div style="display: flex; gap: 12px; justify-content: center; flex-wrap: wrap;">
                    if !props.source_url.is_empty() {
                        <a href={ props.source_url.clone() }
                           target="_blank"
                           rel="noopener noreferrer"
                           style="display: inline-block; padding: 10px 22px; background: #24292e; color: #fff; border-radius: 6px; text-decoration: none; font-weight: 600; font-size: 0.95rem;">
                            { "View Source" }
                        </a>
                    }
                    <a href="https://github.com/chriamue/yew-preview/tree/main/docs"
                       target="_blank"
                       rel="noopener noreferrer"
                       style="display: inline-block; padding: 10px 22px; border: 2px solid #24292e; color: #24292e; border-radius: 6px; text-decoration: none; font-weight: 600; font-size: 0.95rem;">
                        { "Documentation" }
                    </a>
                </div>
            </div>

            <div style="display: grid; grid-template-columns: repeat(auto-fill, minmax(260px, 1fr)); gap: 16px;">
                { for features.into_iter().map(|f| html! {
                    <FeatureCard icon={f.icon} title={f.title} description={f.description} />
                }) }
            </div>
        </div>
    }
}

mod preview {
    use super::*;
    use yew_preview::prelude::*;

    const SOURCE_URL: &str = match option_env!("YEWPREVIEW_SOURCE_URL") {
        Some(url) => url,
        None => "https://github.com/chriamue/yew-preview",
    };

    yew_preview::create_preview!(
        WelcomePage,
        WelcomePageProps {
            source_url: AttrValue::Static(SOURCE_URL),
        },
        (
            "No Source URL",
            WelcomePageProps {
                source_url: AttrValue::from(""),
            }
        )
    );
}
