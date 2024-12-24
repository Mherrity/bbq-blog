use yew::prelude::*;

#[function_component]
pub fn About() -> Html {
    html! {
        <div style="display: flex; justify-content: center; align-items: start;">
            <div style="text-align: center">
                <h1>{"Hi!"}</h1>
                <h1>{" Welcome to MNIR COOKS!"}</h1>
                <h2>{"This is MNIR speaking!"}</h2>
                <h3>{"I started this with two main objects"}</h3>
                <ul>
                    <li>{"1.) to document my bbq journey with y'all"}</li>
                    <li>{"2.) to write a frontend in rust!"}</li>
                </ul>

            </div>
        </div>
    }
}
