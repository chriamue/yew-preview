use crate::component_item::ComponentItem;
use crate::component_list::ComponentList;
use crate::group_selector::GroupSelector;
use crate::search_bar::SearchBar;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SidebarProps {
    pub groups: ComponentList,
    pub selected_group: Option<usize>,
    pub is_sidebar_visible: bool,
    pub search_query: String,
    pub on_group_select: Callback<usize>,
    pub on_tree_component_select: Callback<(usize, usize)>,
    pub on_single_component_select: Callback<usize>,
    pub toggle_sidebar: Callback<MouseEvent>,
    pub on_search: Callback<String>,
}
#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let all_components: Vec<(usize, usize, ComponentItem)> = {
        let mut components = Vec::new();
        for (group_index, group) in props.groups.iter().enumerate() {
            for (component_index, component) in group.components.iter().enumerate() {
                components.push((group_index, component_index, component.clone()));
            }
        }
        components
    };

    let on_search_props = props.on_search.clone();

    html! {
        <div style={get_sidebar_style(props.is_sidebar_visible)}>
            {render_toggle_button(props.is_sidebar_visible, props.toggle_sidebar.clone())}
            <div class="yew-preview-sidebar-content">
                <SearchBar on_search={on_search_props} />
                {render_group_selector(
                    &props.groups,
                    &props.on_group_select.clone(),
                    &props.on_tree_component_select.clone()
                )}
                {render_component_selector(
                    &props.groups,
                    props.selected_group,
                    &props.on_tree_component_select,
                    props.search_query.clone(),
                    &all_components
                )}
            </div>
        </div>
    }
}

fn get_sidebar_style(is_visible: bool) -> String {
    format!(
        "flex: 0 0 250px; border-right: 1px solid #ccc; overflow-y: auto; background-color: #f8f8f8; display: {};",
        if is_visible { "block" } else { "none" }
    )
}

fn render_toggle_button(is_sidebar_visible: bool, onclick: Callback<MouseEvent>) -> Html {
    html! {
        <button
            onclick={onclick}
            style="
                            position: fixed;
                            top: 20px;
                            left: 20px;
                            z-index: 1000;
                            padding: 8px 12px;
                            border: none;
                            border-radius: 4px;
                            background-color: #007bff;
                            color: white;
                            cursor: pointer;
                            transition: background-color 0.2s;
                        "
        >
            {
                if is_sidebar_visible {
                    "Hide Sidebar"
                } else {
                    "Show Sidebar"
                }
            }
        </button>
    }
}

fn render_group_selector(
    groups: &ComponentList,
    on_group_select: &Callback<usize>,
    on_tree_component_select: &Callback<(usize, usize)>,
) -> Html {
    html! {
        <div class="yew-preview-sidebar-section" style="margin-bottom: 20px;">
            <GroupSelector
                groups={groups.clone()}
                on_select={on_group_select.clone()}
                on_component_select={on_tree_component_select.clone()}
            />
        </div>
    }
}

fn render_component_selector(
    groups: &ComponentList,
    selected_group: Option<usize>,
    on_tree_component_select: &Callback<(usize, usize)>,
    search_query: String,
    all_components: &Vec<(usize, usize, ComponentItem)>,
) -> Html {
    let mut all_rendered_components: Vec<Html> = Vec::new();

    // First, add filtered components if there's a search query
    if !search_query.is_empty() {
        let filtered_components: Vec<_> = all_components
            .iter()
            .filter(|(_, _, component)| {
                component
                    .name
                    .to_lowercase()
                    .contains(&search_query.to_lowercase())
            })
            .collect();

        // Add a section header for search results if there are any
        if !filtered_components.is_empty() {
            all_rendered_components.push(html! {
                <li class="yew-preview-property-item" style="margin: 10px 0;">
                    <h3>{ "Search Results" }</h3>
                </li>
            });
        }

        // Add filtered components
        for (group_index, component_index, component) in filtered_components.iter() {
            let on_select = {
                let on_tree_component_select = on_tree_component_select.clone();
                let group_index = *group_index;
                let component_index = *component_index;
                Callback::from(move |_| {
                    on_tree_component_select.emit((group_index, component_index))
                })
            };

            all_rendered_components.push(html! {
                <li class="yew-preview-property-item" style="margin: 5px 0;">
                    <button style="padding: 10px 15px; cursor: pointer;" onclick={on_select}>
                        {&component.name}
                        <span style="color: #666; font-size: 0.8em; margin-left: 8px;">
                            {format!("({})", groups[*group_index].name)}
                        </span>
                    </button>
                </li>
            });
        }

        // Add a separator if there are both filtered results and selected group components
        if !filtered_components.is_empty() && selected_group.is_some() {
            all_rendered_components.push(html! {
                <li class="yew-preview-property-item" style="margin: 15px 0; border-bottom: 1px solid #eee;"></li>
            });
        }
    }

    // Add components from the selected group
    if let Some(group_index) = selected_group {
        // Add section header for selected group
        all_rendered_components.push(html! {
            <li class="yew-preview-property-item" style="margin: 10px 0;">
                <h3>{ format!("Group: {}", groups[group_index].name) }</h3>
            </li>
        });

        for (index, component) in groups[group_index].components.iter().enumerate() {
            // Skip if this component is already shown in filtered results
            if !search_query.is_empty()
                && component
                    .name
                    .to_lowercase()
                    .contains(&search_query.to_lowercase())
            {
                continue;
            }

            let on_select = {
                let on_tree_component_select = on_tree_component_select.clone();
                let group_index = group_index;
                let index_copy = index;
                Callback::from(move |_| on_tree_component_select.emit((group_index, index_copy)))
            };

            all_rendered_components.push(html! {
                <li class="yew-preview-property-item" style="margin: 5px 0;">
                    <button style="padding: 10px 15px; cursor: pointer;" onclick={on_select}>
                        {&component.name}
                    </button>
                </li>
            });
        }
    }

    html! {
        <div class="yew-preview-sidebar-section" style="margin-bottom: 20px;">
            <h2>{ "Components" }</h2>
            <ul class="yew-preview-property-list" style="list-style: none; padding: 0;">
                { for all_rendered_components }
            </ul>
        </div>
    }
}
