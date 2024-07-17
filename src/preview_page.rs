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
    let components = props.components.clone();
    let selected_component = use_state(|| Option::<usize>::None);
    let selected_property = use_state(|| Option::<String>::None);

    let on_component_select = {
        let selected = selected_component.clone();
        Callback::from(move |index| selected.set(Some(index)))
    };

    let on_property_select = {
        let selected = selected_property.clone();
        Callback::from(move |prop| selected.set(Some(prop)))
    };

    html! {
        <div>
            <ComponentSelector components={components.clone()} on_select={on_component_select} />
            <ComponentPreview item={(*selected_component).map(|index| components[index].clone())} />
            <ConfigPanel properties={(*selected_component).map(|index| components[index].props.clone()).unwrap_or_default()}
                on_select={on_property_select} />
        </div>
    }
}
