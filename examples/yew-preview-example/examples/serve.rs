/// Serve the component preview with a local axum server.
///
/// Run with:
///   cargo run --example serve
///
/// Then open http://localhost:8080
fn main() {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8080);

    yew_preview::serve_blocking(yew_preview_example::preview_groups(), port);
}
