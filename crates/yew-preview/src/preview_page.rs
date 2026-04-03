use crate::component_list::ComponentList;
use crate::component_preview::ComponentPreview;
use crate::component_selector::ComponentSelector;
use crate::config_panel::ConfigPanel;
use crate::group_selector::GroupSelector;
use crate::interactive::ArgValue;
use crate::search_bar::SearchBar;
use crate::search_results::SearchResults;
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
    let is_sidebar_visible = use_state(|| true);
    let search_query = use_state(String::new);
    let live_args = use_state(Vec::<(String, ArgValue)>::new);

    let on_search = {
        let search_query = search_query.clone();
        Callback::from(move |query: String| {
            search_query.set(query);
        })
    };

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
        let live_args = live_args.clone();
        Callback::from(move |comp_index| {
            if let Some(group_index) = *selected_group {
                selected.set(Some(SelectedComponent {
                    group_index,
                    component_index: comp_index,
                }));

                let component = &groups[group_index].components[comp_index];
                if let Some(args) = &component.args {
                    live_args.set(args.values.clone());
                } else {
                    live_args.set(vec![]);
                }
                if !component.render.is_empty() {
                    selected_property.set(Some(component.render[0].0.clone()));
                } else if component.args.is_some() {
                    selected_property.set(Some("Interactive".to_string()));
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
        let live_args = live_args.clone();
        Callback::from(move |(group_index, comp_index)| {
            selected_group.set(Some(group_index));
            selected.set(Some(SelectedComponent {
                group_index,
                component_index: comp_index,
            }));

            let component = &groups[group_index].components[comp_index];
            if let Some(args) = &component.args {
                live_args.set(args.values.clone());
            } else {
                live_args.set(vec![]);
            }
            if !component.render.is_empty() {
                selected_property.set(Some(component.render[0].0.clone()));
            } else if component.args.is_some() {
                selected_property.set(Some("Interactive".to_string()));
            } else {
                selected_property.set(None);
            }
        })
    };

    let on_property_select = {
        let selected = selected_property.clone();
        Callback::from(move |prop| selected.set(Some(prop)))
    };

    let on_arg_change = {
        let live_args = live_args.clone();
        Callback::from(move |(key, val): (String, ArgValue)| {
            let mut updated = (*live_args).clone();
            if let Some(entry) = updated.iter_mut().find(|(k, _)| k == &key) {
                entry.1 = val;
            }
            live_args.set(updated);
        })
    };

    let toggle_sidebar = {
        let is_sidebar_visible = is_sidebar_visible.clone();
        Callback::from(move |_| {
            is_sidebar_visible.set(!*is_sidebar_visible);
        })
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

    let is_interactive = current_component
        .as_ref()
        .map(|c| c.args.is_some())
        .unwrap_or(false);

    #[allow(clippy::too_many_arguments)]
    fn render_sidebar(
        groups: &ComponentList,
        on_group_select: &Callback<usize>,
        on_tree_component_select: &Callback<(usize, usize)>,
        selected_group: Option<usize>,
        on_single_component_select: &Callback<usize>,
        is_visible: bool,
        search_query: String,
        on_search: Callback<String>,
        toggle_sidebar: Callback<MouseEvent>,
    ) -> Html {
        let toggle_label = if is_visible { "◀" } else { "▶" };

        html! {
            <div style="
                display: flex;
                flex-direction: column;
                flex: 0 0 auto;
                border-right: 1px solid #e1e4e8;
                background: #f6f8fa;
                overflow: hidden;
            ">
                <div style="
                    display: flex;
                    align-items: center;
                    gap: 8px;
                    padding: 8px 12px;
                    border-bottom: 1px solid #e1e4e8;
                    flex-shrink: 0;
                ">
                    <button
                        onclick={toggle_sidebar}
                        title={if is_visible { "Hide sidebar" } else { "Show sidebar" }}
                        style="
                            padding: 4px 8px;
                            border: 1px solid #d0d7de;
                            border-radius: 4px;
                            background: #fff;
                            color: #24292e;
                            cursor: pointer;
                            font-size: 0.85rem;
                            line-height: 1;
                        "
                    >
                        { toggle_label }
                    </button>
                    if is_visible {
                        <span style="font-size: 0.8rem; color: #57606a; font-weight: 600; letter-spacing: 0.02em; text-transform: uppercase;">
                            { "Components" }
                        </span>
                    }
                </div>

                if is_visible {
                    <div style="width: 250px; flex: 1; overflow-y: auto; padding: 12px;">
                        <SearchBar
                            on_search={on_search}
                            placeholder="Search components..."
                        />

                        <GroupSelector
                            groups={groups.clone()}
                            on_select={on_group_select.clone()}
                            on_component_select={on_tree_component_select.clone()}
                        />

                        <SearchResults
                            groups={groups.clone()}
                            search_query={search_query}
                            on_select={on_tree_component_select.clone()}
                        />

                        {
                            if let Some(group_index) = selected_group {
                                html! {
                                    <div style="margin-top: 16px;">
                                        <ComponentSelector
                                            group={groups[group_index].clone()}
                                            on_select={on_single_component_select.clone()}
                                        />
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                }
            </div>
        }
    }

    html! {
        <div style="display: flex; height: 100%; overflow: hidden;">
            { render_sidebar(
                    &groups,
                    &on_group_select,
                    &on_tree_component_select,
                    *selected_group,
                    &on_single_component_select,
                    *is_sidebar_visible,
                    (*search_query).clone(),
                    on_search.clone(),
                    toggle_sidebar,
                )
            }
            <div style="
                flex: 1;
                display: flex;
                flex-direction: column;
                overflow: hidden;
            ">
                <div style="flex: 1; overflow-y: auto; padding: 20px;">
                    <div style="width: 100%; max-width: 1200px; margin: 0 auto;">
                        <ComponentPreview
                            item={current_component}
                            selected_property={(*selected_property).clone()}
                            live_args={(*live_args).clone()}
                        />
                    </div>
                </div>
                <div style="
                    border-top: 1px solid #e1e4e8;
                    background: #f6f8fa;
                    padding: 10px 16px;
                    flex-shrink: 0;
                ">
                    <ConfigPanel
                        properties={current_properties}
                        selected={(*selected_property).clone()}
                        on_select={on_property_select}
                        live_args={if is_interactive { Some((*live_args).clone()) } else { None }}
                        on_arg_change={on_arg_change}
                    />
                </div>
            </div>
        </div>
    }
}
