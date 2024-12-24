use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct KeyTakeawaysProps {
    pub key_takeaways: Vec<String>,
}

impl KeyTakeawaysProps {
    pub fn new(key_takeaways: Vec<String>) -> Self {
        Self { key_takeaways }
    }
    pub fn to_list(&self) -> Html {
        html! {
                <>
                    {self.key_takeaways.iter().enumerate().map(|(index, key_takeaway)| {
                        html! {
        <div key={index} class="flex items-start gap-3">
                <div class="mt-1 flex-shrink-0">
                  <svg class="w-5 h-5 text-blue-500" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                    <path d="M20 6L9 17L4 12" strokeLinecap="round" strokeLinejoin="round" />
                  </svg>
                </div>
                <p class="text-blue-900">{key_takeaway}</p>
              </div>
                        }
                    }).collect::<Html>()}
                </>
            }
    }
}

#[function_component]
pub fn KeyTakeaways(props: &KeyTakeawaysProps) -> Html {
    html! {
    <div class="bg-blue-50 rounded-lg p-6 my-8">
        <h3 class="text-xl font-semibold text-blue-900 mb-4">
          {"Key Takeaways"}
        </h3>
        <div class="space-y-3">
          {props.to_list()}
        </div>
      </div>
      }
}
