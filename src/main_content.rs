use crate::component_item::ComponentItem;
use crate::component_preview::ComponentPreview;
use crate::config_panel::ConfigPanel;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct MainContentProps {
    pub current_component: Option<ComponentItem>,
    pub selected_property: String,
    pub properties: Vec<(String, Html)>,
    pub on_property_select: Callback<String>,
}

#[function_component(MainContent)]
pub fn main_content(props: &MainContentProps) -> Html {
    html! {
        <div style="flex: 1; display: flex; flex-direction: column; overflow: hidden;">
            <div style="flex: 1; overflow-y: auto; padding: 20px;">
                <div style="width: 100%; max-width: 1200px; margin: 0 auto;">
                    <ComponentPreview
                        item={props.current_component.clone()}
                        selected_property={props.selected_property.clone()}
                    />
                </div>
            </div>
            <div style="border-top: 1px solid #ccc; background-color: #f8f8f8; padding: 20px; display: flex; justify-content: center;">
                <div style="width: 100%; max-width: 600px;">
                    <ConfigPanel
                        properties={props.properties.clone()}
                        on_select={props.on_property_select.clone()}
                    />
                </div>
            </div>
        </div>
    }
}
