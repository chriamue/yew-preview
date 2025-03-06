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
                <div class="yew-preview-component" style="border: 1px solid #ccc; padding: 20px; box-shadow: 0 0 10px rgba(0,0,0,0.1); border-radius: 4px;">
                    <h2 style="margin-top: 0; margin-bottom: 20px; padding-bottom: 10px; border-bottom: 1px solid #eee;">
                        { &item.name }
                    </h2>
                    <div class="yew-preview-component-content" style="padding: 20px; border: 1px solid #eee; border-radius: 4px; background-image: linear-gradient(45deg, #eee 25%, transparent 25%, transparent 75%, #eee 75%), linear-gradient(45deg, #eee 25%, transparent 25%, transparent 75%, #eee 75%); background-size: 20px 20px; background-position: 0 0, 10px 10px;">
                        <div style="min-height: 100px; display: flex; align-items: center; justify-content: center; background-color: #eee;">
                            { html.clone() }
                        </div>
                    </div>
                </div>
            }
        } else {
            html! {
                <div class="yew-preview-component" style="border: 1px solid #ccc; padding: 20px; box-shadow: 0 0 10px rgba(0,0,0,0.1); border-radius: 4px;">
                    <p>{ "Select a property to preview" }</p>
                </div>
            }
        }
    } else {
        html! {
            <div class="yew-preview-component" style="border: 1px solid #ccc; padding: 20px; box-shadow: 0 0 10px rgba(0,0,0,0.1); border-radius: 4px;">
                <p>{ "Select a component to preview" }</p>
            </div>
        }
    }
}
