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
"
Last week I was in texas for three days and one of my biggest takeaways from visiting some of the best in the country is that I am definitely pulling way too early and was too afraid to bump the temperature. 

Most of the best places I went to did 250 and bumped to 300 after wrapping until they felt good. I didn’t understand this at first but I realized that a lot of what I used to think was the meat drying out in places I didn’t like what actually the meat not fully breaking down so it was tough and when its tough the juices can’t get in. 

So the idea here is to basically replicate what I did last time with the wood and foil boat for good bark but then pull much later at like 203 and see how it turns out. Because I completely failed at the cold smoke last time I’m not even gonna try that and just rock with 220-250 til its time to boat and bump it up to 250. 

Honesty after pulling this thing this is the best brisket I have ever made. Super super tender, best bark I’ve gotten yet. I still don’t have the bark because when I handle it its prone to fall off. I also think that I wasn’t aggressive enough with the trimming in some areas. Anyways I’ll try to figure out how to get the bark better. 
";

#[function_component]
pub fn Jan_10th() -> Html {
    let brisket = props! { BrisketInfo {
        name: AttrValue::from("Jan 10th Brisket"),
        supplier: BrisketSupplier::WildFork,
        price: 57.82,
        weight: 11.61,
        rub: BrisketRub::LoneStarRub,
        grade: BrisketGrade::Prime,
        image: AttrValue::from("https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/8d07d1bc-676c-40c5-8709-9b15883eb600/public"),
        start_time: AttrValue::from("2024-01-10-T00:22:00-05:00"),
    } };
    let key_takeaways = props! { key_takeaways::KeyTakeawaysProps{
        key_takeaways: vec![
           "Taking it off at 203 or makes a huge difference in tenderness and juiciness.".to_string(),
            "Need to be more aggressive with trimming, especially in the flat which I didn't think was going to be the case.".to_string(),
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
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/abdfad80-c684-4684-68ac-7a4ccde9e600/public",
                "Sample image 1",
                "Bend test passed 😎 s/o @ovpleasefocus for the camera work",
            ),
            carasoul::CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/4227e044-fee4-45b3-b347-7b4c035bc300/public",
                "Sample image 1",
                "Cross section of the brisket, you can see how it's really breaking down. Those cracks in the meat are a good sign. You can also see the top bark that is falling off.",
            ),
            carasoul::CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/1a791449-e56c-4023-c082-66e9fa459b00/public",
                "Sample image 1",
                "Temp graph, you can see how I have no control over this thing lol",
            ),
            carasoul::CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/8d07d1bc-676c-40c5-8709-9b15883eb600/public",
                "Sample image 1",
                "When the brisket was pulled, exactly the color bark that I'm shooting for.",
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
