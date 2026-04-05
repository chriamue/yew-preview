//! Static HTML catalog generation.
//!
//! Renders every component variant via Yew SSR and writes a single self-contained
//! HTML file — a long-scroll, print-friendly catalog of all components.
//!
//! # Example
//!
//! ```rust,ignore
//! // examples/catalog.rs
//! fn main() {
//!     yew_preview::generate_catalog_blocking(
//!         my_app::preview_groups(),
//!         yew_preview::CatalogOptions::new("my-app")
//!             .css_file("path/to/my-app.css"),
//!     );
//! }
//! ```

use crate::component_list::ComponentList;
use crate::render::{esc, prerender, ComponentData, GroupData, VariantData};
use std::path::PathBuf;

// ── Built-in catalog structure styles ─────────────────────────────────────────
//
// All classes are prefixed with `yp-` to avoid colliding with user component CSS.
// No sticky/fixed positioning — the page is print-friendly by default.

const CATALOG_CSS: &str = "
/* yew-preview catalog */
*, *::before, *::after { box-sizing: border-box; }

body {
  margin: 0;
  font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
  background: #fff;
  color: #1e293b;
}

/* ── Cover (title + TOC) ── */
.yp-cover {
  padding: 3rem 2rem 2rem;
  border-bottom: 3px solid #0f172a;
  max-width: 1440px;
  margin: 0 auto;
}

.yp-cover__title {
  font-size: 2rem;
  font-weight: 800;
  color: #0f172a;
  margin: 0 0 0.25rem;
  letter-spacing: -0.02em;
}

.yp-cover__meta {
  font-size: 0.82rem;
  color: #64748b;
  margin: 0 0 1.5rem;
}

/* ── Table of contents ── */
.yp-toc {
  margin-top: 1rem;
}

.yp-toc__label {
  font-size: 0.7rem;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.08em;
  color: #94a3b8;
  margin: 0 0 0.5rem;
}

.yp-toc__list {
  list-style: none;
  margin: 0;
  padding: 0;
  display: flex;
  flex-wrap: wrap;
  gap: 0.25rem 1.5rem;
}

.yp-toc__item {
  font-size: 0.85rem;
  display: flex;
  align-items: baseline;
  gap: 0.375rem;
}

.yp-toc__link {
  color: #1d4ed8;
  text-decoration: none;
}

.yp-toc__link:hover {
  text-decoration: underline;
}

.yp-toc__count {
  font-size: 0.75rem;
  color: #94a3b8;
}

/* ── Main content ── */
.yp-main {
  padding: 2rem;
  max-width: 1440px;
  margin: 0 auto;
}

/* ── Group section ── */
.yp-group {
  margin-bottom: 4rem;
}

.yp-group__title {
  font-size: 1.3rem;
  font-weight: 700;
  color: #0f172a;
  padding-bottom: 0.5rem;
  border-bottom: 2px solid #cbd5e1;
  margin: 0 0 1.5rem;
  scroll-margin-top: 1rem;
}

/* ── Component card ── */
.yp-component {
  background: #fff;
  border: 1px solid #e2e8f0;
  border-radius: 0.5rem;
  overflow: hidden;
  margin-bottom: 1.5rem;
  box-shadow: 0 1px 3px rgba(0,0,0,.06);
}

.yp-component__header {
  display: flex;
  align-items: center;
  gap: 0.625rem;
  padding: 0.625rem 1rem;
  background: #f8fafc;
  border-bottom: 1px solid #e2e8f0;
}

.yp-component__name {
  font-size: 0.9rem;
  font-weight: 600;
  color: #1e293b;
  font-family: ui-monospace, 'Cascadia Code', 'Source Code Pro', monospace;
}

.yp-component__badge {
  font-size: 0.65rem;
  font-weight: 500;
  padding: 0.125rem 0.5rem;
  border-radius: 1rem;
  line-height: 1.4;
}

.yp-component__badge--interactive {
  background: #dbeafe;
  color: #1d4ed8;
}

.yp-component__badge--tests {
  background: #dcfce7;
  color: #15803d;
}

