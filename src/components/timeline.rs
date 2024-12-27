use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct TimelineEvent {
    pub date: String,
    pub title: String,
    pub description: String,
}
#[derive(Properties, Clone, PartialEq)]
pub struct TimelineProps {
    pub events: Vec<TimelineEvent>,
}

#[function_component]
pub fn Timeline(props: &TimelineProps) -> Html {
    html! {
        <div class="relative py-8">
            <div class="absolute left-4 top-0 bottom-0 w-0.5 bg-gray-200"></div>
            <div class="space-y-8">
                {props.events.iter().map(|event| {
                    html! {
                        <div class="relative flex items-start gap-6 ml-4">
                            <div class="absolute -left-4 mt-1.5">
                                <div class="h-4 w-4 rounded-full bg-blue-500 border-4 border-white ring-2 ring-blue-500"></div>
                            </div>
                            <div class="flex-1 bg-white rounded-lg border border-gray-200 p-4 shadow-sm">
                                <div class="flex items center gap-2 text-sm text-gray-600 mb-2">
                                    <svg class="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                        <circle cx="12" cy="12" r="10"></circle>
                                        <path d="M12 6v6l4 2"></path>
                                    </svg>
                                    <span>{&event.date}</span>
                                </div>
                                <h3 class="font-medium text-lg text-gray-900 mb-2">
                                    {&event.title}
                                </h3>
                                <p class="text-gray-600">
                                    {&event.description}
                                </p>
                            </div>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}

// Timeline = ({ events }) => {
//   return (
//     <div className="relative py-8">
//       {/* Vertical line */}
//       <div className="absolute left-4 top-0 bottom-0 w-0.5 bg-gray-200"></div>

//       <div className="space-y-8">
//         {events.map((event, index) => (
//           <div key={index} className="relative flex items-start gap-6 ml-4">
//             {/* Timeline dot */}
//             <div className="absolute -left-4 mt-1.5">
//               <div className="h-4 w-4 rounded-full bg-blue-500 border-4 border-white ring-2 ring-blue-500"></div>
//             </div>

//             {/* Content */}
//             <div className="flex-1 bg-white rounded-lg border border-gray-200 p-4 shadow-sm">
//               <div className="flex items-center gap-2 text-sm text-gray-600 mb-2">
//                 {/* Clock SVG */}
//                 <svg className="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
//                   <circle cx="12" cy="12" r="10" />
//                   <path d="M12 6v6l4 2" />
//                 </svg>
//                 <span>{event.date}</span>
//               </div>
//               <h3 className="font-medium text-lg text-gray-900 mb-2">
//                 {event.title}
//               </h3>
//               <p className="text-gray-600">
//                 {event.description}
//               </p>
//             </div>
//           </div>
//         ))}
//       </div>
//     </div>
//   );
// };
