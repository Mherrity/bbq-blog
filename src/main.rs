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
            <components::dropdown::Dropdown title="Dec 13th [Brisket]">
                <components::logs::Dec_13::Dec_13 />
            </components::dropdown::Dropdown>
            <components::dropdown::Dropdown title="Dec 30th [Brisket]">
                <components::logs::Dec_30::Dec_30/>
            </components::dropdown::Dropdown>
        </>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
