use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct FeatureCardProps {
    pub icon: AttrValue,
    pub title: AttrValue,
    pub description: AttrValue,
}

#[function_component(FeatureCard)]
pub fn feature_card(props: &FeatureCardProps) -> Html {
    html! {
        <div style="border: 1px solid #e1e4e8; border-radius: 8px; padding: 20px; background: #fff; box-shadow: 0 1px 3px rgba(0,0,0,0.08);">
            <div style="font-size: 2rem; margin-bottom: 12px;">{ &props.icon }</div>
            <h3 style="margin: 0 0 8px 0; font-size: 1.1rem; color: #24292e;">{ &props.title }</h3>
            <p style="margin: 0; color: #586069; font-size: 0.9rem; line-height: 1.5;">{ &props.description }</p>
        </div>
    }
}

mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        FeatureCard,
        FeatureCardProps {
            icon: AttrValue::from("🚀"),
            title: AttrValue::from("Fast Setup"),
            description: AttrValue::from("Add previews directly to your component files with minimal boilerplate."),
        },
        (
            "Macro-Powered",
            FeatureCardProps {
                icon: AttrValue::from("📦"),
                title: AttrValue::from("Macro-Powered"),
                description: AttrValue::from("Concise create_preview! macro syntax keeps your code clean and self-documenting."),
            }
        ),
        (
            "Feature-Gated",
            FeatureCardProps {
                icon: AttrValue::from("🎯"),
                title: AttrValue::from("Feature-Gated"),
                description: AttrValue::from("Optional preview code compiled only with the yew-preview feature flag."),
            }
        ),
        (
            "Built-in Testing",
            FeatureCardProps {
                icon: AttrValue::from("🧪"),
                title: AttrValue::from("Built-in Testing"),
                description: AttrValue::from("Validate component states with integrated matchers via create_preview_with_tests!."),
            }
        ),
        (
            "Interactive UI",
            FeatureCardProps {
                icon: AttrValue::from("📱"),
                title: AttrValue::from("Interactive UI"),
                description: AttrValue::from("Browse all component variants and states live in an interactive sidebar UI."),
            }
        ),
        (
            "Quick Iteration",
            FeatureCardProps {
                icon: AttrValue::from("🔄"),
                title: AttrValue::from("Quick Iteration"),
                description: AttrValue::from("See changes instantly with Trunk hot reload during development."),
            }
        )
    );
}
