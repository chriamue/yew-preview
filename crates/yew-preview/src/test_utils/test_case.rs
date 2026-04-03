use super::matchers::Matcher;

#[derive(Debug, PartialEq, Clone)]
pub struct MatcherResult {
    pub description: String,
    pub passed: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TestCaseResult {
    pub name: String,
    pub passed: bool,
    pub matchers: Vec<MatcherResult>,
}

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

    pub fn run(&self, html: &str) -> TestCaseResult {
        let matchers = self
            .matchers
            .iter()
            .map(|m| MatcherResult {
                description: m.description(),
                passed: m.matches(html),
            })
            .collect::<Vec<_>>();
        let passed = matchers.iter().all(|r| r.passed);
        TestCaseResult {
            name: self.name.clone(),
            passed,
            matchers,
        }
    }
}
