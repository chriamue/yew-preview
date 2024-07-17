pub mod component_macro;
mod component_preview;
mod component_selector;
mod config_panel;
mod preview_page;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ComponentItem {
    pub name: String,
    pub render: Vec<(String, Html)>,
}

pub type ComponentList = Vec<ComponentItem>;

pub mod prelude {
    pub use crate::component_preview::ComponentPreview;
    pub use crate::component_selector::ComponentSelector;
    pub use crate::config_panel::ConfigPanel;
    pub use crate::preview_page::PreviewPage;
    pub use crate::{ComponentItem, ComponentList};
}
