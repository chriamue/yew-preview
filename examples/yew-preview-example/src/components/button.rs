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

#[cfg(test)]
mod tests {
    use super::*;
    use yew_preview::prelude::*;
    use yew_preview::test_utils::{exists, has_text};

    /// Example of a failing test.
    /// Run with: cargo test button_has_icon -- --ignored
    #[tokio::test]
    #[ignore = "demonstrates failing test output"]
    async fn button_has_icon() {
        let html = yew_preview::test_utils::render_component::<Button>(ButtonProps {
            label: "Click me".to_string(),
            disabled: false,
        })
        .await;

        let mut tc = TestCase::new("Has icon element");
        tc.matchers.push(exists("svg"));
        tc.matchers.push(has_text("icon"));

        let result = tc.run(&html);
        if !result.passed {
            let failures = result
                .matchers
                .iter()
                .filter(|m| !m.passed)
                .map(|m| format!("    ✗ {}", m.description))
                .collect::<Vec<_>>()
                .join("\n");
            panic!("[Button] Test '{}' failed:\n{}", tc.name, failures);
        }
    }
}
