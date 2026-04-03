use crate::components::code_snippet::{CodeSnippet, CodeSnippetProps};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default)]
pub struct GettingStartedPageProps {}

#[function_component(GettingStartedPage)]
pub fn getting_started_page(_props: &GettingStartedPageProps) -> Html {
    let steps: Vec<(&str, &str, CodeSnippetProps)> = vec![
        (
            "1. Add Dependency",
            "Add yew-preview to your Cargo.toml and declare the feature flag:",
            CodeSnippetProps {
                label: AttrValue::from("Cargo.toml"),
                code: AttrValue::from(
                    "[dependencies]\nyew = { version = \"0.23\", features = [\"csr\"] }\nyew-preview = { git = \"https://github.com/chriamue/yew-preview\" }\n\n[features]\ndefault = []\nyew-preview = []",
                ),
            },
        ),
        (
            "2. Write Your Component",
            "Define a standard Yew function component as usual:",
            CodeSnippetProps {
                label: AttrValue::from("src/components/button.rs"),
                code: AttrValue::from(
                    "use yew::prelude::*;\n\n#[derive(Properties, PartialEq)]\npub struct ButtonProps {\n    pub label: AttrValue,\n    pub disabled: bool,\n}\n\n#[function_component(Button)]\npub fn button(props: &ButtonProps) -> Html {\n    html! {\n        <button disabled={props.disabled}>{ &props.label }</button>\n    }\n}",
                ),
            },
        ),
        (
            "3. Add Preview Variants",
            "Gate a preview module with the feature flag and register variants with create_preview!:",
            CodeSnippetProps {
                label: AttrValue::from("src/components/button.rs (continued)"),
                code: AttrValue::from(
                    "#[cfg(feature = \"yew-preview\")]\nmod preview {\n    use super::*;\n    use yew_preview::prelude::*;\n\n    yew_preview::create_preview!(\n        Button,\n        ButtonProps { label: AttrValue::from(\"Click me\"), disabled: false },\n        (\n            \"Disabled\",\n            ButtonProps { label: AttrValue::from(\"Can't click\"), disabled: true }\n        ),\n    );\n}",
                ),
            },
        ),
        (
            "4. Register and Run",
            "Wire components into the preview app and run with Trunk:",
            CodeSnippetProps {
                label: AttrValue::from("src/main.rs"),
                code: AttrValue::from(
                    "use yew_preview::{create_component_group, prelude::*};\n\nlet groups: ComponentList = vec![\n    create_component_group!(\"Buttons\", Button),\n];\n\nhtml! { <PreviewPage groups={groups} /> }\n\n// then in the terminal:\n// trunk serve",
                ),
            },
        ),
    ];

    html! {
        <div style="padding: 40px 20px; max-width: 800px; margin: 0 auto; font-family: Arial, sans-serif;">
            <h1 style="font-size: 2rem; color: #24292e; margin: 0 0 8px 0;">
                { "Getting Started" }
            </h1>
            <p style="color: #586069; font-size: 1rem; margin: 0 0 40px 0; line-height: 1.5;">
                { "Four steps to add interactive component previews to your Yew application." }
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

    yew_preview::create_preview!(
        GettingStartedPage,
        GettingStartedPageProps {},
    );
}
