use crate::ComponentItem;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ComponentPreviewProps {
    pub item: Option<ComponentItem>,
}

#[function_component(ComponentPreview)]
pub fn component_preview(props: &ComponentPreviewProps) -> Html {
    if let Some(item) = &props.item {
        html! {
            <div>
                <h2>{ &item.name }</h2>
                { item.render.clone() }
                <p>{ format!("{:?}", item.props) }</p>
            </div>
        }
    } else {
        html! { <div></div> }
    }
}
