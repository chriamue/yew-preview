use yew::prelude::*;
#[derive(Properties, PartialEq, Clone)]
pub struct ConfigPanelProps {
    pub properties: Vec<(String, String)>,
    pub on_select: Callback<String>,
}

#[function_component(ConfigPanel)]
pub fn config_panel(props: &ConfigPanelProps) -> Html {
    html! {
        <div>
            <h2>{ "Select Property" }</h2>
            <ul>
                { for props.properties.iter().map(|(name, prop)| {
                    let on_click = {
                        let on_select = props.on_select.clone();
                        let prop = prop.clone();
                        Callback::from(move |_| on_select.emit(prop.clone()))
                    };
                    html! {
                        <li>
                            <button onclick={on_click}>{ name }</button>
                        </li>
                    }
                }) }
            </ul>
        </div>
    }
}
