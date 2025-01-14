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
"This cook I wanted to try the Bar A, cold smoke method where you smoke at like 150 for 5-6 hours and then cook it higher so you can pull sooner. Because I don’t have an offset I used these Jealous Devil blocks and firewood to try to get more smoky flavor. 

At the beginning I was able to manually maintain the fire at like 150 to 170 but as I went to sleep I put more wood in and the masterbuilt wasn’t able to keep the fire below 250 so we were hitting 160 at like 1:30 AM after putting stuff on at 10 PM. After that I was a lot more careful with adding firewood and found that 1 log was able to last me at 225 til 6 AM. 

I found the consistency and bark was what I wanted around 6 AM so I decided to turn it up to 275 until we hit 200 internal and I pulled it. 

This thing really was sooo good, the bark was what I wanted, the consistency was the best that I’ve had yet. I think that letting it cooking longer and keeping the top cover off really helped the bark develop. My takeaway is that I was wrapping too early. 
";

#[function_component]
pub fn Jan_3rd() -> Html {
    let brisket = props! { BrisketInfo {
        name: AttrValue::from("Jan 3rd Brisket"),
        supplier: BrisketSupplier::WildFork,
        price: 48.60,
        weight: 9.76,
        rub: BrisketRub::LoneStarRub,
        grade: BrisketGrade::Prime,
        image: AttrValue::from("https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/b075eee8-4a96-4a47-5718-3fb3a07fe000/public"),
        start_time: AttrValue::from("2024-01-02-T00:22:00-05:00"),
    } };
    let key_takeaways = props! { key_takeaways::KeyTakeawaysProps{
        key_takeaways: vec![
            "Even if the smoke is bad smoke, real smoke better than no smoke".to_string(),
            "You can let stuff sit for a while on the smoker unwrapped without it drying out".to_string(),
            "The water pan makes a huge difference in the bark and consistency of the cook.".to_string(),
            "Fat cap up is the move and you can develop really good bark on the fat side".to_string(),
            "Still need to trim better, and let it cook longer, going to try to wait til I get probe tender in the flat".to_string(),
            "Foil boat seems to work as well? Unclear weather the tenderness was from the foil boat or the cook.".to_string(),
        ]
    }};

    // let timeline_props = props! { timeline::TimelineProps {
    //     events: vec![
    //         timeline::TimelineEvent {
    //             date: "11:05 PM".to_string(),
    //             title: "Started the cook".to_string(),
    //             description: "Started the cook at 225.".to_string(),
    //         },
    //         timeline::TimelineEvent {
    //             date: "3:51 AM".to_string(),
    //             title: "Wrapped and bumped to 250".to_string(),
    //             description: "Wrapped and bumped to 250, then 275 then 300 for the last hour to finish it off.".to_string(),
    //         },
    //         timeline::TimelineEvent {
    //             date: "9:26 AM".to_string(),
    //             title: "Pulled at 195 internal " .to_string(),
    //             description: "Probably could have let it gone longer, need to start trimming much more evenly with bigger briskets so I can let them both cook to 200 slowly.".to_string(),
    //         },
    //     ]
    // }};

            let images = vec![
            carasoul::CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/b075eee8-4a96-4a47-5718-3fb3a07fe000/public",
                "Sample image 1",
                "Top level when I pulled the brisket, this color bark was a first!!",
            ),
            carasoul::CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/5f9bc849-0ffe-4568-bc1e-37b6a0333b00/public",
                "Sample image 1",
                "Nice cross section showing the bark we got and how juicy it was.",
            ),
            carasoul::CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/50531014-93e4-44fa-0f62-3982e55f9400/public",
                "Sample image 1",
                "Brisket during the stall, you can see how if I wrapped here it would be nowhere close to where I want it to be.",
            ),
            carasoul::CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/096f74e9-0cd2-4244-e63e-5d6d866f9800/public",
                "Sample image 1",
                "Tempeature graph, you can see how bad I am at controlling the temperature early.",
            ),
            // carasoul::CarouselImage::new(
            //     "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/9a6ed61e-306a-41bb-8359-d2f2bc821b00/public",
            //     "Sample image 1",
            //     "Bend Test! Looking pretty good",
            // ),
            // carasoul::CarouselImage::new(
            //     "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/529ebd00-ea07-43d1-06a0-55b6eff65d00/public",
            //     "Sample image 1",
            //     "Need to take better photos of this, but you can see the smoke ring we're getting on a this going fat cap down to fat cap up.",
            // ),
            // carasoul::CarouselImage::new(
            //     "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/c1c678a1-94b4-42d0-e095-ad0c90cf5900/public",
            //     "Sample image 1",
            //     "Cross Section! You can see the smoke ring and juiciness I'm talking about.",
            // ),
            // carasoul::CarouselImage::new(
            //     "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/27eb3550-ed2c-4cab-157c-c3ad0d216400/public",
            //     "Sample image 1",
            //     "Top view of bark. Better! But not where we want to be.",
            // ),
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
    //  <dropdown::Dropdown title="Timeline">
    //     <timeline::Timeline ..timeline_props />
    // </dropdown::Dropdown>

    <dropdown::Dropdown title="Key Takeaways">
        <key_takeaways::KeyTakeaways ..key_takeaways />
    </dropdown::Dropdown>
        
    </>
        }
}
