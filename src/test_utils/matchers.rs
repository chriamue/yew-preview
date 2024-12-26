#[derive(Debug, Clone, PartialEq)]
pub enum Matcher {
    Contains(String),
    HasAttribute(String, String),
    HasClass(String),
    HasStyle(String, String),
    HasText(String),
    Exists(String), // selector
}

impl Matcher {
    pub fn matches(&self, html: &str) -> bool {
        match self {
            Matcher::Contains(text) => html.contains(text),
            Matcher::HasAttribute(attr, value) => html.contains(&format!("{}=\"{}\"", attr, value)),
            Matcher::HasClass(class) => html.contains(&format!("class=\"{}\"", class)),
            Matcher::HasStyle(prop, value) => html.contains(&format!("{}: {}", prop, value)),
            Matcher::HasText(text) => html.contains(text),
            Matcher::Exists(selector) => html.contains(selector),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matcher_contains() {
        let matcher = Matcher::Contains("Hello".to_string());
        assert!(matcher.matches("Hello, world!"));
        assert!(!matcher.matches("Goodbye, world!"));
    }

    #[test]
    fn test_matcher_has_attribute() {
        let matcher = Matcher::HasAttribute("class".to_string(), "foo".to_string());
        assert!(matcher.matches("<div class=\"foo\"></div>"));
        assert!(!matcher.matches("<div class=\"bar\"></div>"));
    }

    #[test]
    fn test_matcher_has_class() {
        let matcher = Matcher::HasClass("foo".to_string());
        assert!(matcher.matches("<div class=\"foo\"></div>"));
        assert!(!matcher.matches("<div class=\"bar\"></div>"));
    }

    #[test]
    fn test_matcher_has_style() {
        let matcher = Matcher::HasStyle("color".to_string(), "red".to_string());
        assert!(matcher.matches("<div style=\"color: red;\"></div>"));
        assert!(!matcher.matches("<div style=\"color: blue;\"></div>"));
    }

    #[test]
    fn test_matcher_has_text() {
        let matcher = Matcher::HasText("Hello".to_string());
        assert!(matcher.matches("<div>Hello, world!</div>"));
        assert!(!matcher.matches("<div>Goodbye, world!</div>"));
    }

    #[test]
    fn test_matcher_exists() {
        let matcher = Matcher::Exists("div".to_string());
        assert!(matcher.matches("<div></div>"));
        assert!(!matcher.matches("<span></span>"));
    }

    #[test]
    fn test_test_case_matches() {
        let test_case = TestCase {
            name: "Test".to_string(),
            matchers: vec![
                Matcher::Contains("Hello".to_string()),
                Matcher::HasAttribute("class".to_string(), "foo".to_string()),
            ],
        };

        assert!(test_case.matches("<div class=\"foo\">Hello, world!</div>"));
        assert!(!test_case.matches("<div class=\"bar\">Goodbye, world!</div>"));
    }

    #[test]
    fn test_test_case_new() {
        let test_case = TestCase::new("Test");
        assert_eq!(test_case.name, "Test");
        assert!(test_case.matchers.is_empty());
    }
}
