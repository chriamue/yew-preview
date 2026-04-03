use crate::interactive::ArgValue;
use crate::test_utils::TestCaseResult;
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ConfigPanelProps {
    pub properties: Vec<(String, Html)>,
    pub on_select: Callback<String>,
    pub selected: Option<String>,
    pub live_args: Option<Vec<(String, ArgValue)>>,
    pub on_arg_change: Callback<(String, ArgValue)>,
    pub test_results: Vec<TestCaseResult>,
}

#[function_component(ConfigPanel)]
pub fn config_panel(props: &ConfigPanelProps) -> Html {
    let has_variants = !props.properties.is_empty();
    let has_interactive = props.live_args.is_some();
    let interactive_selected = props.selected.as_deref() == Some("Interactive");

    if !has_variants && !has_interactive {
        return html! {};
    }

    // Build variant buttons (static + optional "Interactive")
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
        html! { <button style={style} onclick={on_click}>{ name }</button> }
    });

    let interactive_button = if has_interactive {
        let is_active = interactive_selected;
        let on_select = props.on_select.clone();
        let style = if is_active {
            "padding: 5px 12px; cursor: pointer; border-radius: 4px; font-size: 0.85rem; \
             background: #0969da; color: #fff; border: 1px solid #0969da; font-weight: 600;"
        } else {
            "padding: 5px 12px; cursor: pointer; border-radius: 4px; font-size: 0.85rem; \
             background: #fff; color: #0969da; border: 1px solid #0969da;"
        };
        html! {
            <button style={style} onclick={Callback::from(move |_| on_select.emit("Interactive".to_string()))}>
                { "Interactive" }
            </button>
        }
    } else {
        html! {}
    };

    let arg_controls = if interactive_selected {
        if let Some(args) = &props.live_args {
            let controls = args.iter().map(|(name, value)| {
                let control = match value {
                    ArgValue::Bool(b) => {
                        let checked = *b;
                        let on_arg_change = props.on_arg_change.clone();
                        let name = name.clone();
                        html! {
                            <input
                                type="checkbox"
                                checked={checked}
                                onchange={Callback::from(move |e: Event| {
                                    if let Some(input) = e.target()
                                        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                                    {
                                        on_arg_change.emit((name.clone(), ArgValue::Bool(input.checked())));
                                    }
                                })}
                            />
                        }
                    }
                    ArgValue::Int(i) => {
                        let val = *i;
                        let on_arg_change = props.on_arg_change.clone();
                        let name = name.clone();
                        html! {
                            <input
                                type="number"
                                value={val.to_string()}
                                style="width: 80px; padding: 2px 6px; border: 1px solid #d0d7de; border-radius: 4px;"
                                oninput={Callback::from(move |e: InputEvent| {
                                    if let Some(input) = e.target()
                                        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                                    {
                                        if let Ok(n) = input.value().parse::<i64>() {
                                            on_arg_change.emit((name.clone(), ArgValue::Int(n)));
                                        }
                                    }
                                })}
                            />
                        }
                    }
                    ArgValue::IntRange(i, min, max) => {
                        let val = *i;
                        let min = *min;
                        let max = *max;
                        let on_arg_change = props.on_arg_change.clone();
                        let name = name.clone();
                        html! {
                            <div style="display: flex; align-items: center; gap: 8px;">
                                <input
                                    type="range"
                                    min={min.to_string()}
                                    max={max.to_string()}
                                    value={val.to_string()}
                                    style="width: 140px; cursor: pointer;"
                                    oninput={Callback::from(move |e: InputEvent| {
                                        if let Some(input) = e.target()
                                            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                                        {
                                            if let Ok(n) = input.value().parse::<i64>() {
                                                on_arg_change.emit((name.clone(), ArgValue::IntRange(n, min, max)));
                                            }
                                        }
                                    })}
                                />
                                <span style="font-size: 0.85rem; color: #24292e; min-width: 36px;">{ val }</span>
                            </div>
                        }
                    }
                    ArgValue::Float(f) => {
                        let val = *f;
                        let on_arg_change = props.on_arg_change.clone();
                        let name = name.clone();
                        html! {
                            <input
                                type="number"
                                step="0.1"
                                value={val.to_string()}
                                style="width: 80px; padding: 2px 6px; border: 1px solid #d0d7de; border-radius: 4px;"
                                oninput={Callback::from(move |e: InputEvent| {
                                    if let Some(input) = e.target()
                                        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                                    {
                                        if let Ok(n) = input.value().parse::<f64>() {
                                            on_arg_change.emit((name.clone(), ArgValue::Float(n)));
                                        }
                                    }
                                })}
                            />
                        }
                    }
                    ArgValue::Text(s) => {
                        let val = s.clone();
                        let on_arg_change = props.on_arg_change.clone();
                        let name = name.clone();
                        html! {
                            <input
                                type="text"
                                value={val}
                                style="padding: 2px 6px; border: 1px solid #d0d7de; border-radius: 4px;"
                                oninput={Callback::from(move |e: InputEvent| {
                                    if let Some(input) = e.target()
                                        .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                                    {
                                        on_arg_change.emit((name.clone(), ArgValue::Text(input.value())));
                                    }
                                })}
                            />
                        }
                    }
                };
                html! {
                    <label style="display: flex; align-items: center; gap: 6px; font-size: 0.85rem;">
                        <span style="color: #57606a; min-width: 80px;">{ name }</span>
                        { control }
                    </label>
                }
            });
            html! {
                <div style="display: flex; flex-wrap: wrap; gap: 12px; margin-top: 8px; padding-top: 8px; border-top: 1px solid #e1e4e8;">
                    { for controls }
                </div>
            }
        } else {
            html! {}
        }
    } else {
        html! {}
    };

    let label = if has_variants || has_interactive {
        html! {
            <span style="font-size: 0.75rem; color: #57606a; font-weight: 600; text-transform: uppercase; letter-spacing: 0.04em; white-space: nowrap;">
                { "Variant" }
            </span>
        }
    } else {
        html! {}
    };

    let test_panel = render_test_panel(&props.test_results);

    html! {
        <div>
            <div style="display: flex; flex-wrap: wrap; align-items: center; gap: 6px;">
                { label }
                { for buttons }
                { interactive_button }
            </div>
            { arg_controls }
            { test_panel }
        </div>
    }
}

