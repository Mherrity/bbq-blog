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
"So for this cook I’m going to get a 11 Lbs prime brisket that should be pretty even. I think the big takeaway from 12-14 was that a fat cap down makes for hard bark. The takeaway from 12-20 is that trimming REALLY matters and I need to be more aggressive to get the cut even. 

I’m gonna try to start fat cap down while I develop the bark and then flip it when wrapped so HOPEFULLY the rendered fat cap bastes it in the wrap. 

Started this cook around 11 at night, woke up at around 3 am to check the temp. It looked like it was stalling around 150 so I wrapped it. Was very dry so I sprayed it as well. Ended up doing the flip here. Had no problem getting the temp up to 170 at 250 then put in for 2 hours at 275 and got up to 185. Bumped to 300 for an hour and got to 195. 

Didn’t have super high temp readings in the flat but the texture seemed insane in the point and it smells great. I ended up letting it rest with the fat cap up because my theory is the juices will still drip down but we’ll see about that theory later.

Juicy as hell again! I think I could try letting it cook a bit longet but I’m happy with the texture. I think that the flavor could have been stronger and I think I can fix that by letting the brisket bark rest more, like 24 hours instead of 4.

We deff got a good smoke ring and some good smokey flavor on the brisket from doing fat cap down and then fat cap up after wrap. I think I’m going to try goldee’s method next and see what results I get. 
";

#[function_component]
pub fn Dec_31() -> Html {
    let brisket = props! { BrisketInfo {
        name: AttrValue::from("Dec 31st Brisket"),
        supplier: BrisketSupplier::WildFork,
        price: 48.60,
        weight: 11.20,
        rub: BrisketRub::LoneStarRub,
        grade: BrisketGrade::Prime,
        image: AttrValue::from("https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/c0173af6-0e63-41f3-277b-8bce6a33d500/public"),
        start_time: AttrValue::from("2024-12-31T00:11:00-05:00"),
    } };
    let key_takeaways = props! { key_takeaways::KeyTakeawaysProps{
        key_takeaways: vec![
            "Flipping seems to get better bark and a better smoke ring but still not the bark we want!".to_string(),
            "Going to try to cold smoke for a while uncovered to see if I can get it much better but I suspect the rub might be a problem as well".to_string(),
            "Again, if I think I'm more aggressive with trimming I can definitely let the point cook longer to like 200 and become a bit more tender.".to_string()


        ]
    }};

    let timeline_props = props! { timeline::TimelineProps {
        events: vec![
            timeline::TimelineEvent {
                date: "11:05 PM".to_string(),
                title: "Started the cook".to_string(),
                description: "Started the cook at 225.".to_string(),
            },
            timeline::TimelineEvent {
                date: "3:51 AM".to_string(),
                title: "Wrapped and bumped to 250".to_string(),
                description: "Wrapped and bumped to 250, then 275 then 300 for the last hour to finish it off.".to_string(),
            },
            timeline::TimelineEvent {
                date: "9:26 AM".to_string(),
                title: "Pulled at 195 internal " .to_string(),
                description: "Probably could have let it gone longer, need to start trimming much more evenly with bigger briskets so I can let them both cook to 200 slowly.".to_string(),
            },
        ]
    }};

            let images = vec![
            carasoul::CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/c0173af6-0e63-41f3-277b-8bce6a33d500/public",
                "Sample image 1",
                "Brisket before being seasoned, untrimmed.",
            ),
            carasoul::CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/2bdf6198-81bf-44bd-49bd-0de6861c1e00/public",
                "Sample image 1",
                "Temperature graph. Was pretty happy with not having a huge stall and consistent temperature climb.",
            ),
            carasoul::CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/9a6ed61e-306a-41bb-8359-d2f2bc821b00/public",
                "Sample image 1",
                "Bend Test! Looking pretty good",
            ),
            carasoul::CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/529ebd00-ea07-43d1-06a0-55b6eff65d00/public",
                "Sample image 1",
                "Need to take better photos of this, but you can see the smoke ring we're getting on a this going fat cap down to fat cap up.",
            ),
            carasoul::CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/c1c678a1-94b4-42d0-e095-ad0c90cf5900/public",
                "Sample image 1",
                "Cross Section! You can see the smoke ring and juiciness I'm talking about.",
            ),
            carasoul::CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/27eb3550-ed2c-4cab-157c-c3ad0d216400/public",
                "Sample image 1",
                "Top view of bark. Better! But not where we want to be.",
            ),
        ];


    html! {
    <>  
        <dropdown::Dropdown title="Photos">
            <carasoul::Carousel images={images}/>
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
