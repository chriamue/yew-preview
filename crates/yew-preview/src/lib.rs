mod component_group;
mod component_item;
mod component_list;
mod component_preview;
mod component_selector;
mod config_panel;
mod group_selector;
pub mod interactive;
mod macros;
mod preview;
mod preview_page;
mod search_bar;
mod search_results;
pub mod test_utils;

pub mod prelude {
    // Core types
    pub use crate::component_group::ComponentGroup;
    pub use crate::component_item::ComponentItem;
    pub use crate::component_list::ComponentList;
    pub use crate::preview::Preview;
    pub use crate::preview_page::PreviewPage;

    // Interactive arg types and helpers
    pub use crate::interactive::ArgValue;
    pub use crate::interactive::InteractiveArgs;
    pub use crate::interactive::{get_bool, get_float, get_int, get_text};

    // Test types (data structures, always available)
    pub use crate::test_utils::helpers::*;
    pub use crate::test_utils::Matcher;
    pub use crate::test_utils::TestCase;
    pub use crate::test_utils::TestCases;
}
