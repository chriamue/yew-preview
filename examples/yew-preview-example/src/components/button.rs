use yew::prelude::*;
use yew_preview::test_utils::exists;
use yew_preview::{create_preview_with_tests, prelude::*};

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    pub label: String,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button disabled={props.disabled} style="padding: 8px 16px; border-radius: 4px; border: 1px solid #ccc; cursor: pointer;">
            { &props.label }
        </button>
    }
}

create_preview_with_tests!(
    component: Button,
    default_props: ButtonProps {
        label: "Click me".to_string(),
        disabled: false,
    },
    variants: [
        ("Disabled", ButtonProps { label: "Disabled".to_string(), disabled: true }),
        ("Submit",   ButtonProps { label: "Submit".to_string(),   disabled: false }),
    ],
    tests: [
        ("Has button element", exists("button")),
    ]
);
