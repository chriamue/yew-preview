use crate::ComponentList;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SelectionProps {
    pub components: ComponentList,
    pub on_select: Callback<usize>,
}

#[function_component(ComponentSelector)]
pub fn component_selector(props: &SelectionProps) -> Html {
    let on_select = props.on_select.clone();

    html! {
        <div>
            <h2>{ "Select a Component" }</h2>
            <ul>
                { for props.components.iter().enumerate().map(|(index, component)| {
                    let name = component.name.clone();
                    let onclick = {
                        let on_select = on_select.clone();
                        Callback::from(move |_| on_select.emit(index))
                    };
                    html! { <li><button onclick={onclick}>{ name }</button></li> }
                }) }
            </ul>
        </div>
    }
}
