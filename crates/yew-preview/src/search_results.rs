use crate::component_list::ComponentList;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SearchResultsProps {
    pub groups: ComponentList,
    pub search_query: String,
    pub on_select: Callback<(usize, usize)>, // For group_index and component_index
}

#[function_component(SearchResults)]
pub fn search_results(props: &SearchResultsProps) -> Html {
    if props.search_query.is_empty() {
        return html! {};
    }

    html! {
        <div class="search-results" style="margin-top: 20px; border-top: 1px solid #ddd; padding-top: 20px;">
            <h3>{"Search Results"}</h3>
            <ul style="list-style: none; padding: 0;">
                {
                    props.groups.iter().enumerate().map(|(group_index, group)| {
                        group.components.iter().enumerate().filter_map(|(comp_index, component)| {
                            if component.name.to_lowercase().contains(&props.search_query.to_lowercase()) {
                                let on_select = {
                                    let on_select = props.on_select.clone();
                                    let group_index = group_index;
                                    let comp_index = comp_index;
                                    Callback::from(move |_| {
                                        on_select.emit((group_index, comp_index));
                                    })
                                };

                                Some(html! {
                                    <li key={format!("{}-{}", group_index, comp_index)} style="margin: 8px 0;">
                                        <button
                                            onclick={on_select}
                                            style="
                                                width: 100%;
                                                text-align: left;
                                                padding: 8px 12px;
                                                background-color: white;
                                                border: 1px solid #ddd;
                                                border-radius: 4px;
                                                cursor: pointer;
                                                color: #333;
                                                &:hover {
                                                    background-color: #f0f0f0;
                                                }
                                            "
                                        >
                                            <div style="font-weight: bold; color: #333;">
                                                {component.name.clone()}
                                            </div>
                                            <div style="color: #666; font-size: 0.8em;">
                                                {format!("in {}", group.name)}
                                            </div>
                                        </button>
                                    </li>
                                })
                            } else {
                                None
                            }
                        }).collect::<Html>()
                    }).collect::<Html>()
                }
            </ul>
        </div>
    }
}
