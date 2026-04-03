---
title: Testing
tags: [testing, matchers, test-utils]
---

# Testing

← [[index]]

YewPreview ships built-in test utilities behind the `testing` feature flag. They use Yew's `LocalServerRenderer` for server-side rendering so tests run without a browser.

## Enabling the Testing Feature

```toml
[dev-dependencies]
yew-preview = { git = "https://github.com/chriamue/yew-preview", features = ["testing"] }
```

## `create_preview_with_tests!`

The primary way to add SSR tests. Embed test cases directly in the preview declaration — the macro generates both `impl Preview` and a `#[tokio::test]` that runs every test against every variant:

```rust
use yew_preview::test_utils::exists;
use yew_preview::{create_preview_with_tests, prelude::*};

create_preview_with_tests!(
    component: Button,
    default_props: ButtonProps { label: "Click me".to_string(), disabled: false },
    variants: [
        ("Disabled", ButtonProps { label: "Disabled".to_string(), disabled: true }),
    ],
    tests: [
        ("Has button element", exists("button")),
        ("Has label text",     Matcher::HasText("Click me".to_string())),
    ]
);
```

Each test tuple is `("name", matcher1, matcher2, ...)`. Multiple matchers in one test case all run against the same rendered HTML.

## Test Execution Flow

```
cargo test
  └─ {component}_preview::default
       └─ for each variant
            └─ render_component(props)  →  HTML string
                 └─ for each TestCase
                      └─ all matchers pass?  →  ✓ / panic with failure details
```

Failures report which variant and matcher failed, plus the source location:

```
thread '...' panicked at src/components/card.rs:30:1:
[CardComp] [Default] Test 'Has close button' failed:
    ✗ element exists: button
```

## Integration Test for All Components

Call `run_groups_tests` from a single test to cover every component at once:

```rust
#[cfg(test)]
mod tests {
    use super::get_groups;
    use yew_preview::test_utils::run_groups_tests;

    #[tokio::test]
    async fn test_all_components() {
        run_groups_tests(&get_groups()).await;
    }
}
```

## Matchers

`Matcher` is an enum that describes one assertion against rendered HTML.

| Variant | Checks |
|---|---|
| `Contains(String)` | Raw HTML contains substring |
| `HasText(String)` | Visible text content contains value |
| `HasClass(String)` | Any element has CSS class |
| `HasStyle(String, String)` | Any element has inline style `property: value` |
| `HasAttribute(String, String)` | Any element has `attribute="value"` |
| `Exists(String)` | CSS selector matches at least one element |
| `ElementCount(String, usize)` | CSS selector matches exactly N elements |

### Helper Functions

```rust
use yew_preview::test_utils::*;

exists("h1")                         // Matcher::Exists
has_text("Hello")                    // Matcher::HasText
has_class("btn-primary")             // Matcher::HasClass
has_style("color", "red")            // Matcher::HasStyle
has_attribute("disabled", "true")    // Matcher::HasAttribute
```

## `render_component`

Low-level async function for rendering a component to an HTML string:

```rust
let html = yew_preview::test_utils::render_component::<MyComp>(props).await;
assert!(html.contains("expected text"));
```

## Failing Tests and `tests-ignored`

When a test fails it is visible in both `cargo test` output and the browser UI test panel. This is intentional — the browser shows which test cases pass or fail for each variant.

To keep CI green while a test is known-failing, enable the `tests-ignored` feature on `yew-preview`. All macro-generated tests are marked `#[ignore]`:

```toml
[dev-dependencies]
yew-preview = { git = "...", features = ["testing", "tests-ignored"] }
```

```
test components::card::card_comp_preview::default ... ignored, tests-ignored feature enabled
test result: ok. 0 passed; 0 failed; 7 ignored
```

Run the ignored tests explicitly at any time:

```bash
cargo test -- --ignored
```

The `tests-ignored` feature only affects the generated `#[tokio::test]` functions. The test cases are still stored in `ComponentItem.test_cases` and shown in the browser UI regardless.
