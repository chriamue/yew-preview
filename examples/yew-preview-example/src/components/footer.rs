use yew::prelude::*;
use yew_preview::test_utils::{exists, has_style};
use yew_preview::{create_preview_with_tests, prelude::*};

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
        ("Has footer element", exists("footer")),
        ("Has paragraph element", exists("p")),
        ("Has correct border style", has_style("border-top", "1px solid #ccc")),
        ("Has correct padding", has_style("padding", "10px")),
    ]
);
