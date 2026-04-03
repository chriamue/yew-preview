use yew::prelude::Html;

use crate::interactive::InteractiveArgs;
use crate::test_utils::{TestCase, TestCases};

#[derive(Clone, PartialEq)]
pub struct ComponentItem {
    pub name: String,
    /// Pre-rendered variants — populated by `create_preview!`, kept for backward compat.
    pub render: Vec<(String, Html)>,
    /// Live-editable args — populated by `create_interactive_preview!`, `None` for static previews.
    pub args: Option<InteractiveArgs>,
    pub test_cases: Vec<TestCase>,
}

impl TestCases for ComponentItem {
    fn get_test_cases(&self) -> Vec<TestCase> {
        self.test_cases.clone()
    }
}
