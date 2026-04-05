use crate::components::code_snippet::{CodeSnippet, CodeSnippetProps};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default)]
pub struct CatalogPageProps {}

#[function_component(CatalogPage)]
pub fn catalog_page(_props: &CatalogPageProps) -> Html {
    let steps: Vec<(&str, &str, CodeSnippetProps)> = vec![
        (
            "1. Add the catalog feature to dev-dependencies",
            "The catalog feature pulls in tokio and Yew SSR — only for native builds, never for the WASM production build. If you already enable the serve feature, catalog is included automatically.",
            CodeSnippetProps {
                label: AttrValue::from("Cargo.toml"),
                code: AttrValue::from(
                    "[lib]\nname = \"my_components\"\npath = \"src/lib.rs\"\n\n[dev-dependencies]\nyew-preview = { ..., features = [\"testing\", \"catalog\"] }\n\n[[example]]\nname = \"catalog\"\nrequired-features = [\"yew-preview/catalog\"]",
                ),
            },
        ),
        (
            "2. Export your component groups from a library target",
            "The example binary imports your groups from the library target, so expose them from lib.rs:",
            CodeSnippetProps {
                label: AttrValue::from("src/lib.rs"),
                code: AttrValue::from(
                    "use yew_preview::prelude::*;\nuse yew_preview::create_component_group;\n\npub fn preview_groups() -> ComponentList {\n    vec![\n        create_component_group!(\"Buttons\", Button::preview()),\n        create_component_group!(\"Forms\",   Input::preview()),\n    ]\n}",
                ),
            },
        ),
        (
            "3. Add examples/catalog.rs",
            "Pass CatalogOptions to configure the output filename and inject your component CSS:",
            CodeSnippetProps {
                label: AttrValue::from("examples/catalog.rs"),
                code: AttrValue::from(
                    "fn main() {\n    let css = std::path::PathBuf::from(env!(\"CARGO_MANIFEST_DIR\"))\n        .join(\"my-styles.css\");\n\n    let output = std::env::var(\"OUTPUT\")\n        .ok()\n        .map(std::path::PathBuf::from);\n\n    let mut options = yew_preview::CatalogOptions::new(\"my-components\")\n        .css_file(css);\n\n    if let Some(out) = output {\n        options = options.output(out);\n    }\n\n    yew_preview::generate_catalog_blocking(\n        my_components::preview_groups(),\n        options,\n    );\n}",
                ),
            },
        ),
        (
            "4. Run",
            "No trunk, no wasm-pack. A native binary renders every component via Yew SSR and writes a single HTML file:",
            CodeSnippetProps {
                label: AttrValue::from("terminal"),
                code: AttrValue::from(
                    "# Output: my-components-yew-preview.html (current directory)\ncargo run --example catalog\n\n# Custom output path\nOUTPUT=dist/catalog.html cargo run --example catalog",
                ),
            },
        ),
    ];

    html! {
        <div style="padding: 40px 20px; max-width: 800px; margin: 0 auto; font-family: Arial, sans-serif;">
            <h1 style="font-size: 2rem; color: #24292e; margin: 0 0 8px 0;">
                { "Static HTML Catalog" }
            </h1>
            <p style="color: #586069; font-size: 1rem; margin: 0 0 8px 0; line-height: 1.5;">
                { "The " }
                <code style="background:#f6f8fa;padding:2px 6px;border-radius:3px;font-size:.9em;">{ "catalog" }</code>
                { " feature renders every component variant via Yew SSR and writes a single self-contained HTML file — a long-scroll catalog of all components, like a design-system reference sheet." }
            </p>
            <p style="color: #586069; font-size: 0.9rem; margin: 0 0 32px 0; line-height: 1.5;">
                { "Because it lives in " }
                <code style="background:#f6f8fa;padding:2px 6px;border-radius:3px;font-size:.9em;">{ "dev-dependencies" }</code>
                { ", tokio and Yew SSR are never included in your WASM production build. The " }
                <code style="background:#f6f8fa;padding:2px 6px;border-radius:3px;font-size:.9em;">{ "serve" }</code>
                { " feature already implies " }
                <code style="background:#f6f8fa;padding:2px 6px;border-radius:3px;font-size:.9em;">{ "catalog" }</code>
                { ", so no extra setup is needed if you use both." }
            </p>
            { for steps.into_iter().map(|(title, subtitle, snippet)| html! {
                <div style="margin-bottom: 40px;">
                    <h2 style="font-size: 1.2rem; color: #24292e; margin: 0 0 6px 0;">{ title }</h2>
                    <p style="color: #586069; font-size: 0.9rem; margin: 0 0 12px 0;">{ subtitle }</p>
                    <CodeSnippet code={snippet.code} label={snippet.label} />
                </div>
            }) }
            <div style="background:#f6f8fa;border:1px solid #e1e4e8;border-radius:6px;padding:20px 24px;margin-top:8px;">
                <h2 style="font-size: 1rem; color: #24292e; margin: 0 0 12px 0;">{ "Catalog vs. native server" }</h2>
                <table style="width:100%;border-collapse:collapse;font-size:0.85rem;color:#586069;">
                    <thead>
                        <tr>
                            <th style="text-align:left;padding:6px 10px;border-bottom:1px solid #e1e4e8;color:#24292e;"></th>
                            <th style="text-align:left;padding:6px 10px;border-bottom:1px solid #e1e4e8;color:#24292e;">
                                <code style="font-size:.85em;">{ "cargo run --example catalog" }</code>
                            </th>
                            <th style="text-align:left;padding:6px 10px;border-bottom:1px solid #e1e4e8;color:#24292e;">
                                <code style="font-size:.85em;">{ "cargo run --example serve" }</code>
                            </th>
                        </tr>
                    </thead>
                    <tbody>
                        { for [
                            ("Output",          "Static .html file",   "Live HTTP server"),
                            ("Navigation",      "Sticky nav + anchors","Sidebar + query params"),
                            ("Usable offline",  "Yes",                 "No"),
                            ("CI / artefacts",  "Yes",                 "No"),
                            ("Extra deps",      "tokio only",          "tokio + axum"),
                        ].iter().map(|(label, catalog, serve)| html! {
                            <tr>
                                <td style="padding:6px 10px;border-bottom:1px solid #f3f4f6;color:#24292e;font-weight:500;">{ label }</td>
                                <td style="padding:6px 10px;border-bottom:1px solid #f3f4f6;">{ catalog }</td>
                                <td style="padding:6px 10px;border-bottom:1px solid #f3f4f6;">{ serve }</td>
                            </tr>
                        }) }
                    </tbody>
                </table>
            </div>
        </div>
    }
}

mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(CatalogPage, CatalogPageProps {},);
}
