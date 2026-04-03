use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct BadgeProps {
    pub label: AttrValue,
    pub color: AttrValue,
    pub rounded: bool,
}

#[function_component(Badge)]
pub fn badge(props: &BadgeProps) -> Html {
    let border_radius = if props.rounded { "999px" } else { "4px" };
    html! {
        <span style={format!(
            "display: inline-block; padding: 4px 12px; background: {}; color: #fff; \
             border-radius: {}; font-size: 0.85rem; font-weight: 600; letter-spacing: 0.02em;",
            props.color, border_radius
        )}>
            { &props.label }
        </span>
    }
}

mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_interactive_preview!(
        Badge,
        args: [
            ("label",   ArgValue::Text("Hello".to_string())),
            ("color",   ArgValue::Text("#0969da".to_string())),
            ("rounded", ArgValue::Bool(true)),
        ],
        |args| {
            let label = get_text(args, "label");
            let color = get_text(args, "color");
            let rounded = get_bool(args, "rounded");
            html! {
                <Badge
                    label={AttrValue::from(label)}
                    color={AttrValue::from(color)}
                    rounded={rounded}
                />
            }
        }
    );
}
