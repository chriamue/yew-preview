use crate::component_preview::ComponentPreview;
use crate::component_selector::ComponentSelector;
use crate::config_panel::ConfigPanel;
use crate::ComponentList;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct PreviewPageProps {
    pub components: ComponentList,
}

#[function_component(PreviewPage)]
pub fn preview_page(props: &PreviewPageProps) -> Html {
    let components = use_state(|| props.components.clone());
    let selected_component = use_state(|| Some(0));
    let selected_property = use_state(|| {
        if let Some(index) = *selected_component {
            if !components[index].render.is_empty() {
                Some(components[index].render[0].0.clone())
            } else {
                None
            }
        } else {
            None
        }
    });

    let on_component_select = {
        let components = components.clone();
        let selected = selected_component.clone();
        let selected_property = selected_property.clone();
        Callback::from(move |index| {
            selected.set(Some(index));
            if !components[index].render.is_empty() {
                selected_property.set(Some(components[index].render[0].0.clone()));
            } else {
                selected_property.set(None);
            }
        })
    };

    let on_property_select = {
        let selected = selected_property.clone();
        Callback::from(move |prop| selected.set(Some(prop)))
    };

    let current_properties = selected_component
        .and_then(|index| Some(components[index].render.clone()))
        .unwrap_or_else(Vec::new);

    html! {
        <div>
            <ComponentSelector components={(*components).clone()} on_select={on_component_select} />
            <ComponentPreview item={selected_component.map(|index| components[index].clone())} selected_property={(*selected_property).clone()} />
            <ConfigPanel properties={current_properties} on_select={on_property_select} />
        </div>
    }
}