/* ── Variants row ── */
.yp-variants {
  display: flex;
  flex-wrap: wrap;
}

.yp-variant {
  flex: 1 1 320px;
  min-width: 0;
  border-right: 1px solid #f1f5f9;
}

.yp-variant:last-child {
  border-right: none;
}

.yp-variant__label {
  font-size: 0.65rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.07em;
  color: #94a3b8;
  padding: 0.375rem 1rem;
  background: #fafafa;
  border-bottom: 1px solid #f1f5f9;
}

.yp-variant__preview {
  padding: 1.25rem;
}

.yp-variant--interactive .yp-variant__preview {
  color: #94a3b8;
  font-style: italic;
  font-size: 0.82rem;
}

/* ── Test cases (collapsible) ── */
.yp-tests {
  border-top: 1px solid #f1f5f9;
}

.yp-tests__summary {
  font-size: 0.72rem;
  color: #64748b;
  cursor: pointer;
  user-select: none;
  padding: 0.5rem 1rem;
  background: #fafafa;
  list-style: none;
  display: flex;
  align-items: center;
  gap: 0.375rem;
}

.yp-tests__summary::marker,
.yp-tests__summary::-webkit-details-marker {
  display: none;
}

.yp-tests__summary::before {
  content: '\\25B6';
  font-size: 0.55rem;
  display: inline-block;
  transition: transform 0.15s;
}

details[open] > .yp-tests__summary::before {
  transform: rotate(90deg);
}

.yp-tests__list {
  list-style: none;
  margin: 0;
  padding: 0.5rem 1rem 0.75rem;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  background: #fafafa;
}

.yp-tests__item {
  font-size: 0.78rem;
  padding: 0.375rem 0.625rem;
  border-radius: 0.25rem;
  background: #fff;
  border: 1px solid #e2e8f0;
}

.yp-tests__item-name {
  font-weight: 500;
  color: #334155;
}

.yp-tests__item-matchers {
  margin-top: 0.125rem;
  color: #6e7781;
  font-size: 0.72rem;
  font-family: ui-monospace, 'Cascadia Code', monospace;
}

/* ── Print ── */
@media print {
  .yp-group { break-before: page; }
  .yp-component { break-inside: avoid; box-shadow: none; }
  .yp-variant { flex-basis: 50%; }
  .yp-tests { display: none; }
}

/* ── Responsive ── */
@media (max-width: 640px) {
  .yp-cover { padding: 1.5rem 1rem 1rem; }
  .yp-main { padding: 1rem; }
  .yp-variant { flex-basis: 100%; border-right: none; border-bottom: 1px solid #f1f5f9; }
  .yp-variant:last-child { border-bottom: none; }
}
";

// ── HTML building blocks ──────────────────────────────────────────────────────

/// Convert a display name into a URL-safe `id` slug.
fn slug(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            'A'..='Z' => c.to_ascii_lowercase(),
            'a'..='z' | '0'..='9' => c,
            _ => '-',
        })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn render_variant(v: &VariantData) -> String {
    format!(
        concat!(
            "<div class=\"yp-variant\">",
            "<div class=\"yp-variant__label\">{label}</div>",
            "<div class=\"yp-variant__preview\">{html}</div>",
            "</div>",
        ),
        label = esc(&v.name),
        html = v.html,
    )
}

