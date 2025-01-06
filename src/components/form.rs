use yew::prelude::*;
use gloo::console::log;
use super::text_input::TextInput;
use super::button::Button;

#[function_component(Form)]
pub fn form() -> Html {

    let input_state = use_state(|| "no name set".to_owned());
    let clone_input_state = input_state.clone();
    let input_handle_on_change = Callback::from(move |value: String| {
        log!("parent form component get: ", value.clone());
        // 编辑完之后触发，重新设置值
        clone_input_state.set(value.clone());
    });


    html! {
        <form>
            <TextInput name="name" handle_on_change={ input_handle_on_change }/>
            <Button btn_name="Submit" />
            <p>{ "Input things: " } { &*input_state }</p>
        </form>
    }
}