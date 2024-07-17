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
        <div style="margin-bottom: 20px;">
            <h2>{ "Select a Component" }</h2>
            <ul style="list-style: none; padding: 0;">
                { for props.components.iter().enumerate().map(|(index, component)| {
                    let name = component.name.clone();
                    let onclick = {
                        let on_select = on_select.clone();
                        Callback::from(move |_| on_select.emit(index))
                    };
                    html! {
                        <li style="margin: 5px 0;">
                            <button style="padding: 10px 15px; cursor: pointer;" onclick={onclick}>{ name }</button>
                        </li>
                    }
                }) }
            </ul>
        </div>
    }
}
