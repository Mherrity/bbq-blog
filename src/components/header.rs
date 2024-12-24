use yew::prelude::*;

#[function_component]
pub fn Header() -> Html {
    html! {
        <div class="flex items-center justify-center w-full bg-white px-8 py-4 gap-4">
                <a href="#" class="hover:text-gray-600 self-end text-2xl">{ "About" }</a>

            <div class="text-5xl font-bold text-gray-900">
                { "MNIR COOKS!" }
            </div>
                 <a href="#" class="hover:text-gray-600 self-end text-2xl">{ "Log" }</a>
        </div>
    }
}
