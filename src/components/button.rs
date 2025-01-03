use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub btn_name: String,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    html! {
        <>
            <div>
                <button { onclick } class="inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-colors focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0 bg-primary text-primary-foreground shadow hover:bg-primary/90 h-9 px-4 py-2"> 
                    { &props.btn_name }
                </button>
                <p> { *counter } </p>
            </div>
        </>
    }
}