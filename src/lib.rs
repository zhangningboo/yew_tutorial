use yew::prelude::*;
use serde::{Deserialize, Serialize};
use gloo::console::log;
use stylist::{style, yew::styled_component};

mod components;
use components::button::Button;

#[derive(Serialize, Deserialize)]
struct MyData {
    name: String,
    age: u32,
}

#[function_component(App)]
pub fn app() -> Html {

    let data = MyData {
        name: "John".to_string(),
        age: 30,
    };

    log!(serde_json::to_string(&data).unwrap());

    let var_class = "red";
    let if_block = true;
    let message = Some("Hello, condition view!");
    let cameras = vec![
        "Canon", "Sony", "Nikon", "Olympus", "Leica", "Fuji", "Panasonic", "Kodak"
    ];
    html! {
        <>
            <h1 class={ var_class }>{ "Hello, world!" }</h1>
            if if_block {
                <p>{ "This is a paragraph." }</p>
            } else {
                <p>{ "This is another paragraph." }</p>
            }

            if let Some(msg) = message {
                <p>{ msg }</p>
            } else {
                <p>{ "No message" }</p>
            }
            <br/>
            <ul style="margin-left: 100px;">
                { cameras.iter().map(|camera| html! { <li>{ camera }</li> }).collect::<Html>() }
            </ul>
            <br/>
            <StyledApp />
            <br/>
            <Button btn_name="Click me +1" />
        </>
    }
}

#[styled_component(StyledApp)]
fn styled_component() -> Html {
    let style  = style!(
        r#"
            color: red;
            background-color: green;
        "#
    ).unwrap();
    html! {
        <h1 class={ style }>{ "Hello, styled_component!" }</h1>
    }
}