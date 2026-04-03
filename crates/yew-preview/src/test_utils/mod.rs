pub mod helpers;
pub mod matchers;
pub mod test_case;

pub use helpers::*;
pub use matchers::Matcher;
pub use test_case::{MatcherResult, TestCase, TestCaseResult};

#[cfg(feature = "testing")]
use yew::{BaseComponent, Properties};

pub trait TestCases {
    fn get_test_cases(&self) -> Vec<TestCase>;
}

#[cfg(feature = "testing")]
pub async fn render_component<C>(props: C::Properties) -> String
where
    C: BaseComponent,
    C::Properties: Properties + PartialEq + Clone,
{
    let renderer = yew::LocalServerRenderer::<C>::with_props(props);
    renderer.render().await
}
