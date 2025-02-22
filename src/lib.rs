mod component_group;
mod component_item;
mod component_list;
mod component_preview;
mod component_selector;
mod config_panel;
pub mod examples;
mod group_selector;
pub mod macros;
mod main_content;
mod preview;
mod preview_page;
mod search_bar;
mod search_results;
mod sidebar;
pub mod test_utils;

pub mod prelude {
    pub use crate::component_group::ComponentGroup;
    pub use crate::component_item::ComponentItem;
    pub use crate::component_list::ComponentList;
    pub use crate::component_preview::ComponentPreview;
    pub use crate::component_selector::ComponentSelector;
    pub use crate::config_panel::ConfigPanel;
    pub use crate::group_selector::GroupSelector;
    pub use crate::main_content::MainContent;
    pub use crate::preview::Preview;
    pub use crate::preview_page::PreviewPage;
    pub use crate::search_bar::SearchBar;
    pub use crate::search_results::SearchResults;
    pub use crate::sidebar::Sidebar;
    pub use crate::test_utils::helpers::*;
    pub use crate::test_utils::Matcher;
    pub use crate::test_utils::TestCase;
    pub use crate::test_utils::TestCases;
}
