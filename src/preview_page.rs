use crate::component_preview::ComponentPreview;
use crate::component_selector::ComponentSelector;
use crate::ComponentList;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct PreviewPageProps {
    pub components: ComponentList,
}

#[function_component(PreviewPage)]
pub fn preview_page(props: &PreviewPageProps) -> Html {
    let components = props.components.clone();
    let selected = use_state(|| 0);

    let on_select = {
        let selected = selected.clone();
        Callback::from(move |index| selected.set(index))
    };

    html! {
        <div>
            <ComponentSelector components={components.clone()} on_select={on_select} />
            <ComponentPreview item={components[*selected].clone()} />
        </div>
    }
}
