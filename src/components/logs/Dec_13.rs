use crate::components::brisket_info_table::{
    BrisketGrade, BrisketInfo, BrisketInfoTable, BrisketSupplier, BrisketRub
};
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use chrono_tz::America::New_York;
use yew::prelude::*;
use yew::props;
use crate::components::carasoul;
use crate::components::key_takeaways;
use crate::components::timeline;
use crate::components::dropdown;

const BLOG_TEXT : &str = 
"Started this cook at around 12:20 AM at 225. Checked in on it at 4:30 and then again at 6:30. It felt dry to the touch on the point when I checked on it the second time. Probably should start a little later like 3 am and then check at 6 am instead of 12 when spraying every hour is unrealistic having work at 9 am. 

Sprayed both times I checked on it and wrapped it at 7:16 AM when it looked like the temperature was stalling out at around 2:15. I think part of this might be due to the fact it was around 20 degrees when I started. I’ve never seen the temp on the masterbuilt drop that fast! Cooked wrapped until 1:07 PM at ~195-200 in the point. 

Feeling really good about it but going to let it rest for two hours before looking at the bark. Then I’ll do one hour unwrapping and cutting the first slice. 

The juices are pooling around it so hopefully it is super juice. I feel like if it feels really limp when I bring it out that’s a good sign. 

Cut it at 3:39, roughly 2:30 minutes resting. I realized I cooked in a fat cap up! This definitely makes a difference in terms of the juice, fat cap up is much juicer but I don’t know how to get the bark as good. The bark was wayyy too moist so I don’t know how to get around that. I think fat cap down + tallow on the bark might be the move. I also didn’t get the smoke ring that I usually get when I smoke for this long. 

Passed the tear test and bend test!

I also think that the flat was less juicy than the cap but not dry. I wonder if I cooked the other way around if it would have dried out. 
";

#[function_component]
pub fn Dec_13() -> Html {
    let brisket = props! { BrisketInfo {
        name: AttrValue::from("Dec 13th Brisket"),
        supplier: BrisketSupplier::WildFork,
        price: 57.0,
        weight: 10.46,
        rub: BrisketRub::LoneStarRub,
        grade: BrisketGrade::Prime,
        image: AttrValue::from("https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/16a1d066-d07d-4e4e-8041-7f102f079500/public"),
        start_time: AttrValue::from("2024-12-13T00:00:00-05:00"),
    } };
    let key_takeaways = props! { key_takeaways::KeyTakeawaysProps{
        key_takeaways: vec![
            "Fat cap up is much juicer but the quality of the bark seems to suffer".to_string(),
            "Smoke ring seems to come more heavily from the top and suffers when the fat cap is up".to_string(),
            "Even for 11 lbs, starting earlier than midnight probably makes sense".to_string(),
            "You don't really need to spray until like 4 hours in".to_string(),
        ]
    }};

    let timeline_props = props! { timeline::TimelineProps {
        events: vec![
            timeline::TimelineEvent {
                date: "12:20 AM".to_string(),
                title: "Started the cook".to_string(),
                description: "Started the cook at 225. Checked in on it at 4:30 and then again at 6:30. It felt dry to the touch on the point when I checked on it the second time. Probably should start a little later like 3 am and then check at 6 am instead of 12 when spraying every hour is unrealistic having work at 9 am.".to_string(),
            },
            timeline::TimelineEvent {
                date: "7:16 AM".to_string(),
                title: "Wrapped the brisket".to_string(),
                description: "Sprayed both times I checked on it and wrapped it at 7:16 AM when it looked like the temperature was stalling out at around 150. Bumped the temp from 225 to 275".to_string(),
            },
            timeline::TimelineEvent {
                date: "11:47 AM".to_string(),
                title: "Bumped temp to 300".to_string(),
                description: "After hitting a stall at around 185 I bumped the temp from 275 to 300.".to_string(),
            },
            timeline::TimelineEvent {
                date: "1:07 PM".to_string(),
                title: "Took the brisket off".to_string(),
                description: "Took the brisket off at around 195 in the point.".to_string(),
            },
            timeline::TimelineEvent {
                date: "3:39 PM".to_string(),
                title: "Cut the brisket".to_string(),
                description: "Cut it at 3:39, roughly 2:30 minutes resting. Was super juicy and tender.".to_string(),
            },
        ]
    }};


    html! {
    <>  
        <dropdown::Dropdown title="Photos">
            <carasoul::Carousel />
         </dropdown::Dropdown>

        <dropdown::Dropdown title="Brisket Info">

        <div class="flex justify-center items-center">
            <div class="max-w-3xl">
                <BrisketInfoTable ..brisket />
            </div>
        </div>
        
        </dropdown::Dropdown>

    <dropdown::Dropdown title="Analysis">
        <div class="max-w-3xl mx-auto px-4">
            <div class="bg-white p-8 rounded-lg shadow-md font-serif">
                {BLOG_TEXT.split("\n").map(|line| {
                    if line.trim().is_empty() {
                        html! { <div class="h-5"/> }
                    } else {
                        html! {
                            <p class="mb-4 text-lg leading-relaxed text-gray-700">
                                {line}
                            </p>
                        }
                    }
                }).collect::<Html>()}
            </div>
        </div>
     </dropdown::Dropdown>
     <dropdown::Dropdown title="Timeline">
        <timeline::Timeline ..timeline_props />
    </dropdown::Dropdown>

    <dropdown::Dropdown title="Key Takeaways">
        <key_takeaways::KeyTakeaways ..key_takeaways />
    </dropdown::Dropdown>
        
    </>
        }
}
