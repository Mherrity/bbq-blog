use core::time;

// brisket_log.rs
use crate::components::{
    brisket_info_table::{
        BrisketGrade, BrisketInfo, BrisketInfoTable, BrisketRub, BrisketSupplier,
    },
    carasoul::{Carousel, CarouselImage},
    dropdown::Dropdown,
    key_takeaways::{KeyTakeaways, KeyTakeawaysProps},
    timeline::{Timeline, TimelineProps},
};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct BrisketLog {
    pub date: AttrValue,
    pub brisket_info: BrisketInfo,
    pub images: Vec<CarouselImage>,
    pub blog_text: AttrValue,
    pub key_takeaways: Vec<String>,
    pub timeline: Option<TimelineProps>,
}

impl BrisketLog {
    pub fn new(
        date: impl Into<AttrValue>,
        brisket_info: BrisketInfo,
        images: Vec<CarouselImage>,
        blog_text: impl Into<AttrValue>,
        key_takeaways: Vec<String>,
        timeline: Option<TimelineProps>,
    ) -> Self {
        Self {
            date: date.into(),
            brisket_info,
            images,
            blog_text: blog_text.into(),
            key_takeaways,
            timeline,
        }
    }
}

#[function_component]
pub fn BrisketLogView(props: &BrisketLog) -> Html {
    let key_takeaways = KeyTakeawaysProps {
        key_takeaways: props.key_takeaways.clone(),
    };

    html! {
        <>
            <Dropdown title="Brisket Info">
                <div class="flex justify-center items-center">
                    <div class="max-w-3xl">
                        <BrisketInfoTable ..props.brisket_info.clone() />
                    </div>
                </div>
            </Dropdown>

            <Dropdown title="Images">
                <Carousel images={props.images.clone()}/>
            </Dropdown>

            <Dropdown title="Cook Details">
                <div class="max-w-3xl mx-auto px-4">
                    <div class="bg-white p-8 rounded-lg shadow-md font-serif">
                        {props.blog_text.split("\n").map(|line| {
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
            </Dropdown>

            <Dropdown title="Key Takeaways">
                <KeyTakeaways ..key_takeaways />
            </Dropdown>
        </>
    }
}

// Example usage in Dec_20.rs:
pub fn create_dec_20_log() -> BrisketLog {
    BrisketLog::new(
        "Dec 20th",
        BrisketInfo {
            name: "Dec 20th Brisket".into(),
            supplier: BrisketSupplier::RestaurantDepot,
            price: 76.0,
            weight: 14.64,
            rub: BrisketRub::LoneStarRub,
            grade: BrisketGrade::Choice,
            image: "https://imagedelivery.net/...".into(),
            start_time: "2024-20-13T00:00:00-05:00".into(),
        },
        vec![
            CarouselImage::new(
                "https://imagedelivery.net/...",
                "Sample image 1",
                "Seasoned brisket from restaurant depot before going on",
            ),
            // ... other images
        ],
        "Started this cook at 12:02 AM at 225...",
        vec![
            "Fat cap down seems to work for better bark, Smoke ring was amazing.".to_string(),
            "Trimming the brisket to be even is super important...".to_string(),
        ],
        None,
    )
}
