use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct HeaderCompProps {
    pub title: String,
}

#[function_component(HeaderComp)]
pub fn header_comp(props: &HeaderCompProps) -> Html {
    html! {
        <header style="border-bottom: 1px solid #ccc; padding: 10px;">
            <h1>{ &props.title }</h1>
        </header>
    }
}
