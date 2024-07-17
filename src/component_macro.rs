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
