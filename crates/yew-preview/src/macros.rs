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
            args: None,
            test_cases: Vec::new(),
            ssr_runner: None,
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
                    args: None,
                    test_cases: Vec::new(),
                    ssr_runner: None,
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
                    args: None,
                    test_cases: test_cases.clone(),
                    ssr_runner: {
                        Some(std::rc::Rc::new(move || {
                            let variants: Vec<(String, <$component as ::yew::BaseComponent>::Properties)> = vec![
                                ("Default".to_string(), $default_props),
                                $(($prop_name.to_string(), $props),)*
                            ];
                            let tcs = test_cases.clone();
                            Box::pin(async move {
                                for (variant_name, props) in variants {
                                    let html =
                                        $crate::test_utils::render_component::<$component>(props)
                                            .await;
                                    for tc in &tcs {
                                        let result = tc.run(&html);
                                        if result.passed {
                                            println!(
                                                "  [{}] [{}] ✓ {}",
                                                stringify!($component),
                                                variant_name,
                                                result.name
                                            );
                                        } else {
                                            let failures = result
                                                .matchers
                                                .iter()
                                                .filter(|m| !m.passed)
                                                .map(|m| format!("    ✗ {}", m.description))
                                                .collect::<Vec<_>>()
                                                .join("\n");
                                            panic!(
                                                "[{}] [{}] Test '{}' failed:\n{}",
                                                stringify!($component),
                                                variant_name,
                                                result.name,
                                                failures
                                            );
                                        }
                                    }
                                }
                            })
                        }))
                    },
                }
            }
        }

        // Generate a test module per component with one test per variant.
        // Module name groups the tests: `{component}_preview::default`, `::hello`, etc.
        #[cfg(test)]
        paste::paste! {
            mod [<$component:snake _preview>] {
                #[tokio::test]
                async fn default() {
                    let item = <super::$component as $crate::prelude::Preview>::preview();
                    if let Some(runner) = &item.ssr_runner {
                        runner().await;
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! create_interactive_preview {
    (
        $component:ty,
        args: [$( ($arg_name:expr, $arg_value:expr) ),* $(,)?],
        $render_fn:expr $(,)?
    ) => {
        impl Preview for $component {
            fn preview() -> ComponentItem {
                use std::rc::Rc;
                let initial_args: Vec<(String, $crate::interactive::ArgValue)> = vec![
                    $(($arg_name.to_string(), $arg_value),)*
                ];
                let render_fn: Rc<dyn Fn(&[(String, $crate::interactive::ArgValue)]) -> ::yew::Html> =
                    Rc::new($render_fn);
                ComponentItem {
                    name: stringify!($component).to_string(),
                    render: vec![],
                    args: Some($crate::interactive::InteractiveArgs {
                        values: initial_args,
                        render_fn,
                    }),
                    test_cases: Vec::new(),
                    ssr_runner: None,
                }
            }
        }
    };
}

#[cfg(feature = "testing")]
#[macro_export]
macro_rules! generate_component_test {
    // With explicit test name
    (tokio, $component:ty, $test_name:ident, $props:expr, $test_cases:expr) => {
        #[tokio::test]
        async fn $test_name() {
            let html = $crate::test_utils::render_component::<$component>($props).await;

            for test_case in $test_cases {
                let result = test_case.run(&html);
                if !result.passed {
                    let failures = result.matchers.iter()
                        .filter(|m| !m.passed)
                        .map(|m| format!("  ✗ {}", m.description))
                        .collect::<Vec<_>>()
                        .join("\n");
                    panic!("Test '{}' failed:\n{}", result.name, failures);
                }
            }
        }
    };
    // Without explicit test name (using default name)
    (tokio, $component:ty, $props:expr, $test_cases:expr) => {
        paste::paste! {
            #[tokio::test]
            async fn [<tokio_test_ $component:snake _rendering>]() {
                let html = $crate::test_utils::render_component::<$component>($props).await;

                for test_case in $test_cases {
                    let result = test_case.run(&html);
                    if !result.passed {
                        let failures = result.matchers.iter()
                            .filter(|m| !m.passed)
                            .map(|m| format!("  ✗ {}", m.description))
                            .collect::<Vec<_>>()
                            .join("\n");
                        panic!("Test '{}' failed:\n{}", result.name, failures);
                    }
                }
            }
        }
    };
    // With explicit test name
    (wasm, $component:ty, $test_name:ident, $props:expr, $test_cases:expr) => {
        #[wasm_bindgen_test]
        async fn $test_name() {
            let html = $crate::test_utils::render_component::<$component>($props).await;

            for test_case in $test_cases {
                let result = test_case.run(&html);
                if !result.passed {
                    let failures = result.matchers.iter()
                        .filter(|m| !m.passed)
                        .map(|m| format!("  ✗ {}", m.description))
                        .collect::<Vec<_>>()
                        .join("\n");
                    panic!("Test '{}' failed:\n{}", result.name, failures);
                }
            }
        }
    };
    // Without explicit test name (using default name)
    (wasm, $component:ty, $props:expr, $test_cases:expr) => {
        paste::paste! {
            #[wasm_bindgen_test]
            async fn [<wasm_test_ $component:snake _rendering>]() {
                let html = $crate::test_utils::render_component::<$component>($props).await;

                for test_case in $test_cases {
                    let result = test_case.run(&html);
                    if !result.passed {
                        let failures = result.matchers.iter()
                            .filter(|m| !m.passed)
                            .map(|m| format!("  ✗ {}", m.description))
                            .collect::<Vec<_>>()
                            .join("\n");
                        panic!("Test '{}' failed:\n{}", result.name, failures);
                    }
                }
            }
        }
    };
}
