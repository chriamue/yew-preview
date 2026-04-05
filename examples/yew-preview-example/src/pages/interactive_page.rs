use crate::components::code_snippet::CodeSnippet;
use yew::prelude::*;

struct ArgTypeDoc {
    variant: &'static str,
    description: &'static str,
    snippet: &'static str,
    accessor: &'static str,
}

const ARG_TYPES: &[ArgTypeDoc] = &[
    ArgTypeDoc {
        variant: "ArgValue::Text(String)",
        description: "A free-form text input. Use for labels, URLs, or any string prop.",
        snippet: r#"("label", ArgValue::Text("Hello".to_string()))"#,
        accessor: r#"let label = get_text(args, "label");"#,
    },
    ArgTypeDoc {
        variant: "ArgValue::Bool(bool)",
        description: "A checkbox. Use for flags, toggles, and boolean props.",
        snippet: r#"("enabled", ArgValue::Bool(true))"#,
        accessor: r#"let enabled = get_bool(args, "enabled");"#,
    },
    ArgTypeDoc {
        variant: "ArgValue::Int(i64)",
        description: "A number input for an unconstrained integer.",
        snippet: r#"("count", ArgValue::Int(42))"#,
        accessor: r#"let count = get_int(args, "count");"#,
    },
    ArgTypeDoc {
        variant: "ArgValue::IntRange(value, min, max)",
        description: "A slider for a bounded integer. Ideal for sizes, counts with known limits.",
        snippet: r#"("size", ArgValue::IntRange(80, 1, 200))"#,
        accessor: r#"let size = get_int(args, "size") as u32;"#,
    },
    ArgTypeDoc {
        variant: "ArgValue::Float(f64)",
        description: "A decimal number input. Use for opacity, ratios, or fractional values.",
        snippet: r#"("ratio", ArgValue::Float(0.8))"#,
        accessor: r#"let ratio = get_float(args, "ratio");"#,
    },
];

const FULL_EXAMPLE: &str = r#"use yew_preview::prelude::*;

create_interactive_preview!(
    MyComponent,
    args: [
        ("label",   ArgValue::Text("Hello".to_string())),
        ("enabled", ArgValue::Bool(true)),
        ("count",   ArgValue::Int(42)),
        ("size",    ArgValue::IntRange(80, 1, 200)),
        ("ratio",   ArgValue::Float(0.8)),
    ],
    |args| {
        let label   = get_text(args, "label");
        let enabled = get_bool(args, "enabled");
        let count   = get_int(args, "count");
        let size    = get_int(args, "size") as u32;
        let ratio   = get_float(args, "ratio");
        html! {
            <MyComponent
                label={label}
                enabled={enabled}
                count={count}
                size={size}
                ratio={ratio}
            />
        }
    }
);"#;

const MIXED_EXAMPLE: &str = r#"// Static variants + interactive mode in one Preview impl
impl Preview for ImageComp {
    fn preview() -> ComponentItem {
        use std::rc::Rc;
        ComponentItem {
            name: "ImageComp".to_string(),
            render: vec![
                ("256".to_string(), html! { <ImageComp src={SRC} size={256u32} /> }),
                ("512".to_string(), html! { <ImageComp src={SRC} size={512u32} /> }),
            ],
            args: Some(InteractiveArgs {
                values: vec![
                    ("src".to_string(),  ArgValue::Text(SRC.to_string())),
                    ("size".to_string(), ArgValue::IntRange(256, 24, 1024)),
                ],
                render_fn: Rc::new(|args| {
                    let src  = get_text(args, "src");
                    let size = get_int(args, "size") as u32;
                    html! { <ImageComp src={src} size={size} /> }
                }),
            }),
            test_cases: vec![],
        }
    }
}"#;

#[derive(Properties, PartialEq, Clone, Default)]
pub struct InteractivePageProps {}

