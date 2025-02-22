use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct SearchBarProps {
    pub on_search: Callback<String>,
    #[prop_or("Search components...".to_string())]
    pub placeholder: String,
}

#[function_component(SearchBar)]
pub fn search_bar(props: &SearchBarProps) -> Html {
    let search_text = use_state(String::new);

    let oninput = {
        let search_text = search_text.clone();
        let on_search = props.on_search.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            search_text.set(value.clone());
            on_search.emit(value);
        })
    };

    html! {
        <div class="yew-preview-search" style="margin: 0 0 20px 0;">
            <input
                class="yew-preview-search-input"
                type="text"
                placeholder={props.placeholder.clone()}
                value={(*search_text).clone()}
                {oninput}
                style="
                    width: 100%;
                    padding: 8px 12px;
                    border: 1px solid #ddd;
                    border-radius: 4px;
                    font-size: 14px;
                    transition: all 0.3s ease;
                    box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
                    &:focus {
                        border-color: #007bff;
                        box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
                        outline: none;
                    }
                "
            />
        </div>
    }
}
