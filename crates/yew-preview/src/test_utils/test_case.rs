use super::matchers::Matcher;

#[derive(Debug, PartialEq, Clone)]
pub struct TestCase {
    pub name: String,
    pub matchers: Vec<Matcher>,
}

impl TestCase {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            matchers: Vec::new(),
        }
    }

    pub fn matches(&self, html: &str) -> bool {
        self.matchers.iter().all(|matcher| matcher.matches(html))
    }
}
