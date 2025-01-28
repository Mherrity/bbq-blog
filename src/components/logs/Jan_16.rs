use crate::components::brisket_info_table::{
    BrisketGrade, BrisketInfo, BrisketInfoTable, BrisketSupplier, BrisketRub
};
use chrono::{DateTime, Local, NaiveDateTime, Utc};
use chrono_tz::America::New_York;
use yew::prelude::*;
use yew::props;
use crate::components::multimedia_carasoul;
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
pub fn Jan_16th() -> Html {
    let brisket = props! { BrisketInfo {
        name: AttrValue::from("Jan 16th Brisket"),
        supplier: BrisketSupplier::WildFork,
        price: 77.01,
        weight: 14.46,
        rub: BrisketRub::LoneStarRub,
        grade: BrisketGrade::Prime,
        image: AttrValue::from("https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/6bfddaa2-6e26-4454-c711-5c98193f6d00/public"),
        start_time: AttrValue::from("2024-01-16-T00:22:00-05:00"),
    } };
    let key_takeaways = props! { key_takeaways::KeyTakeawaysProps{
        key_takeaways: vec![
           "If you let moisture build up / pool that's going to make certain areas cook slower".to_string(),
            "Need to be more aggressive with trimming, esp in terms of making it an even but which I did not think was going to be the case.".to_string(),
            "Wrapping at 185 does produce very solid bark".to_string(),
            "There is such a thing as overcooking! We should stay with pulling at 203.".to_string(),
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

            let items = vec![
            multimedia_carasoul::CarouselItem::new_video(
                "https://videos.mnir.food/IMG_8546.mov",
                "Sample image 1",
                "Trimming the brisket, deff should have focused on evening this out more.",
            ),
            multimedia_carasoul::CarouselItem::new_image(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/0328e73d-c408-4efb-5fb3-a795c21a0d00/public",
                "Sample image 2",
                "The brisket before going on",
            ), 
            multimedia_carasoul::CarouselItem::new_image(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/293ea9d8-d9f6-4d53-e54a-e6ff26b3bf00/public",
                "Sample image 2",
                "The brisket after being pulled, you can see the moisture spots I was talking about that I think made it cook unevenly.",
            ),  // 5fe39c375003448d808c3968e19a1d21.mov
                multimedia_carasoul::CarouselItem::new_video(
                "https://videos.mnir.food/5fe39c375003448d808c3968e19a1d21.mov",
                "Sample image 1",
                "Brisket right before I wrapped, you can also see how odd the sahpe is. Something I should try is making it much more round and even.",
            ),
                multimedia_carasoul::CarouselItem::new_video(
                "https://videos.mnir.food/IMG_8578.mov",
                "Sample image 1",
                "Cross section of the brisket. Was juicy enough and I love the bark, I think the drier spots from tasting were from the uneven cooking.",
            ),


        ];


    html! {
    <>  
        <dropdown::Dropdown title="Photos">
            <multimedia_carasoul::MultiMediaCarousel items={items}/>
         </dropdown::Dropdown>

        <dropdown::Dropdown title="Brisket Info">

        <div class="flex justify-center items-center">
            <div class="max-w-3xl">
                <BrisketInfoTable ..brisket />
            </div>
        </div>
        
        </dropdown::Dropdown>

    <dropdown::Dropdown title="Analysis">

    <div class="max-w-2xl mx-auto bg-white rounded-lg shadow-md p-6">
        
        <div class="space-y-6 text-gray-700">
            <div>
                <p><span class="font-bold">{"10 pm:"}</span> {"So we're starting to cook tonight at around 10 pm and I am re-attempting the cold smoke method. I saw a youtube video from smoking dad barbecue that suggested you can get great results with just charcoal. Also! My barbecue discord (from madscientestbbq) suggested I wrap at around 185 to get better bark so we're going to try that for the first time today."}</p>
            </div>

            <div>
                <p><span class="font-bold">{"11 pm:"}</span> {"So this definitely is still cooking at like 200 for some reason, I feel like if I really tried to choke it I could get it down and maybe if I have more charcoal in the hopper so there's not as much room for fire it could work but we'll see next week I guess!"}</p>
            </div>

            <div>
                <p><span class="font-bold">{"5 am:"}</span> {"For the first time since I've been doing this I fell asleep! When I went outside the ambient temp is like 80 and looks like we lost 10-20 degrees internal, going to try to bump it up to 275 and run with it from there."}</p>
            </div>

            <div>
                <p><span class="font-bold">{"10 am:"}</span> {"Been going pretty well at 275 using mostly wood split. Currently I'm registering a 10 degree gap between the point and flat (172 vs. 183 so I'm going to try to baffle the flat with my water tray to even it out."}</p>
            </div>

            <div>
                <p><span class="font-bold">{"11 am:"}</span> {"Wrapping at 185 in a foil boat, the baffle worked too well so I removed it and and now cranking it up to 300 and waiting until we get around 205 to start probe testing."}</p>
            </div>

            <div>
                <p><span class="font-bold">{"1:10-1:20 pm:"}</span> {"Pulled at around 210 - 209, the flat baffle ended up too well so it needed to catch up. Based on probe feel I think I did pretty well! The bark looks good at well I just really hope that it stays on. There is still a bit in the flat that seems harder than the rest and I don't know why that is. I might have to go online and find out explanations. One theory I have is that if moisture is pooling than maybe that leads to it cooling certain parts of it. Anyways wrapping and resting until 4."}</p>
            </div>

            <div>
                <p><span class="font-bold">{"6 pm:"}</span> {"After tasting this I think that I deff left it on too long but the bark and flavor it on point!! Next time I don't think I'll play around with it too much, wait to 185 to wrap, but pull at 203."}</p>
            </div>
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
