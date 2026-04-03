use yew::prelude::Html;

use crate::interactive::InteractiveArgs;
use crate::test_utils::{TestCase, TestCases};

pub struct ComponentItem {
    pub name: String,
    /// Pre-rendered variants — populated by `create_preview!`, kept for backward compat.
    pub render: Vec<(String, Html)>,
    /// Live-editable args — populated by `create_interactive_preview!`, `None` for static previews.
    pub args: Option<InteractiveArgs>,
    pub test_cases: Vec<TestCase>,
    /// SSR test runner capturing default props + test cases.
    /// Populated by `create_preview_with_tests!`; `None` otherwise.
    pub ssr_runner: Option<
        std::rc::Rc<
            dyn Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = ()>>>,
        >,
    >,
}

impl Clone for ComponentItem {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            render: self.render.clone(),
            args: self.args.clone(),
            test_cases: self.test_cases.clone(),
            ssr_runner: self.ssr_runner.clone(),
        }
    }
}

impl PartialEq for ComponentItem {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.render == other.render
            && self.args == other.args
            && self.test_cases == other.test_cases
    }
}

impl TestCases for ComponentItem {
    fn get_test_cases(&self) -> Vec<TestCase> {
        self.test_cases.clone()
    }
}
