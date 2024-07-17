mod component_preview;
mod component_selector;
mod preview_page;

use yew::Html;

#[derive(Clone, PartialEq)]
pub struct ComponentItem {
    pub name: String,
    pub render: Html,
    pub props: String,
}

pub type ComponentList = Vec<ComponentItem>;

pub mod prelude {
    pub use crate::component_preview::ComponentPreview;
    pub use crate::component_selector::ComponentSelector;
    pub use crate::preview_page::PreviewPage;
    pub use crate::{ComponentItem, ComponentList};
}
