use crate::component_item::ComponentItem;
use yew::prelude::{function_component, html, Html, Properties};

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
                <div style="border: 1px solid #ccc; padding: 20px; box-shadow: 0 0 10px rgba(0,0,0,0.1); border-radius: 4px;">
                    <h2 style="margin-top: 0; margin-bottom: 20px; padding-bottom: 10px; border-bottom: 1px solid #eee;">
                        { &item.name }
                    </h2>
                    <div style="padding: 20px; border: 1px solid #eee; border-radius: 4px; background-color: #fff;">
                        <div style="min-height: 100px; display: flex; align-items: center; justify-content: center;">
                            { html.clone() }
                        </div>
                    </div>
                </div>
            }
        } else {
            html! {
                <div style="border: 1px solid #ccc; padding: 20px; box-shadow: 0 0 10px rgba(0,0,0,0.1); border-radius: 4px;">
                    <p>{ "Select a property to preview" }</p>
                </div>
            }
        }
    } else {
        html! {
            <div style="border: 1px solid #ccc; padding: 20px; box-shadow: 0 0 10px rgba(0,0,0,0.1); border-radius: 4px;">
                <p>{ "Select a component to preview" }</p>
            </div>
        }
    }
}
