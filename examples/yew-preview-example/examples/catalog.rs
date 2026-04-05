//! Generate a static HTML component catalog.
//!
//! Renders every component variant via Yew SSR and writes a single
//! self-contained HTML file that shows all components in a long-scroll layout.
//!
//! Run with:
//!   cargo run --example catalog
//!
//! The output file defaults to `yew-preview-example-yew-preview.html` in the
//! current working directory. Override with the `OUTPUT` environment variable:
//!   OUTPUT=dist/catalog.html cargo run --example catalog

fn main() {
    let css_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("yew-preview.css");

    let output = std::env::var("OUTPUT").ok().map(std::path::PathBuf::from);

    let mut options = yew_preview::CatalogOptions::new("yew-preview-example").css_file(css_path);

    if let Some(out) = output {
        options = options.output(out);
    }

    yew_preview::generate_catalog_blocking(yew_preview_example::preview_groups(), options);
}
