use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct CodeSnippetProps {
    pub code: AttrValue,
    #[prop_or_default]
    pub label: AttrValue,
}

#[function_component(CodeSnippet)]
pub fn code_snippet(props: &CodeSnippetProps) -> Html {
    html! {
        <div>
            if !props.label.is_empty() {
                <div style="font-size: 0.8rem; color: #586069; margin-bottom: 4px; font-family: monospace;">
                    { &props.label }
                </div>
            }
            <pre style="background: #f6f8fa; border: 1px solid #e1e4e8; border-radius: 6px; padding: 16px; overflow-x: auto; font-size: 0.85rem; line-height: 1.5; margin: 0; white-space: pre-wrap; word-break: break-word;">
                <code style="font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace; color: #24292e;">
                    { &props.code }
                </code>
            </pre>
        </div>
    }
}

mod preview {
    use super::*;
    use yew_preview::prelude::*;
    use yew_preview::test_utils::exists;

    yew_preview::create_preview_with_tests!(
        component: CodeSnippet,
        default_props: CodeSnippetProps {
            label: AttrValue::from("Cargo.toml"),
            code: AttrValue::from("[dependencies]\nyew-preview = { git = \"https://github.com/chriamue/yew-preview\" }"),
        },
        variants: [
            (
                "create_preview! macro",
                CodeSnippetProps {
                    label: AttrValue::from("component.rs"),
                    code: AttrValue::from("yew_preview::create_preview!(\n    MyComp,\n    MyCompProps { text: AttrValue::from(\"Default\") },\n    (\"Variant\", MyCompProps { text: AttrValue::from(\"Hello!\") }),\n);"),
                }
            ),
            (
                "Component group",
                CodeSnippetProps {
                    label: AttrValue::from("main.rs"),
                    code: AttrValue::from("let groups = vec![\n    create_component_group!(\"UI\", MyComp),\n];\nhtml! { <PreviewPage groups={groups} /> }"),
                }
            ),
            (
                "No label",
                CodeSnippetProps {
                    label: AttrValue::from(""),
                    code: AttrValue::from("fn main() { println!(\"hello\"); }"),
                }
            ),
        ],
        tests: [
            ("Has pre element", exists("pre")),
            ("Has code element", exists("code")),
            ("Has wrapper element", exists("div")),
        ]
    );
}
