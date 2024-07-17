#[macro_export]
macro_rules! create_component_item {
    ($name:expr, $component:ty, $props:expr) => {
        ComponentItem {
            name: $name.to_string(),
            render: $props
                .into_iter()
                .map(|(prop_name, props)| {
                    let html = html! { <$component ..props /> };
                    (prop_name.to_string(), html)
                })
                .collect(),
        }
    };
}
#[macro_export]
macro_rules! create_preview {
    ($component:ty, $default_props:expr, $(($prop_name:expr, $props:expr)),* $(,)?) => {
        impl Preview for $component {
            fn preview() -> ComponentItem {
                let mut render: Vec<(String, Html)> = vec![
                    (
                        "Default".to_string(),
                        html! { <$component ..$default_props /> },
                    ),
                ];

                $(
                    render.push((
                        $prop_name.to_string(),
                        html! { <$component ..$props /> },
                    ));
                )*

                ComponentItem {
                    name: stringify!($component).to_string(),
                    render,
                }
            }
        }
    };
}
