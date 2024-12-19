use crate::component_list::ComponentList;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct GroupSelectorProps {
    pub groups: ComponentList,
    pub on_select: Callback<usize>,
    #[prop_or_default]
    pub on_component_select: Callback<(usize, usize)>,
}

#[function_component(GroupSelector)]
pub fn group_selector(props: &GroupSelectorProps) -> Html {
    let expanded_groups = use_state(|| vec![false; props.groups.len()]);

    let list_items = props.groups.iter().enumerate().map(|(group_index, group)| {
        let name = group.name.clone();
        let is_expanded = expanded_groups[group_index];

        let toggle_expand = {
            let expanded_groups = expanded_groups.clone();
            Callback::from(move |_| {
                let mut new_expanded = (*expanded_groups).clone();
                new_expanded[group_index] = !new_expanded[group_index];
                expanded_groups.set(new_expanded);
            })
        };

        let on_select = props.on_select.clone();
        let on_component_select = props.on_component_select.clone();

        html! {
            <li key={group_index} style="margin: 5px 0;">
                <div style="display: flex; align-items: center;">
                    <button
                        style="padding: 5px; margin-right: 5px; width: 20px; height: 20px;
                               cursor: pointer; display: flex; align-items: center; justify-content: center;
                               background: none; border: 1px solid #ccc; border-radius: 3px;"
                        onclick={toggle_expand}
                    >
                        { if is_expanded { "−" } else { "+" } }
                    </button>
                    <span
                        style="cursor: pointer; padding: 5px 10px;
                               color: #333; font-weight: bold;"
                        onclick={Callback::from(move |_| on_select.emit(group_index))}
                    >
                        { name }
                    </span>
                </div>
                if is_expanded {
                    <ul style="list-style: none; padding-left: 25px; margin: 5px 0;">
                        { for group.components.iter().enumerate().map(|(comp_index, component)| {
                            let on_component_select = on_component_select.clone();
                            let group_index = group_index;

                            html! {
                                <li key={comp_index} style="margin: 3px 0;">
                                    <span
                                        style="color: #666; padding: 3px 10px;
                                               display: inline-block; cursor: pointer;
                                               border-radius: 3px;
                                               &:hover { background-color: #f0f0f0; }"
                                        onclick={Callback::from(move |_| {
                                            on_component_select.emit((group_index, comp_index));
                                        })}
                                    >
                                        { &component.name }
                                    </span>
                                </li>
                            }
                        })}
                    </ul>
                }
            </li>
        }
    });

    html! {
        <div style="margin-bottom: 20px;">
            <style>
                {"
                    span:hover {
                        background-color: #f0f0f0;
                    }
                "}
            </style>
            <h2>{ "Component Groups" }</h2>
            <ul style="list-style: none; padding: 0;">
                { for list_items }
            </ul>
        </div>
    }
}
