/// Demonstrates a failing test. The component renders correctly but the
/// "Has close button" test checks for a `button` element that isn't implemented
/// yet — this is intentional, to show how failing tests appear in both the
/// browser UI and `cargo test` output.
use yew::prelude::*;
use yew_preview::test_utils::exists;
use yew_preview::{create_preview_with_tests, prelude::*};

#[derive(Properties, PartialEq, Clone)]
pub struct CardProps {
    pub title: String,
    pub body: String,
}

#[function_component(CardComp)]
pub fn card_comp(props: &CardProps) -> Html {
    html! {
        <div style="border: 1px solid #ddd; border-radius: 8px; padding: 16px; max-width: 400px;">
            <h2 style="margin: 0 0 8px 0; font-size: 1rem;">{ &props.title }</h2>
            <p style="margin: 0; color: #555;">{ &props.body }</p>
            // TODO: add a close button
        </div>
    }
}

create_preview_with_tests!(
    component: CardComp,
    default_props: CardProps {
        title: "Hello".to_string(),
        body: "This card renders correctly.".to_string(),
    },
    variants: [
        ("Warning", CardProps {
            title: "Warning".to_string(),
            body: "Something needs your attention.".to_string(),
        }),
    ],
    tests: [
        ("Has card element",  exists("div")),
        ("Has heading",       exists("h2")),
        ("Has close button",  exists("button")),   // intentionally fails — not yet implemented
    ]
);
