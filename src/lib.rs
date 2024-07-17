pub mod component_macro;
mod component_preview;
mod component_selector;
mod config_panel;
mod preview_page;

use yew::Html;

#[derive(Clone, PartialEq)]
pub struct ComponentItem {
    pub name: String,
    pub render: Html,
    pub props: Vec<(String, String)>,
}

pub type ComponentList = Vec<ComponentItem>;

pub mod prelude {
    pub use crate::component_preview::ComponentPreview;
    pub use crate::component_selector::ComponentSelector;
    pub use crate::config_panel::ConfigPanel;
    pub use crate::preview_page::PreviewPage;
    pub use crate::{ComponentItem, ComponentList};
}