fn render_component(c: &ComponentData) -> String {
    let mut badges = String::new();
    if c.has_interactive {
        badges.push_str(
            "<span class=\"yp-component__badge yp-component__badge--interactive\">interactive</span>",
        );
    }
    if !c.tests.is_empty() {
        badges.push_str(&format!(
            "<span class=\"yp-component__badge yp-component__badge--tests\">{n} test{s}</span>",
            n = c.tests.len(),
            s = if c.tests.len() == 1 { "" } else { "s" },
        ));
    }

    let variants_html: String = if c.variants.is_empty() && c.has_interactive {
        concat!(
            "<div class=\"yp-variant yp-variant--interactive\">",
            "<div class=\"yp-variant__label\">interactive</div>",
            "<div class=\"yp-variant__preview\">",
            "Live-editable \u{2014} use <code>trunk serve</code> for an interactive preview.",
            "</div></div>",
        ).to_string()
    } else {
        c.variants.iter().map(render_variant).collect()
    };

    let tests_html: String = if c.tests.is_empty() {
        String::new()
    } else {
        let items: String = c.tests.iter().map(|t| {
            let matchers: String = t.matchers.iter()
                .map(|m| format!("<div>{}</div>", esc(m)))
                .collect();
            format!(
                concat!(
                    "<li class=\"yp-tests__item\">",
                    "<div class=\"yp-tests__item-name\">{name}</div>",
                    "<div class=\"yp-tests__item-matchers\">{matchers}</div>",
                    "</li>",
                ),
                name = esc(&t.name),
                matchers = matchers,
            )
        }).collect();
        format!(
            concat!(
                "<details class=\"yp-tests\" open>",
                "<summary class=\"yp-tests__summary\">{n} test case{s}</summary>",
                "<ul class=\"yp-tests__list\">{items}</ul>",
                "</details>",
            ),
            n = c.tests.len(),
            s = if c.tests.len() == 1 { "" } else { "s" },
            items = items,
        )
    };

    format!(
        concat!(
            "<article class=\"yp-component\" id=\"component-{id}\">",
            "<div class=\"yp-component__header\">",
            "<span class=\"yp-component__name\">{name}</span>{badges}",
            "</div>",
            "<div class=\"yp-variants\">{variants}</div>",
            "{tests}",
            "</article>",
        ),
        id = slug(&c.name),
        name = esc(&c.name),
        badges = badges,
        variants = variants_html,
        tests = tests_html,
    )
}

fn render_group(g: &GroupData) -> String {
    let components: String = g.components.iter().map(render_component).collect();
    format!(
        concat!(
            "<section class=\"yp-group\" id=\"group-{id}\">",
            "<h2 class=\"yp-group__title\">{name}</h2>",
            "{components}",
            "</section>",
        ),
        id = slug(&g.name),
        name = esc(&g.name),
        components = components,
    )
}

