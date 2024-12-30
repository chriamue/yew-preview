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
            test_cases: Vec::new(),
        }
    };
}

#[macro_export]
macro_rules! create_component_group {
    ($name:expr, $($components:expr),* $(,)?) => {
        ComponentGroup {
            name: $name.to_string(),
            components: vec![$($components),*],
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
                    test_cases: Vec::new(),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! create_preview_with_tests {
    (
        component: $component:ty,
        default_props: $default_props:expr,
        variants: [$(($prop_name:expr, $props:expr)),* $(,)?],
        tests: [$(($test_name:expr, $($matcher:expr),* $(,)?)),* $(,)?]
    ) => {
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

                let test_cases = vec![
                    $(
                        {
                            let mut test_case = TestCase::new($test_name);
                            $(
                                test_case.matchers.push($matcher);
                            )*
                            test_case
                        },
                    )*
                ];

                ComponentItem {
                    name: stringify!($component).to_string(),
                    render,
                    test_cases,
                }
            }
        }
    };
}

#[macro_export]
macro_rules! generate_component_test {
    // With explicit test name
    (tokio, $component:ty, $test_name:ident, $props:expr, $test_cases:expr) => {
        #[tokio::test]
        async fn $test_name() {
            let html = crate::test_utils::render_component::<$component>($props).await;

            for test_case in $test_cases {
                assert!(
                    test_case.matches(&html),
                    "Test case '{}' failed",
                    test_case.name
                );
            }
        }
    };
    // Without explicit test name (using default name)
    (tokio, $component:ty, $props:expr, $test_cases:expr) => {
        paste::paste! {
            #[tokio::test]
            async fn [<tokio_test_ $component:snake _rendering>]() {
                let html = crate::test_utils::render_component::<$component>($props).await;

                for test_case in $test_cases {
                    assert!(
                        test_case.matches(&html),
                        "Test case '{}' failed",
                        test_case.name
                    );
                }
            }
        }
    };
    // With explicit test name
    (wasm, $component:ty, $test_name:ident, $props:expr, $test_cases:expr) => {
        #[wasm_bindgen_test]
        async fn $test_name() {
            let html = crate::test_utils::render_component::<$component>($props).await;

            for test_case in $test_cases {
                assert!(
                    test_case.matches(&html),
                    "Test case '{}' failed",
                    test_case.name
                );
            }
        }
    };
    // Without explicit test name (using default name)
    (wasm, $component:ty, $props:expr, $test_cases:expr) => {
        paste::paste! {
            #[wasm_bindgen_test]
            async fn [<wasm_test_ $component:snake _rendering>]() {
                let html = crate::test_utils::render_component::<$component>($props).await;

                for test_case in $test_cases {
                    assert!(
                        test_case.matches(&html),
                        "Test case '{}' failed",
                        test_case.name
                    );
                }
            }
        }
    };
}
