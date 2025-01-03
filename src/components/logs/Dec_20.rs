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
"Started this cook at 12:02 AM at 225. Smoking a 14 Lbs choice Angus brisket from restaurant depot. Smoking with the fat cap down to see if I can get a better bark. Cooked at 225 until the stall. Woke up at 5 am, 6 am, 7 am and 8 am to spritz it. The bark coating didn’t seem to be forming which is annoying because I put a huge amount of garlic salt in it specifically so we would get a better bark. Might have to dig into what’s going on more. 

Wrapped around 8 am at a temp of 165. Forgot to put the tallow on earlier so I wasn’t able to coat it in the tallow but I will do that after resting to make it a bit more juicy. I think one of the benefits of restaurant depot is that you can trim the fat yourself to get it where you want it and then you can keep the tallow for other cooks. 

Overall this brisket wasn’t as good as the one last week at all. The point was amazing, super juicy with the smoke ring and bark that I want, but the flat was overcooked for sure. I put the heat up because we were at like 177 with maybe two hours left at some place in the point. I think I made the classic mistake of not trimming it so it’s even so I overcooked the flat and the point was cooked perfectly. Next time I get a huge brisket from restaurant depot I’ll make sure to trim it as much as possible to let it cook evenly. 

Overall I give this brisket a 5/10. The point was amazing but the flat was sooo bad I only served like half of it. 
";


#[function_component]
pub fn Dec_20() -> Html {

        let brisket = props! { BrisketInfo {
        name: AttrValue::from("Dec 20th Brisket"),
        supplier: BrisketSupplier::RestaurantDepot,
        price: 76.0,
        weight: 14.64,
        rub: BrisketRub::LoneStarRub,
        grade: BrisketGrade::Choice, 
        image: AttrValue::from("https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/f5817ce5-a0f5-441d-c3d6-b22a609e1700/public"),
        start_time: AttrValue::from("2024-20-13T00:00:00-05:00"),
    } };

            let images = vec![
            carasoul::CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/f5817ce5-a0f5-441d-c3d6-b22a609e1700/public",
                "Sample image 1",
                "Seasoned brisket from restaurant depot before going on",
            ),
            carasoul::CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/b616ba53-ac09-49aa-627f-7a8f4b115600/public",
                "Sample image 2",
                "Point of Brisket once sliced. You can see the smoke ring was much better than the flat",
            ), 
              carasoul::CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/d1755538-ba41-427c-ae6b-4d76e738a300/public",
                "Sample image 2",
                "Temp log, ambient temp is accurate, the brisket temp is way off so I starred the mesaurements I took. When the green probe is gone the brisket is wrapped",
            )
        ];

        let key_takeaways = props! { key_takeaways::KeyTakeawaysProps{
        key_takeaways: vec![
            "Fat cap down seems to work for better bark, Smoke ring was amazing.".to_string(),
            "Trimming the brisket to be even is super important to get an even cook don't be afraid to cut it down".to_string(),
        ]
    }};

    html!{
<>

     <dropdown::Dropdown title="Brisket Info">
        <div class="flex justify-center items-center">
            <div class="max-w-3xl">
                <BrisketInfoTable ..brisket />
            </div>
        </div>
     </dropdown::Dropdown>

    <dropdown::Dropdown title="Photos">
        <carasoul::Carousel images={images}/>
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


    <dropdown::Dropdown title="Key Takeaways">
        <key_takeaways::KeyTakeaways ..key_takeaways />
    </dropdown::Dropdown>


</>
    }

}



