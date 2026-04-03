use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ConfigPanelProps {
    pub properties: Vec<(String, Html)>,
    pub on_select: Callback<String>,
    pub selected: Option<String>,
}

#[function_component(ConfigPanel)]
pub fn config_panel(props: &ConfigPanelProps) -> Html {
    if props.properties.is_empty() {
        return html! {};
    }

    let buttons = props.properties.iter().map(|(name, _)| {
        let is_active = props.selected.as_deref() == Some(name.as_str());
        let on_click = {
            let on_select = props.on_select.clone();
            let name = name.clone();
            Callback::from(move |_| on_select.emit(name.clone()))
        };
        let style = if is_active {
            "padding: 5px 12px; cursor: pointer; border-radius: 4px; font-size: 0.85rem; \
             background: #24292e; color: #fff; border: 1px solid #24292e; font-weight: 600;"
        } else {
            "padding: 5px 12px; cursor: pointer; border-radius: 4px; font-size: 0.85rem; \
             background: #fff; color: #24292e; border: 1px solid #d0d7de;"
        };
        html! {
            <button style={style} onclick={on_click}>{ name }</button>
        }
    });

    html! {
        <div style="display: flex; flex-wrap: wrap; align-items: center; gap: 6px;">
            <span style="font-size: 0.75rem; color: #57606a; font-weight: 600; text-transform: uppercase; letter-spacing: 0.04em; white-space: nowrap;">
                { "Variant" }
            </span>
            { for buttons }
        </div>
    }
}
