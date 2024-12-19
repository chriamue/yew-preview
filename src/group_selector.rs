use crate::component_list::ComponentList;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct GroupSelectorProps {
    pub groups: ComponentList,
    pub on_select: Callback<usize>,
}

#[function_component(GroupSelector)]
pub fn group_selector(props: &GroupSelectorProps) -> Html {
    let list_items = props.groups.iter().enumerate().map(|(index, group)| {
        let name = group.name.clone();
        let onclick = {
            let on_select = props.on_select.clone();
            Callback::from(move |_| on_select.emit(index))
        };
        html! {
            <li style="margin: 5px 0;">
                <button style="padding: 10px 15px; cursor: pointer;" onclick={onclick}>{ name }</button>
            </li>
        }
    });

    html! {
        <div style="margin-bottom: 20px;">
            <h2>{ "Select a Group" }</h2>
            <ul style="list-style: none; padding: 0;">
                { for list_items }
            </ul>
        </div>
    }
}
