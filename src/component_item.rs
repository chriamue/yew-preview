use yew::prelude::Html;

#[derive(Clone, PartialEq)]
pub struct ComponentItem {
    pub name: String,
    pub render: Vec<(String, Html)>,
}
