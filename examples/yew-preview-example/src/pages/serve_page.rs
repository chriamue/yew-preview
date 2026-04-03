use crate::components::code_snippet::{CodeSnippet, CodeSnippetProps};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default)]
pub struct ServePageProps {}

#[function_component(ServePage)]
pub fn serve_page(_props: &ServePageProps) -> Html {
    let steps: Vec<(&str, &str, CodeSnippetProps)> = vec![
        (
            "1. Add the serve feature to dev-dependencies",
            "The serve feature pulls in axum and tokio — only for native builds, never for the WASM production build.",
            CodeSnippetProps {
                label: AttrValue::from("Cargo.toml"),
                code: AttrValue::from(
                    "[lib]\nname = \"my_components\"\npath = \"src/lib.rs\"\n\n[dev-dependencies]\nyew-preview = { ..., features = [\"testing\", \"serve\"] }",
                ),
            },
        ),
        (
            "2. Export your component groups from a library target",
            "The example file needs to import your groups, so expose them from lib.rs:",
            CodeSnippetProps {
                label: AttrValue::from("src/lib.rs"),
                code: AttrValue::from(
                    "use yew_preview::prelude::*;\nuse yew_preview::create_component_group;\n\npub fn preview_groups() -> ComponentList {\n    vec![\n        create_component_group!(\"Buttons\", Button::preview()),\n        create_component_group!(\"Forms\",   Input::preview()),\n    ]\n}",
                ),
            },
        ),
        (
            "3. Add examples/serve.rs",
            "Three lines — call serve_blocking with your groups and a port:",
            CodeSnippetProps {
                label: AttrValue::from("examples/serve.rs"),
                code: AttrValue::from(
                    "fn main() {\n    let port = std::env::var(\"PORT\")\n        .ok().and_then(|p| p.parse().ok()).unwrap_or(8080);\n    yew_preview::serve_blocking(my_components::preview_groups(), port);\n}",
                ),
            },
        ),
        (
            "4. Run",
            "No trunk, no wasm-pack. A native binary starts immediately:",
            CodeSnippetProps {
                label: AttrValue::from("terminal"),
                code: AttrValue::from(
                    "cargo run --example serve\n# yew-preview: http://localhost:8080\n\n# different port:\nPORT=9000 cargo run --example serve",
                ),
            },
        ),
    ];

    html! {
        <div style="padding: 40px 20px; max-width: 800px; margin: 0 auto; font-family: Arial, sans-serif;">
            <h1 style="font-size: 2rem; color: #24292e; margin: 0 0 8px 0;">
                { "Native Preview Server" }
            </h1>
            <p style="color: #586069; font-size: 1rem; margin: 0 0 8px 0; line-height: 1.5;">
                { "Serve a component preview without trunk or WASM. The " }
                <code style="background:#f6f8fa;padding:2px 6px;border-radius:3px;font-size:.9em;">{ "serve" }</code>
                { " feature pre-renders every component variant via Yew SSR and serves the result with axum. No files on disk — all HTML is generated at startup." }
            </p>
            <p style="color: #586069; font-size: 0.9rem; margin: 0 0 32px 0; line-height: 1.5;">
                { "Because it lives in " }
                <code style="background:#f6f8fa;padding:2px 6px;border-radius:3px;font-size:.9em;">{ "dev-dependencies" }</code>
                { ", axum and tokio are never included in your WASM production build." }
            </p>
            { for steps.into_iter().map(|(title, subtitle, snippet)| html! {
                <div style="margin-bottom: 40px;">
                    <h2 style="font-size: 1.2rem; color: #24292e; margin: 0 0 6px 0;">{ title }</h2>
                    <p style="color: #586069; font-size: 0.9rem; margin: 0 0 12px 0;">{ subtitle }</p>
                    <CodeSnippet code={snippet.code} label={snippet.label} />
                </div>
            }) }
        </div>
    }
}

mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(ServePage, ServePageProps {},);
}
