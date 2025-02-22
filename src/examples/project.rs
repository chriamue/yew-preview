use crate::{create_preview_with_tests, prelude::*};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone, Default)]
pub struct ProjectCompProps {
    pub title: String,
    pub description: String,
    pub url: String,
    pub repo: Option<String>,
}

#[function_component(ProjectComp)]
pub fn project_comp(props: &ProjectCompProps) -> Html {
    html! {
        <div style="border: 1px solid #eee; padding: 20px; margin: 10px; border-radius: 8px;">
            <h3 style="margin: 0 0 10px 0;">{ &props.title }</h3>
            <p style="margin: 0 0 15px 0; color: #666;">{ &props.description }</p>
            <div style="display: flex; gap: 10px;">
                <a href={props.url.clone()}
                   target="_blank"
                   style="text-decoration: none; color: #0366d6;">
                    { "Demo" }
                </a>
                {
                    if let Some(repo) = &props.repo {
                        html! {
                            <a href={repo.clone()}
                               target="_blank"
                               style="text-decoration: none; color: #0366d6;">
                                { "Repository" }
                            </a>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>
    }
}

create_preview_with_tests!(
    component: ProjectComp,
    default_props: ProjectCompProps {
        title: "YewPreview".to_string(),
        description: "A component explorer and test framework for Yew applications".to_string(),
        url: "https://blog.chriamue.de/yew-preview/".to_string(),
        repo: Some("https://github.com/chriamue/yew-preview".to_string()),
    },
    variants: [
        (
            "Konnektoren",
            ProjectCompProps {
                title: "Konnektoren".to_string(),
                description: "A web application for learning German grammar".to_string(),
                url: "https://konnektoren.github.io/konnektoren-yew/".to_string(),
                repo: Some("https://github.com/konnektoren/konnektoren-yew".to_string()),
            }
        ),
        (
            "Without Repo",
            ProjectCompProps {
                title: "Demo Project".to_string(),
                description: "A project without repository link".to_string(),
                url: "https://example.com".to_string(),
                repo: None,
            }
        ),
        (
            "Long Description",
            ProjectCompProps {
                title: "Complex Project".to_string(),
                description: "This is a much longer description that demonstrates how the component handles more text content. It should wrap appropriately and maintain readable formatting.".to_string(),
                url: "https://example.com".to_string(),
                repo: Some("https://github.com/example/project".to_string()),
            }
        )
    ],
    tests: [
        (
            "Has title",
            exists("h3"),
        ),
        (
            "Has description",
            exists("p"),
        ),
        (
            "Has demo link",
            exists("a"),
        ),
        (
            "Has correct title text",
            has_text("YewPreview"),
        ),
        (
            "Has correct description",
            has_text("A component explorer and test framework for Yew applications"),
        ),
        (
            "Has correct styling",
            has_style("border", "1px solid #eee"),
            has_style("padding", "20px"),
            has_style("border-radius", "8px"),
        ),
    ]
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generate_component_test;
    use crate::test_utils::TestCase;

    generate_component_test!(
        tokio,
        ProjectComp,
        tokio_test_project_comp_rendering,
        ProjectCompProps {
            title: "Test Project".to_string(),
            description: "Test Description".to_string(),
            url: "https://test.com".to_string(),
            repo: Some("https://github.com/test/repo".to_string()),
        },
        vec![
            TestCase {
                name: "Has correct title".to_string(),
                matchers: vec![has_text("Test Project")],
            },
            TestCase {
                name: "Has correct description".to_string(),
                matchers: vec![has_text("Test Description")],
            },
            TestCase {
                name: "Has demo link".to_string(),
                matchers: vec![exists("a")],
            },
        ]
    );

    generate_component_test!(
        tokio,
        ProjectComp,
        tokio_test_project_comp_no_repo,
        ProjectCompProps {
            title: "No Repo Project".to_string(),
            description: "Project without repository".to_string(),
            url: "https://test.com".to_string(),
            repo: None,
        },
        vec![TestCase {
            name: "Has single link".to_string(),
            matchers: vec![Matcher::ElementCount("a".to_string(), 1)],
        },]
    );
}
