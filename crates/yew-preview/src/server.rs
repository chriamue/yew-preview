use crate::component_list::ComponentList;
use crate::render::{prerender, esc, GroupData};
use axum::{Router, extract::Query, routing::get};
use serde::Deserialize;
use std::sync::Arc;

// ── HTML generation ───────────────────────────────────────────────────────────

const CSS: &str = r#"
*{box-sizing:border-box;margin:0;padding:0}
body{font-family:Arial,sans-serif;display:flex;height:100vh;overflow:hidden;background:#f6f8fa}
nav{width:220px;flex-shrink:0;background:#24292e;overflow-y:auto;padding:8px 0}
nav h1{font-size:.8rem;font-weight:700;color:#fff;padding:8px 14px 10px;letter-spacing:.06em;text-transform:uppercase}
.gl{font-size:.65rem;text-transform:uppercase;letter-spacing:.08em;color:#8b949e;padding:8px 14px 3px}
nav a{display:block;padding:4px 14px;font-size:.82rem;color:#cdd9e5;text-decoration:none;border-left:3px solid transparent}
nav a:hover{background:#2d333b;color:#fff}
nav a.active{background:#2d333b;color:#fff;border-left-color:#58a6ff}
main{flex:1;overflow-y:auto;padding:24px 28px}
h2{font-size:1.1rem;color:#24292e;margin-bottom:3px}
.meta{font-size:.78rem;color:#8b949e;margin-bottom:16px}
.tabs{display:flex;gap:6px;margin-bottom:12px;flex-wrap:wrap}
.tab{font-size:.78rem;padding:3px 12px;border-radius:12px;text-decoration:none;background:#e8f0fe;color:#1a56a4}
.tab.active{background:#1a56a4;color:#fff}
.preview{border:1px solid #e1e4e8;border-radius:6px;padding:20px;background:#fff;margin-bottom:18px;min-height:60px}
.sl{font-size:.7rem;text-transform:uppercase;letter-spacing:.06em;color:#8b949e;margin-bottom:8px}
ul.tests{list-style:none}
ul.tests li{padding:5px 10px;border-radius:4px;font-size:.82rem;margin-bottom:3px;background:#fff;border:1px solid #e1e4e8}
.tn{color:#24292e;font-weight:500}
.tm{color:#6e7781;font-size:.74rem;margin-top:2px}
.empty{color:#8b949e;font-style:italic;font-size:.82rem}
"#;

fn render_page(groups: &[GroupData], sel_g: &str, sel_c: &str, sel_v: &str) -> String {
    let nav: String = groups
        .iter()
        .map(|g| {
            let links: String = g
                .components
                .iter()
                .map(|c| {
                    let active = g.name == sel_g && c.name == sel_c;
                    format!(
                        r#"<a href="/?g={g}&c={c}"{cls}>{label}</a>"#,
                        g = enc(&g.name),
                        c = enc(&c.name),
                        cls = if active { r#" class="active""# } else { "" },
                        label = esc(&c.name),
                    )
                })
                .collect();
            format!(r#"<div class="gl">{}</div>{}"#, esc(&g.name), links)
        })
        .collect();

    let content = groups
        .iter()
        .find(|g| g.name == sel_g)
        .and_then(|g| g.components.iter().find(|c| c.name == sel_c))
        .map(|c| {
            let active_v = if sel_v.is_empty() {
                c.variants.first().map(|v| v.name.as_str()).unwrap_or("")
            } else {
                sel_v
            };

            let tabs: String = c
                .variants
                .iter()
                .map(|v| {
                    format!(
                        r#"<a class="tab{act}" href="/?g={g}&c={c}&v={v}">{label}</a>"#,
                        act = if v.name == active_v { " active" } else { "" },
                        g = enc(sel_g),
                        c = enc(&c.name),
                        v = enc(&v.name),
                        label = esc(&v.name),
                    )
                })
                .collect();

            let preview = c
                .variants
                .iter()
                .find(|v| v.name == active_v)
                .map(|v| format!(r#"<div class="preview">{}</div>"#, v.html))
                .unwrap_or_else(|| {
                    if c.has_interactive {
                        r#"<div class="preview"><span class="empty">interactive — use trunk for live editing</span></div>"#
                            .to_string()
                    } else {
                        String::new()
                    }
                });

            let tests: String = if c.tests.is_empty() {
                r#"<span class="empty">no test cases</span>"#.to_string()
            } else {
                c.tests
                    .iter()
                    .map(|t| {
                        let ms: String = t
                            .matchers
                            .iter()
                            .map(|m| format!("<div>{}</div>", esc(m)))
                            .collect();
                        format!(
                            r#"<li><div class="tn">{}</div><div class="tm">{}</div></li>"#,
                            esc(&t.name),
                            ms
                        )
                    })
                    .collect()
            };

            format!(
                r#"<h2>{n}</h2>
<div class="meta">{nv} variant{vs} · {nt} test{ts}{ia}</div>
<div class="tabs">{tabs}</div>{preview}
<div class="sl">Test Cases</div><ul class="tests">{tests}</ul>"#,
                n = esc(&c.name),
                nv = c.variants.len(),
                vs = if c.variants.len() == 1 { "" } else { "s" },
                nt = c.tests.len(),
                ts = if c.tests.len() == 1 { "" } else { "s" },
                ia = if c.has_interactive { " · interactive" } else { "" },
            )
        })
        .unwrap_or_else(|| {
            r#"<p class="empty">Select a component from the sidebar.</p>"#.to_string()
        });

    format!(
        r#"<!DOCTYPE html><html lang="en"><head>
<meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>YewPreview</title><style>{CSS}</style>
</head><body>
<nav><h1>YewPreview</h1>{nav}</nav>
<main>{content}</main>
</body></html>"#
    )
}

fn enc(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' => vec![c],
            ' ' => vec!['+'],
            c => format!("%{:02X}", c as u32).chars().collect(),
        })
        .collect()
}

// ── Public API ────────────────────────────────────────────────────────────────

#[derive(Deserialize, Default)]
struct Sel {
    g: Option<String>,
    c: Option<String>,
    v: Option<String>,
}

async fn start_server(rendered: Arc<Vec<GroupData>>, port: u16) {
    let dg = rendered
        .first()
        .map(|g| g.name.clone())
        .unwrap_or_default();
    let dc = rendered
        .first()
        .and_then(|g| g.components.first())
        .map(|c| c.name.clone())
        .unwrap_or_default();

    let router = Router::new().route(
        "/",
        get({
            let rendered = Arc::clone(&rendered);
            let dg = dg.clone();
            let dc = dc.clone();
            move |Query(sel): Query<Sel>| {
                let rendered = Arc::clone(&rendered);
                let g = sel.g.unwrap_or_else(|| dg.clone());
                let c = sel.c.unwrap_or_else(|| dc.clone());
                let v = sel.v.unwrap_or_default();
                async move { axum::response::Html(render_page(&rendered, &g, &c, &v)) }
            }
        }),
    );

    let listener = match tokio::net::TcpListener::bind(format!("0.0.0.0:{port}")).await {
        Ok(l) => l,
        Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
            eprintln!("error: port {port} is already in use");
            eprintln!("hint:  set a different port via the PORT environment variable");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("error: failed to bind port {port}: {e}");
            std::process::exit(1);
        }
    };
    println!("yew-preview: http://localhost:{port}");
    axum::serve(listener, router).await.expect("server error");
}

/// Pre-render all component variants with Yew SSR, then serve with axum.
/// All HTML is generated from `groups` — no filesystem access.
///
/// Call this from a native `main` (not WASM). Add to your project:
///
/// ```rust,ignore
/// // examples/serve.rs
/// fn main() {
///     yew_preview::serve_blocking(my_app::preview_groups(), 8080);
/// }
/// ```
///
/// Then run: `cargo run --example serve --features yew-preview/serve`
pub fn serve_blocking(groups: ComponentList, port: u16) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");

    // Yew's LocalServerRenderer requires a LocalSet (uses spawn_local).
    let rendered = rt.block_on(async {
        tokio::task::LocalSet::new()
            .run_until(prerender(groups))
            .await
    });

    rt.block_on(start_server(Arc::new(rendered), port));
}
