//! Shared SSR infrastructure used by both the `serve` and `catalog` features.
//!
//! This module provides:
//! - The core data types (`GroupData`, `ComponentData`, `VariantData`, `TestData`)
//! - `prerender()` — converts a `ComponentList` into pre-rendered `String` data
//! - `esc()` — HTML-escaping helper

use crate::component_list::ComponentList;
use yew::prelude::*;

// ── SSR wrapper ───────────────────────────────────────────────────────────────

#[derive(Properties, PartialEq, Clone)]
pub(crate) struct VNodeProps {
    pub node: Html,
}

#[function_component(VNodeRenderer)]
pub(crate) fn vnode_renderer(props: &VNodeProps) -> Html {
    props.node.clone()
}

/// Render a Yew `Html` node to a `String` via SSR.
pub(crate) async fn ssr(node: Html) -> String {
    yew::LocalServerRenderer::<VNodeRenderer>::with_props(VNodeProps { node })
        .render()
        .await
}

// ── Send-safe pre-rendered data types ────────────────────────────────────────

pub(crate) struct VariantData {
    pub name: String,
    pub html: String,
}

pub(crate) struct TestData {
    pub name: String,
    pub matchers: Vec<String>,
}

pub(crate) struct ComponentData {
    pub name: String,
    pub variants: Vec<VariantData>,
    pub tests: Vec<TestData>,
    pub has_interactive: bool,
}

pub(crate) struct GroupData {
    pub name: String,
    pub components: Vec<ComponentData>,
}

// ── Pre-rendering ─────────────────────────────────────────────────────────────

/// A function that wraps a variant's [`Html`] before SSR — use it to inject
/// context providers (i18n, theme, repository, …) that components require.
///
/// Stored as an `Rc` so it can live on the `LocalSet` thread without `Send`.
pub type RenderWrapper = std::rc::Rc<dyn Fn(Html) -> Html>;

/// Consume a `ComponentList` (not `Send`), SSR-render every variant, and return
/// plain `String` data that is `Send + Sync`.
///
/// Must be called inside a `tokio::task::LocalSet` because Yew's
/// `LocalServerRenderer` uses `spawn_local` internally.
///
/// If `wrapper` is `Some`, every variant node is passed through it before
/// SSR so that context providers (i18n, theme, …) are in scope.
pub(crate) async fn prerender(
    groups: ComponentList,
    wrapper: Option<&RenderWrapper>,
) -> Vec<GroupData> {
    let mut result = Vec::new();
    for group in groups {
        let mut components = Vec::new();
        for item in group.components {
            let mut variants = Vec::new();
            for (name, node) in item.render {
                let node = match wrapper {
                    Some(w) => w(node),
                    None => node,
                };
                variants.push(VariantData {
                    name,
                    html: ssr(node).await,
                });
            }
            let tests = item
                .test_cases
                .iter()
                .map(|tc| TestData {
                    name: tc.name.clone(),
                    matchers: tc.matchers.iter().map(|m| format!("{m:?}")).collect(),
                })
                .collect();
            components.push(ComponentData {
                name: item.name,
                variants,
                tests,
                has_interactive: item.args.is_some(),
            });
        }
        result.push(GroupData {
            name: group.name,
            components,
        });
    }
    result
}

// ── Utilities ─────────────────────────────────────────────────────────────────

/// Escape a string for safe inclusion in HTML text / attribute values.
pub(crate) fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