#[function_component(InteractivePage)]
pub fn interactive_page(_props: &InteractivePageProps) -> Html {
    html! {
        <div style="padding: 40px 20px; max-width: 800px; margin: 0 auto; font-family: Arial, sans-serif;">
            <h1 style="font-size: 2rem; color: #24292e; margin: 0 0 8px 0;">
                { "Interactive Previews" }
            </h1>
            <p style="color: #586069; font-size: 1rem; margin: 0 0 32px 0; line-height: 1.6;">
                { "Interactive previews let you edit component props live in the browser — no recompile needed. \
                   Select the " }
                <strong>{ "Interactive" }</strong>
                { " tab in the config panel to see controls for each declared arg." }
            </p>

            // — Arg types table —
            <h2 style="font-size: 1.3rem; color: #24292e; margin: 0 0 16px 0;">{ "Arg Types" }</h2>
            <div style="border: 1px solid #e1e4e8; border-radius: 8px; overflow: hidden; margin-bottom: 40px;">
                <table style="width: 100%; border-collapse: collapse; font-size: 0.875rem;">
                    <thead>
                        <tr style="background: #f6f8fa; border-bottom: 1px solid #e1e4e8;">
                            <th style="text-align: left; padding: 10px 16px; color: #24292e; font-weight: 600;">{ "Variant" }</th>
                            <th style="text-align: left; padding: 10px 16px; color: #24292e; font-weight: 600;">{ "Control" }</th>
                            <th style="text-align: left; padding: 10px 16px; color: #24292e; font-weight: 600;">{ "Description" }</th>
                        </tr>
                    </thead>
                    <tbody>
                        { for ARG_TYPES.iter().enumerate().map(|(i, doc)| {
                            let bg = if i % 2 == 0 { "#fff" } else { "#f6f8fa" };
                            let control = match i {
                                0 => "text input",
                                1 => "checkbox",
                                2 => "number input",
                                3 => "slider",
                                _ => "number input",
                            };
                            html! {
                                <tr style={format!("background: {}; border-bottom: 1px solid #e1e4e8;", bg)}>
                                    <td style="padding: 10px 16px; font-family: monospace; color: #0969da; white-space: nowrap;">
                                        { doc.variant }
                                    </td>
                                    <td style="padding: 10px 16px; color: #57606a;">{ control }</td>
                                    <td style="padding: 10px 16px; color: #57606a; line-height: 1.4;">{ doc.description }</td>
                                </tr>
                            }
                        }) }
                    </tbody>
                </table>
            </div>

            // — Usage snippets per type —
            <h2 style="font-size: 1.3rem; color: #24292e; margin: 0 0 16px 0;">{ "Per-type Usage" }</h2>
            { for ARG_TYPES.iter().map(|doc| html! {
                <div style="margin-bottom: 24px;">
                    <div style="display: flex; align-items: baseline; gap: 12px; margin-bottom: 6px;">
                        <code style="font-size: 0.85rem; background: #f6f8fa; padding: 2px 8px; border-radius: 4px; border: 1px solid #e1e4e8; color: #0969da;">
                            { doc.variant.split('(').next().unwrap_or(doc.variant) }
                        </code>
                        <span style="font-size: 0.85rem; color: #57606a;">{ doc.description }</span>
                    </div>
                    <CodeSnippet
                        code={AttrValue::from(format!("// declare\n{}\n// read in render closure\n{}", doc.snippet, doc.accessor))}
                        label={AttrValue::from("")}
                    />
                </div>
            }) }

            // — Full macro example —
            <h2 style="font-size: 1.3rem; color: #24292e; margin: 0 0 12px 0;">
                { "create_interactive_preview! (all types)" }
            </h2>
            <p style="color: #586069; font-size: 0.9rem; margin: 0 0 12px 0; line-height: 1.5;">
                { "Use " }<code style="background: #f6f8fa; padding: 1px 6px; border-radius: 3px;">{ "create_interactive_preview!" }</code>
                { " when a component has only interactive variants — no static snapshots." }
            </p>
            <CodeSnippet code={AttrValue::from(FULL_EXAMPLE)} label={AttrValue::from("interactive only")} />

            // — Mixed static + interactive —
            <h2 style="font-size: 1.3rem; color: #24292e; margin: 40px 0 12px 0;">
                { "Static Variants + Interactive" }
            </h2>
            <p style="color: #586069; font-size: 0.9rem; margin: 0 0 12px 0; line-height: 1.5;">
                { "Populate both " }
                <code style="background: #f6f8fa; padding: 1px 6px; border-radius: 3px;">{ "render" }</code>
                { " and " }
                <code style="background: #f6f8fa; padding: 1px 6px; border-radius: 3px;">{ "args" }</code>
                { " in " }
                <code style="background: #f6f8fa; padding: 1px 6px; border-radius: 3px;">{ "ComponentItem" }</code>
                { " to get fixed snapshot tabs alongside the live Interactive tab." }
            </p>
            <CodeSnippet code={AttrValue::from(MIXED_EXAMPLE)} label={AttrValue::from("manual Preview impl")} />

            // — Live demo pointer —
            <div style="margin-top: 40px; padding: 16px 20px; background: #ddf4ff; border: 1px solid #54aeff66; border-radius: 8px;">
                <strong style="color: #0969da;">{ "Try it live" }</strong>
                <p style="margin: 6px 0 0 0; color: #0550ae; font-size: 0.9rem; line-height: 1.5;">
                    { "Open " }
                    <strong>{ "Example Components → PropShowcase" }</strong>
                    { " in the sidebar and click the " }
                    <strong>{ "Interactive" }</strong>
                    { " tab. Every arg type above is represented — edit them and watch the component update instantly." }
                </p>
            </div>
        </div>
    }
}

mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(InteractivePage, InteractivePageProps {},);
}
