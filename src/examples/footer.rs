use crate::{create_preview_with_tests, prelude::*};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct FooterCompProps {
    pub copyright: String,
}

#[function_component(FooterComp)]
pub fn footer_comp(props: &FooterCompProps) -> Html {
    html! {
        <footer style="border-top: 1px solid #ccc; padding: 10px;">
            <p>{ &props.copyright }</p>
        </footer>
    }
}

create_preview_with_tests!(
    component: FooterComp,
    default_props: FooterCompProps {
        copyright: "© 2021".to_string()
    },
    variants: [
        (
            "Current Year",
            FooterCompProps {
                copyright: "© 2024".to_string()
            }
        ),
        (
            "With Company",
            FooterCompProps {
                copyright: "© 2024 My Company".to_string()
            }
        ),
        (
            "Empty",
            FooterCompProps {
                copyright: "".to_string()
            }
        )
    ],
    tests: [
        (
            "Has footer element",
            exists("footer"),
        ),
        (
            "Has paragraph element",
            exists("p"),
        ),
        (
            "Has correct border style",
            has_style("border-top", "1px solid #ccc"),
        ),
        (
            "Has correct padding",
            has_style("padding", "10px"),
        ),
        (
            "Contains copyright symbol",
            has_text("©"),
        ),
        (
            "Default copyright text is present",
            has_text("© 2021"),
        ),
    ]
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_component_test;
    use crate::test_utils::TestCase;

    generate_component_test!(
        tokio,
        FooterComp,
        tokio_test_footer_comp_rendering,
        FooterCompProps {
            copyright: "© 2024 Test Company".to_string(),
        },
        vec![
            TestCase {
                name: "Has copyright text".to_string(),
                matchers: vec![has_text("© 2024 Test Company")],
            },
            TestCase {
                name: "Has footer element".to_string(),
                matchers: vec![exists("footer")],
            },
            TestCase {
                name: "Has paragraph element".to_string(),
                matchers: vec![exists("p")],
            },
            TestCase {
                name: "Has correct styles".to_string(),
                matchers: vec![
                    has_style("border-top", "1px solid #ccc"),
                    has_style("padding", "10px"),
                ],
            },
        ]
    );

    generate_component_test!(
        tokio,
        FooterComp,
        tokio_test_footer_comp_empty,
        FooterCompProps {
            copyright: "".to_string(),
        },
        vec![TestCase {
            name: "Renders with empty copyright".to_string(),
            matchers: vec![exists("footer"), exists("p"), has_text(""),],
        },]
    );
}