fn render_test_panel(results: &[TestCaseResult]) -> Html {
    if results.is_empty() {
        return html! {};
    }

    let total = results.len();
    let passing = results.iter().filter(|r| r.passed).count();
    let all_pass = passing == total;

    let summary_color = if all_pass { "#1a7f37" } else { "#cf222e" };
    let summary_bg = if all_pass { "#dafbe1" } else { "#ffebe9" };
    let summary_icon = if all_pass { "✓" } else { "✗" };

    let rows = results.iter().map(|tc| {
        let icon = if tc.passed { "✓" } else { "✗" };
        let color = if tc.passed { "#1a7f37" } else { "#cf222e" };

        let matcher_detail = if !tc.passed {
            let details = tc.matchers.iter().map(|m| {
                let m_icon = if m.passed { "✓" } else { "✗" };
                let m_color = if m.passed { "#1a7f37" } else { "#cf222e" };
                html! {
                    <div style={format!("padding: 2px 0 2px 20px; font-size: 0.78rem; color: {};", m_color)}>
                        { format!("{} {}", m_icon, m.description) }
                    </div>
                }
            });
            html! { <div style="margin-top: 2px;">{ for details }</div> }
        } else {
            html! {}
        };

        html! {
            <div style="padding: 5px 0; border-bottom: 1px solid #f0f0f0;">
                <div style={format!("font-size: 0.82rem; color: {}; font-weight: 500;", color)}>
                    { format!("{} {}", icon, tc.name) }
                </div>
                { matcher_detail }
            </div>
        }
    });

    html! {
        <div style="margin-top: 10px; border-top: 1px solid #e1e4e8; padding-top: 10px;">
            <div style={format!(
                "display: flex; align-items: center; justify-content: space-between; \
                 padding: 5px 10px; border-radius: 4px 4px 0 0; \
                 background: {}; border: 1px solid {}44;",
                summary_bg, summary_color
            )}>
                <span style={format!("font-size: 0.78rem; font-weight: 700; color: {};", summary_color)}>
                    { format!("{} Tests", summary_icon) }
                </span>
                <span style={format!("font-size: 0.78rem; color: {};", summary_color)}>
                    { format!("{}/{} passing", passing, total) }
                </span>
            </div>
            <div style="border: 1px solid #e1e4e8; border-top: none; border-radius: 0 0 4px 4px; padding: 4px 10px; background: #fff;">
                { for rows }
            </div>
        </div>
    }
}
