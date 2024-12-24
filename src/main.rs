use yew::prelude::*;

mod components;

#[function_component]
fn App() -> Html {
    // let counter = use_state(|| 0);
    // let onclick = {
    //     let counter = counter.clone();
    //     move |_| {
    //         let value = *counter + 1;
    //         counter.set(value);
    //     }
    // };

    html! {
        <>
            <components::header::Header />
            <components::about::About />
            <components::logs::Dec_13::Dec_13 />
        </>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
