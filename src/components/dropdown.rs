use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct DropdownComponent {
    pub children: Children,
    #[prop_or("Click me".to_string())]
    pub title: String,
}

#[function_component]
pub fn Dropdown(drop: &DropdownComponent) -> Html {
    let is_open = use_state(|| false);

    let onclick = {
        let is_open = is_open.clone();
        Callback::from(move |_| is_open.set(!*is_open))
    };

    html! {
        <div class="w-full border border-gray-200 rounded-lg">
            <button {onclick} class="w-full p-4 flex items-center justify-center bg-white hover:bg-gray-50 rounded-lg">
                <span class="font-medium text-gray-700">{&drop.title}</span>
            </button>
            if *is_open {
                <>
                {drop.children.clone()}
                </>
            }
        </div>
    }
}
