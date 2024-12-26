use crate::test_utils::{exists, has_style, has_text};
use crate::{create_preview_with_tests, prelude::*};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct HeaderCompProps {
    pub title: String,
}

#[function_component(HeaderComp)]
pub fn header_comp(props: &HeaderCompProps) -> Html {
    html! {
        <header style="border-bottom: 1px solid #ccc; padding: 10px;">
            <h1>{ &props.title }</h1>
        </header>
    }
}

create_preview_with_tests!(
    component: HeaderComp,
    default_props: HeaderCompProps {
        title: "Default Header".to_string()
    },
    variants: [
        (
            "Hello",
            HeaderCompProps {
                title: "Hello, World!".to_string()
            }
        ),
        (
            "Goodbye",
            HeaderCompProps {
                title: "Goodbye, World!".to_string()
            }
        )
    ],
    tests: [
        (
            "Has header element",
            exists("header"),
        ),
        (
            "Has h1 element",
            exists("h1"),
        ),
        (
            "Has correct border style",
            has_style("border-bottom", "1px solid #ccc"),
        ),
        (
            "Has correct padding",
            has_style("padding", "10px"),
        ),
        (
            "Default title is present",
            has_text("Default Header"),
        ),
    ]
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_component_test;

    generate_component_test!(
        tokio,
        HeaderComp,
        HeaderCompProps {
            title: "Test Title".to_string(),
        },
        vec![
            TestCase {
                name: "Has title text".to_string(),
                matchers: vec![has_text("Test Title")],
            },
            TestCase {
                name: "Has header element".to_string(),
                matchers: vec![exists("header")],
            },
            TestCase {
                name: "Has correct styles".to_string(),
                matchers: vec![
                    has_style("border-bottom", "1px solid #ccc"),
                    has_style("padding", "10px"),
                ],
            },
        ]
    );
}
