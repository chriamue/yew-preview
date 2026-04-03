use gloo_net::http::Request;
use pulldown_cmark::{html as md_html, Options, Parser};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
enum LoadState {
    Loading,
    Loaded(String),
    Error(String),
}

#[derive(Properties, PartialEq, Clone)]
pub struct DocPageProps {
    pub src: AttrValue,
    pub title: AttrValue,
}

#[function_component(DocPage)]
pub fn doc_page(props: &DocPageProps) -> Html {
    let state = use_state(|| LoadState::Loading);

    {
        let state = state.clone();
        use_effect_with(props.src.clone(), move |src| {
            let src = src.to_string();
            state.set(LoadState::Loading);
            wasm_bindgen_futures::spawn_local(async move {
                let result: Result<String, String> = async {
                    let resp = Request::get(&src)
                        .send()
                        .await
                        .map_err(|e| e.to_string())?;
                    if !resp.ok() {
                        return Err(format!("HTTP {}", resp.status()));
                    }
                    resp.text().await.map_err(|e| e.to_string())
                }
                .await;

                match result {
                    Ok(text) => {
                        let processed = preprocess_wiki_links(&text);
                        let mut opts = Options::empty();
                        opts.insert(Options::ENABLE_TABLES);
                        opts.insert(Options::ENABLE_STRIKETHROUGH);
                        let parser = Parser::new_ext(&processed, opts);
                        let mut output = String::new();
                        md_html::push_html(&mut output, parser);
                        state.set(LoadState::Loaded(output));
                    }
                    Err(e) => state.set(LoadState::Error(e)),
                }
            });
        });
    }

    let github_link = props
        .src
        .replace("raw.githubusercontent.com", "github.com")
        .replace("/main/", "/blob/main/");

    html! {
        <div style="padding: 32px; max-width: 860px; margin: 0 auto; font-family: Arial, sans-serif; line-height: 1.6;">
            <div style="display: flex; align-items: baseline; justify-content: space-between; margin-bottom: 24px; padding-bottom: 16px; border-bottom: 1px solid #e1e4e8; gap: 16px; flex-wrap: wrap;">
                <h1 style="margin: 0; color: #24292e; font-size: 1.75rem;">{ &props.title }</h1>
                <a href={github_link}
                   target="_blank"
                   rel="noopener noreferrer"
                   style="font-size: 0.85rem; color: #0366d6; text-decoration: none; white-space: nowrap; flex-shrink: 0;">
                    { "View on GitHub \u{2197}" }
                </a>
            </div>
            {
                match &*state {
                    LoadState::Loading => html! {
                        <div style="color: #57606a; padding: 48px 0; text-align: center; font-size: 0.95rem;">
                            { "Loading\u{2026}" }
                        </div>
                    },
                    LoadState::Error(e) => html! {
                        <div style="color: #cf222e; padding: 16px; border: 1px solid #ffd8d3; border-radius: 6px; background: #fff8f7; font-size: 0.9rem;">
                            { format!("Failed to load: {}", e) }
                        </div>
                    },
                    LoadState::Loaded(html_str) => {
                        let html = format!("<div class=\"md\">{}</div>", html_str);
                        Html::from_html_unchecked(AttrValue::from(html))
                    }
                }
            }
        </div>
    }
}

/// Converts Obsidian wiki links to GitHub markdown links.
/// `[[filename]]` → `[filename](filename.md)`
/// `[[filename#section]]` → `[filename](filename.md#section)`
fn preprocess_wiki_links(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut rest = input;
    while let Some(start) = rest.find("[[") {
        result.push_str(&rest[..start]);
        rest = &rest[start + 2..];
        if let Some(end) = rest.find("]]") {
            let link = &rest[..end];
            let (name, anchor) = link.split_once('#').unwrap_or((link, ""));
            if anchor.is_empty() {
                result.push_str(&format!("[{}]({}.md)", name, name));
            } else {
                result.push_str(&format!("[{}]({}.md#{})", name, name, anchor));
            }
            rest = &rest[end + 2..];
        } else {
            result.push_str("[[");
        }
    }
    result.push_str(rest);
    result
}

mod preview {
    use super::*;
    use yew_preview::prelude::*;

    const SOURCE_URL: &str = match option_env!("YEWPREVIEW_SOURCE_URL") {
        Some(url) => url,
        None => "https://github.com/chriamue/yew-preview",
    };

    fn raw_doc(file: &str) -> AttrValue {
        let raw = SOURCE_URL.replace(
            "https://github.com/",
            "https://raw.githubusercontent.com/",
        );
        AttrValue::from(format!("{}/main/docs/{}", raw, file))
    }

    yew_preview::create_preview!(
        DocPage,
        DocPageProps {
            src: raw_doc("index.md"),
            title: AttrValue::from("Overview"),
        },
        (
            "Getting Started",
            DocPageProps {
                src: raw_doc("getting-started.md"),
                title: AttrValue::from("Getting Started"),
            }
        ),
        (
            "Macros",
            DocPageProps {
                src: raw_doc("macros.md"),
                title: AttrValue::from("Macros Reference"),
            }
        ),
        (
            "Components",
            DocPageProps {
                src: raw_doc("components.md"),
                title: AttrValue::from("UI Components"),
            }
        ),
        (
            "Testing",
            DocPageProps {
                src: raw_doc("testing.md"),
                title: AttrValue::from("Testing"),
            }
        ),
        (
            "Architecture",
            DocPageProps {
                src: raw_doc("architecture.md"),
                title: AttrValue::from("Architecture"),
            }
        ),
        (
            "Examples",
            DocPageProps {
                src: raw_doc("examples.md"),
                title: AttrValue::from("Examples"),
            }
        )
    );
}
