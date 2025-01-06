use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use gloo::console::log;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
    pub value: Option<String>,
    pub on_input: Option<Callback<String>>,
    pub handle_on_change: Option<Callback<String>>,
}

#[function_component(TextInput)]
pub fn text_input(props: &Props) -> Html {

    if let Some(on_input) = &props.on_input {
        if let Some(value) = &props.value {
            on_input.emit(value.clone());
        }
    }

    let handle_on_change = props.handle_on_change.clone();

    let on_change = Callback::from(move |event: web_sys::Event| {
        let target = event.target().unwrap();
        let input = target.unchecked_into::<HtmlInputElement>();
        let value = input.value();
        log!("Input value: ", value.clone());

        if let Some(handle) = handle_on_change.clone() {
            handle.emit(value.clone());
        }
    });

    html! {
        <input type="text" name={ props.name.clone() } value={ props.value.clone() } onchange={ on_change }
            class="border-sky-400"
         />
    }
}