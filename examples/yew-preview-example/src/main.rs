use yew::prelude::*;
use yew_preview::prelude::*;
use yew_preview_example::preview_groups;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div style="font-family: Arial, sans-serif; height: 100vh; display: flex; flex-direction: column; overflow: hidden;">
            <div style="padding: 8px 20px; background: #24292e; flex-shrink: 0; display: flex; align-items: center; gap: 16px;">
                <span style="color: #fff; font-weight: 700; font-size: 1.1rem;">{ "YewPreview" }</span>
                <span style="color: #8b949e; font-size: 0.85rem;">{ "Interactive component browser for Yew" }</span>
                <a href="catalog.html"
                   target="_blank" rel="noopener noreferrer"
                   style="margin-left: auto; color: #8b949e; font-size: 0.8rem; text-decoration: none; padding: 3px 10px; border: 1px solid #444d56; border-radius: 12px; white-space: nowrap;">
                    { "\u{1F4C4} Catalog" }
                </a>
            </div>
            <div style="flex: 1; overflow: hidden;">
                <PreviewPage groups={preview_groups()} />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[cfg(test)]
mod tests {
    use yew_preview::test_utils::run_groups_tests;
    use yew_preview_example::preview_groups;

    #[tokio::test]
    async fn test_all_components() {
        run_groups_tests(&preview_groups()).await;
    }
}
