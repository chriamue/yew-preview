pub use super::matchers::Matcher;

pub fn has_text(text: &str) -> Matcher {
    Matcher::HasText(text.to_string())
}

pub fn has_class(class: &str) -> Matcher {
    Matcher::HasClass(class.to_string())
}

pub fn has_style(property: &str, value: &str) -> Matcher {
    Matcher::HasStyle(property.to_string(), value.to_string())
}

pub fn has_attribute(attr: &str, value: &str) -> Matcher {
    Matcher::HasAttribute(attr.to_string(), value.to_string())
}

pub fn exists(selector: &str) -> Matcher {
    Matcher::Exists(selector.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_text() {
        let matcher = has_text("Hello, world!");
        assert!(matcher.matches("Hello, world!"));
        assert!(!matcher.matches("Goodbye, world!"));
    }

    #[test]
    fn test_has_class() {
        let matcher = has_class("foo");
        assert!(matcher.matches("<div class=\"foo\"></div>"));
        assert!(!matcher.matches("<div class=\"bar\"></div>"));
    }

    #[test]
    fn test_has_style() {
        let matcher = has_style("color", "red");
        assert!(matcher.matches("<div style=\"color: red;\"></div>"));
        assert!(!matcher.matches("<div style=\"color: blue;\"></div>"));
    }

    #[test]
    fn test_has_attribute() {
        let matcher = has_attribute("class", "foo");
        assert!(matcher.matches("<div class=\"foo\"></div>"));
        assert!(!matcher.matches("<div class=\"bar\"></div>"));
    }

    #[test]
    fn test_exists() {
        let matcher = exists("div");
        assert!(matcher.matches("<div></div>"));
        assert!(!matcher.matches("<span></span>"));
    }
}