/// Build the full catalog HTML document.
fn generate_html(groups: &[GroupData], title: &str, extra_css: Option<&str>) -> String {
    let total_groups = groups.len();
    let total_components: usize = groups.iter().map(|g| g.components.len()).sum();
    let total_variants: usize = groups
        .iter()
        .flat_map(|g| g.components.iter())
        .map(|c| c.variants.len())
        .sum();

    // TOC — one entry per group showing component count
    let toc: String = groups
        .iter()
        .map(|g| {
            let n = g.components.len();
            format!(
                concat!(
                    "<li class=\"yp-toc__item\">",
                    "<a class=\"yp-toc__link\" href=\"#group-{id}\">{name}</a>",
                    "<span class=\"yp-toc__count\">{n}&nbsp;component{s}</span>",
                    "</li>",
                ),
                id = slug(&g.name),
                name = esc(&g.name),
                n = n,
                s = if n == 1 { "" } else { "s" },
            )
        })
        .collect::<Vec<_>>()
        .join("\n        ");

    let body: String = groups
        .iter()
        .map(render_group)
        .collect::<Vec<_>>()
        .join("\n");

    // User CSS injected after scaffold styles so component CSS takes effect
    // inside .yp-variant__preview regions.
    let user_css = extra_css
        .map(|css| format!("  <style>\n{css}\n  </style>\n"))
        .unwrap_or_default();

    // Build page sections separately to avoid one giant format! string
    let head = format!(
        concat!(
            "<!DOCTYPE html>\n",
            "<html lang=\"en\">\n",
            "<head>\n",
            "  <meta charset=\"utf-8\" />\n",
            "  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\" />\n",
            "  <title>{title} \u{2014} Component Catalog</title>\n",
            "  <style>{css}</style>\n",
            "{user_css}",
            "</head>\n",
        ),
        title = esc(title),
        css = CATALOG_CSS,
        user_css = user_css,
    );

    let cover = format!(
        concat!(
            "<div class=\"yp-cover\">\n",
            "  <h1 class=\"yp-cover__title\">{title}</h1>\n",
            "  <p class=\"yp-cover__meta\">",
            "{total_groups}&nbsp;group{gs} \u{00b7} ",
            "{total_components}&nbsp;component{cs} \u{00b7} ",
            "{total_variants}&nbsp;variant{vs}",
            "</p>\n",
            "  <nav class=\"yp-toc\" aria-label=\"Contents\">\n",
            "    <p class=\"yp-toc__label\">Contents</p>\n",
            "    <ol class=\"yp-toc__list\">\n",
            "        {toc}\n",
            "    </ol>\n",
            "  </nav>\n",
            "</div>\n",
        ),
        title = esc(title),
        total_groups = total_groups,
        gs = if total_groups == 1 { "" } else { "s" },
        total_components = total_components,
        cs = if total_components == 1 { "" } else { "s" },
        total_variants = total_variants,
        vs = if total_variants == 1 { "" } else { "s" },
        toc = toc,
    );

    format!(
        concat!(
            "{head}",
            "<body>\n",
            "{cover}",
            "<main class=\"yp-main\">\n",
            "{body}\n",
            "</main>\n",
            "<!-- generated by yew-preview | {tc} components, {tv} variants -->\n",
            "</body>\n",
            "</html>\n",
        ),
        head = head,
        cover = cover,
        body = body,
        tc = total_components,
        tv = total_variants,
    )
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Configuration for [`generate_catalog_blocking`].
#[derive(Debug, Clone)]
pub struct CatalogOptions {
    /// Project / crate name — used as the page title and as the default output
    /// filename (`{project_name}-yew-preview.html`).
    pub project_name: String,
    /// Path to a CSS file whose contents are injected into the catalog `<head>`
    /// after the built-in scaffold styles, so your component CSS takes effect
    /// inside each `.yp-variant__preview` region.
    pub css_file: Option<PathBuf>,
    /// Where to write the HTML file.
    /// Defaults to `{project_name}-yew-preview.html` in the current directory.
    pub output: Option<PathBuf>,
}

impl CatalogOptions {
    /// Create options with just a project name; all other fields default.
    pub fn new(project_name: impl Into<String>) -> Self {
        Self {
            project_name: project_name.into(),
            css_file: None,
            output: None,
        }
    }

    /// Inject the contents of `path` as a `<style>` block in the catalog.
    pub fn css_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.css_file = Some(path.into());
        self
    }

    /// Write the catalog to `path` instead of the default filename.
    pub fn output(mut self, path: impl Into<PathBuf>) -> Self {
        self.output = Some(path.into());
        self
    }
}

/// Pre-render all component variants via Yew SSR and write a single static
/// HTML catalog file.
///
/// This is the blocking (synchronous) entry-point — call it from a plain
/// `fn main()` in a native binary or Cargo example.
///
/// Returns the path of the file that was written.
///
/// # Panics
///
/// Panics if the tokio runtime cannot be created or if the output file cannot
/// be written.
pub fn generate_catalog_blocking(groups: ComponentList, options: CatalogOptions) -> PathBuf {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");

    // Yew's LocalServerRenderer requires a LocalSet (uses spawn_local).
    let rendered = rt.block_on(async {
        tokio::task::LocalSet::new()
            .run_until(prerender(groups))
            .await
    });

    let extra_css = options.css_file.as_ref().and_then(|p| {
        std::fs::read_to_string(p)
            .map_err(|e| {
                eprintln!(
                    "yew-preview: warning: could not read CSS file {}: {e}",
                    p.display()
                )
            })
            .ok()
    });

    let html = generate_html(&rendered, &options.project_name, extra_css.as_deref());

    let output = options.output.unwrap_or_else(|| {
        PathBuf::from(format!("{}-yew-preview.html", options.project_name))
    });

    std::fs::write(&output, &html).unwrap_or_else(|e| {
        panic!(
            "yew-preview: failed to write catalog to {}: {e}",
            output.display()
        )
    });

    println!("yew-preview: catalog written to {}", output.display());
    output
}
