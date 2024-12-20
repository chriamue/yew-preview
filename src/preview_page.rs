use crate::component_item::ComponentItem;
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

    let on_single_component_select = {
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

    let on_tree_component_select = {
        let groups = groups.clone();
        let selected = selected_component.clone();
        let selected_group = selected_group.clone();
        let selected_property = selected_property.clone();
        Callback::from(move |(group_index, comp_index)| {
            selected_group.set(Some(group_index));
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

    fn render_group_selector(
        groups: &ComponentList,
        on_group_select: &Callback<usize>,
        on_tree_component_select: &Callback<(usize, usize)>,
        selected_group: Option<usize>,
        on_single_component_select: &Callback<usize>,
    ) -> Html {
        html! {
            <div style="
                flex: 0 0 250px;
                border-right: 1px solid #ccc;
                overflow-y: auto;
                padding: 20px;
                background-color: #f8f8f8;
            ">
                <GroupSelector
                    groups={groups.clone()}
                    on_select={on_group_select.clone()}
                    on_component_select={on_tree_component_select.clone()}
                />
                {
                    if let Some(group_index) = selected_group {
                        html! {
                            <ComponentSelector
                                group={groups[group_index].clone()}
                                on_select={on_single_component_select.clone()}
                            />
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    }

    fn render_preview_area(
        current_component: Option<ComponentItem>,
        selected_property: Option<String>,
    ) -> Html {
        html! {
            <div style="flex: 1; overflow-y: auto; padding: 20px;">
                <div style="width: 100%; max-width: 800px; margin: 0 auto;">
                    <ComponentPreview
                        item={current_component}
                        selected_property={selected_property}
                    />
                </div>
            </div>
        }
    }

    fn render_config_panel(
        properties: Vec<(String, Html)>,
        on_property_select: Callback<String>,
    ) -> Html {
        html! {
            <div style="
                border-top: 1px solid #ccc;
                background-color: #f8f8f8;
                padding: 20px;
                display: flex;
                justify-content: center;
            ">
                <div style="width: 100%; max-width: 600px;">
                    <ConfigPanel
                        properties={properties}
                        on_select={on_property_select}
                    />
                </div>
            </div>
        }
    }

    fn render_main_content(
        current_component: Option<ComponentItem>,
        selected_property: Option<String>,
        properties: Vec<(String, Html)>,
        on_property_select: Callback<String>,
    ) -> Html {
        html! {
            <div style="
                flex: 1;
                display: flex;
                flex-direction: column;
                overflow: hidden;
            ">
                { render_preview_area(current_component, selected_property) }
                { render_config_panel(properties, on_property_select) }
            </div>
        }
    }

    html! {
        <div style="display: flex; height: 100%; max-height: 100%; overflow: hidden;">
            { render_group_selector(
                &groups,
                &on_group_select,
                &on_tree_component_select,
                *selected_group,
                &on_single_component_select
            ) }
            { render_main_content(
                current_component,
                (*selected_property).clone(),
                current_properties,
                on_property_select
            ) }
        </div>
    }
}
