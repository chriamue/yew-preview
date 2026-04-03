use yew::prelude::*;
use yew_preview::prelude::*;

/// Demonstrates every ArgValue type in a single component.
#[derive(Properties, PartialEq, Clone)]
pub struct PropShowcaseProps {
    pub label: String,
    pub enabled: bool,
    pub count: i64,
    pub size: i64,
    pub ratio: f64,
}

#[function_component(PropShowcase)]
pub fn prop_showcase(props: &PropShowcaseProps) -> Html {
    let opacity = (props.ratio.clamp(0.0, 1.0) * 100.0).round() as u8;
    let bar_width = ((props.size as f64 / 200.0) * 100.0).min(100.0).round() as u8;
    let status_color = if props.enabled { "#1a7f37" } else { "#cf222e" };
    let status_label = if props.enabled { "enabled" } else { "disabled" };

    html! {
        <div style="font-family: Arial, sans-serif; border: 1px solid #e1e4e8; border-radius: 8px; padding: 20px; max-width: 360px; background: #fff;">
            <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 16px;">
                <span style="font-size: 1.1rem; font-weight: 700; color: #24292e;">{ &props.label }</span>
                <span style={format!("font-size: 0.75rem; font-weight: 600; padding: 2px 8px; border-radius: 99px; background: {}22; color: {};", status_color, status_color)}>
                    { status_label }
                </span>
            </div>

            <div style="margin-bottom: 12px;">
                <div style="font-size: 0.75rem; color: #57606a; margin-bottom: 4px;">{ "count (Int)" }</div>
                <div style="font-size: 1.5rem; font-weight: 700; color: #0969da;">{ props.count }</div>
            </div>

            <div style="margin-bottom: 12px;">
                <div style="font-size: 0.75rem; color: #57606a; margin-bottom: 4px;">
                    { format!("size {} (IntRange 1–200)", props.size) }
                </div>
                <div style="height: 8px; background: #e1e4e8; border-radius: 4px; overflow: hidden;">
                    <div style={format!("height: 100%; width: {}%; background: #0969da; border-radius: 4px; transition: width 0.1s;", bar_width)}></div>
                </div>
            </div>

            <div style="margin-bottom: 4px;">
                <div style="font-size: 0.75rem; color: #57606a; margin-bottom: 4px;">
                    { format!("ratio {:.2} (Float) → opacity {}%", props.ratio, opacity) }
                </div>
                <div style={format!("height: 32px; background: #0969da; border-radius: 4px; opacity: {};", props.ratio.clamp(0.0, 1.0))}>
                </div>
            </div>
        </div>
    }
}

impl Preview for PropShowcase {
    fn preview() -> ComponentItem {
        use std::rc::Rc;
        use yew::html;

        let initial_args: Vec<(String, ArgValue)> = vec![
            ("label".to_string(),   ArgValue::Text("My Component".to_string())),
            ("enabled".to_string(), ArgValue::Bool(true)),
            ("count".to_string(),   ArgValue::Int(42)),
            ("size".to_string(),    ArgValue::IntRange(80, 1, 200)),
            ("ratio".to_string(),   ArgValue::Float(0.8)),
        ];
        let render_fn: Rc<dyn Fn(&[(String, ArgValue)]) -> Html> = Rc::new(|args| {
            let label   = get_text(args, "label");
            let enabled = get_bool(args, "enabled");
            let count   = get_int(args, "count");
            let size    = get_int(args, "size");
            let ratio   = get_float(args, "ratio");
            html! {
                <PropShowcase
                    label={label}
                    enabled={enabled}
                    count={count}
                    size={size}
                    ratio={ratio}
                />
            }
        });

        ComponentItem {
            name: "PropShowcase".to_string(),
            render: vec![],
            args: Some(InteractiveArgs {
                values: initial_args,
                render_fn,
            }),
            test_cases: vec![],
        }
    }
}
