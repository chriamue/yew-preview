use crate::ComponentItem;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ComponentPreviewProps {
    pub item: Option<ComponentItem>,
    pub selected_property: Option<String>,
}

#[function_component(ComponentPreview)]
pub fn component_preview(props: &ComponentPreviewProps) -> Html {
    if let Some(item) = &props.item {
        if let Some((_, html)) = item
            .render
            .iter()
            .find(|(name, _)| Some(name) == props.selected_property.as_ref())
        {
            html! {
                <div style="border: 1px solid #ccc; width: 100%; box-shadow: 0 0 10px rgba(0,0,0,0.1);">
                    <h2>{ &item.name }</h2>
                    { html.clone() }
                </div>
            }
        } else {
            html! { <div></div> }
        }
    } else {
        html! { <div></div> }
    }
}
