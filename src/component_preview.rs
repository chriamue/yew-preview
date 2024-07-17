use crate::ComponentItem;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ComponentPreviewProps {
    pub item: ComponentItem,
}

#[function_component(ComponentPreview)]
pub fn component_preview(props: &ComponentPreviewProps) -> Html {
    let ComponentItem {
        name,
        render,
        props,
    } = &props.item;
    html! {
        <div>
            <h2>{ name }</h2>
            { render.clone() }
            <p>{ props }</p>
        </div>
    }
}
