#[macro_export]
macro_rules! create_component_item {
    ($name:expr, $render:expr, $props:expr) => {
        ComponentItem {
            name: $name.to_string(),
            render: $render,
            props: $props,
        }
    };
}
