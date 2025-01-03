use yew::prelude::*;

mod components;

#[function_component]
fn App() -> Html {
    html! {
        <>
            <components::header::Header />
            <components::about::About />
        <h1 style="text-align: center; font-size: 2.5em;">{"Logs"}</h1>
            <components::dropdown::Dropdown title="Dec 13th [Brisket]">
                <components::logs::Dec_13::Dec_13 />
            </components::dropdown::Dropdown>
            <components::dropdown::Dropdown title="Dec 20th [Brisket]">
                <components::logs::Dec_20::Dec_20/>
            </components::dropdown::Dropdown>
            <components::dropdown::Dropdown title="Dec 31st [Brisket]">
                <components::logs::Dec_31::Dec_31/>
            </components::dropdown::Dropdown>

        </>

    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
