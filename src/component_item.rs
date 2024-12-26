use yew::prelude::Html;

use crate::test_utils::{TestCase, TestCases};

#[derive(Clone, PartialEq)]
pub struct ComponentItem {
    pub name: String,
    pub render: Vec<(String, Html)>,
    pub test_cases: Vec<TestCase>,
}

impl TestCases for ComponentItem {
    fn get_test_cases(&self) -> Vec<TestCase> {
        self.test_cases.clone()
    }
}
