pub mod helpers;
pub mod matchers;
pub mod test_case;

pub use helpers::*;
pub use matchers::Matcher;
pub use test_case::{MatcherResult, TestCase, TestCaseResult};

use yew::{BaseComponent, Properties};

pub trait TestCases {
    fn get_test_cases(&self) -> Vec<TestCase>;
}

/// Render component `C` with `props` via SSR.
/// Returns the rendered HTML string when the `testing` feature is enabled;
/// returns an empty string otherwise (so ssr_runner closures always compile).
pub async fn render_component<C>(props: C::Properties) -> String
where
    C: BaseComponent,
    C::Properties: Properties + PartialEq + Clone,
{
    #[cfg(feature = "testing")]
    {
        let renderer = yew::LocalServerRenderer::<C>::with_props(props);
        renderer.render().await
    }
    #[cfg(not(feature = "testing"))]
    {
        let _ = props;
        String::new()
    }
}

/// Run SSR tests for every component in every group that has an `ssr_runner`.
/// Call this from a single `#[tokio::test]` pointing at your app's groups.
/// Run with `-- --nocapture` to see per-test-case output.
///
/// When the `tests-ignored` feature is active this function is a no-op, which
/// mirrors the `#[ignore]` behaviour applied to the auto-generated test modules
/// produced by `create_preview_with_tests!`.
#[cfg(feature = "testing")]
pub async fn run_groups_tests(groups: &crate::component_list::ComponentList) {
    #[cfg(feature = "tests-ignored")]
    {
        let _ = groups;
        return;
    }
    #[cfg(not(feature = "tests-ignored"))]
    for group in groups {
        let testable: Vec<_> = group.components.iter()
            .filter(|c| c.ssr_runner.is_some())
            .collect();
        if testable.is_empty() {
            continue;
        }
        println!("\n[{}]", group.name);
        for component in testable {
            if let Some(runner) = &component.ssr_runner {
                runner().await;
            }
        }
    }
}

/// Render `C` with `props` via SSR and run every `TestCase` in `item.test_cases`.
/// Panics on the first failing test case, reporting which matchers failed.
#[cfg(feature = "testing")]
pub async fn run_component_tests<C>(props: C::Properties, item: &crate::component_item::ComponentItem)
where
    C: BaseComponent,
    C::Properties: Properties + PartialEq + Clone,
{
    let html = render_component::<C>(props).await;
    for tc in &item.test_cases {
        let result = tc.run(&html);
        if !result.passed {
            let failures = result
                .matchers
                .iter()
                .filter(|m| !m.passed)
                .map(|m| format!("  ✗ {}", m.description))
                .collect::<Vec<_>>()
                .join("\n");
            panic!("Test '{}' failed:\n{}", result.name, failures);
        }
    }
}
