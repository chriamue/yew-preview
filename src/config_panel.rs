use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ConfigPanelProps {
    pub properties: Vec<(String, Html)>,
    pub on_select: Callback<String>,
}

#[function_component(ConfigPanel)]
pub fn config_panel(props: &ConfigPanelProps) -> Html {
    html! {
        <div style="border: 1px solid #ccc; padding: 20px; width: 100%; max-width: 600px; box-shadow: 0 0 10px rgba(0,0,0,0.1);">
            <h2>{ "Select Property" }</h2>
            <ul style="list-style: none; padding: 0;">
                { for props.properties.iter().map(|(name, _)| {
                    let on_click = {
                        let on_select = props.on_select.clone();
                        let name = name.clone();
                        Callback::from(move |_| on_select.emit(name.clone()))
                    };
                    html! {
                        <li style="margin: 5px 0;">
                            <button style="padding: 10px 15px; cursor: pointer;" onclick={on_click}>{ name }</button>
                        </li>
                    }
                }) }
            </ul>
        </div>
    }
}
