use crate::{create_preview, prelude::*};
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

create_preview!(
    FooterComp,
    FooterCompProps {
        copyright: "© 2021".to_string()
    },
);
