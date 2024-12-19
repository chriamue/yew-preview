use crate::component_list::ComponentList;
use crate::component_preview::ComponentPreview;
use crate::component_selector::ComponentSelector;
use crate::config_panel::ConfigPanel;
use crate::group_selector::GroupSelector;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct PreviewPageProps {
    pub groups: ComponentList,
}

#[derive(Clone, PartialEq)]
struct SelectedComponent {
    group_index: usize,
    component_index: usize,
}

#[function_component(PreviewPage)]
pub fn preview_page(props: &PreviewPageProps) -> Html {
    let groups = use_state(|| props.groups.clone());
    let selected_group = use_state(|| None::<usize>);
    let selected_component = use_state(|| None::<SelectedComponent>);
    let selected_property = use_state(|| None::<String>);

    let on_group_select = {
        let selected = selected_group.clone();
        Callback::from(move |index| {
            selected.set(Some(index));
        })
    };

    let on_component_select = {
        let groups = groups.clone();
        let selected_group = selected_group.clone();
        let selected = selected_component.clone();
        let selected_property = selected_property.clone();
        Callback::from(move |comp_index| {
            if let Some(group_index) = *selected_group {
                selected.set(Some(SelectedComponent {
                    group_index,
                    component_index: comp_index,
                }));

                let component = &groups[group_index].components[comp_index];
                if !component.render.is_empty() {
                    selected_property.set(Some(component.render[0].0.clone()));
                } else {
                    selected_property.set(None);
                }
            }
        })
    };

    let on_property_select = {
        let selected = selected_property.clone();
        Callback::from(move |prop| selected.set(Some(prop)))
    };

    let current_properties = selected_component
        .as_ref()
        .map(|selected| {
            groups[selected.group_index].components[selected.component_index]
                .render
                .clone()
        })
        .unwrap_or_default();

    let current_component = selected_component
        .as_ref()
        .map(|selected| groups[selected.group_index].components[selected.component_index].clone());

    html! {
        <div style="display: flex; height: 100vh; flex-direction: column;">
            <div style="display: flex; flex: 1;">
                <div style="flex: 0 0 200px; padding: 20px; border-right: 1px solid #ccc;">
                    <GroupSelector groups={(*groups).clone()} on_select={on_group_select.clone()} />
                    {
                        if let Some(group_index) = *selected_group {
                            html! {
                                <ComponentSelector
                                    group={groups[group_index].clone()}
                                    on_select={on_component_select.clone()}
                                />
                            }
                        } else {
                            html! {}
                        }
                    }
                </div>
                <div style="flex: 1; padding: 20px;">
                    <ComponentPreview item={current_component} selected_property={(*selected_property).clone()} />
                </div>
            </div>
            <div style="flex: 0 0 200px; padding: 20px; border-top: 1px solid #ccc;">
                <ConfigPanel properties={current_properties} on_select={on_property_select} />
            </div>
        </div>
    }
}
