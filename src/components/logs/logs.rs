// // brisket_logs_data.rs
// use crate::components::{
//     brisket_info_table::{BrisketGrade, BrisketInfo, BrisketRub, BrisketSupplier},
//     brisket_log::BrisketLog,
//     carasoul::CarouselImage,
//     timeline::{TimelineEvent, TimelineProps},
// };
// use lazy_static::lazy_static;
// use yew::AttrValue;

// pub const Dec_13_Brisket : BrisketInfo =  BrisketInfo {
//             name: "Dec 20th Brisket".into(),
//             supplier: BrisketSupplier::RestaurantDepot,
//             price: 76.0,
//             weight: 14.64,
//             rub: BrisketRub::LoneStarRub,
//             grade: BrisketGrade::Choice,
//             image: "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/f5817ce5-a0f5-441d-c3d6-b22a609e1700/public".into(),
//             start_time: "2024-20-13T00:00:00-05:00".into(),
//         };

// lazy_static! {
//     pub static ref  BRISKET_LOGS: [BrisketLog; 1] = [
//     // Dec 20th Log
//     BrisketLog::new(
//         "Dec 20th",
//         BrisketInfo {
//             name: "Dec 20th Brisket".into(),
//             supplier: BrisketSupplier::RestaurantDepot,
//             price: 76.0,
//             weight: 14.64,
//             rub: BrisketRub::LoneStarRub,
//             grade: BrisketGrade::Choice,
//             image: "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/f5817ce5-a0f5-441d-c3d6-b22a609e1700/public".into(),
//             start_time: "2024-20-13T00:00:00-05:00".into(),
//         },
//         vec![
//             CarouselImage::new(
//                 "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/f5817ce5-a0f5-441d-c3d6-b22a609e1700/public",
//                 "Sample image 1",
//                 "Seasoned brisket from restaurant depot before going on",
//             ),
//             CarouselImage::new(
//                 "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/b616ba53-ac09-49aa-627f-7a8f4b115600/public",
//                 "Sample image 2",
//                 "Point of Brisket once sliced. You can see the smoke ring was much better than the flat",
//             ),
//             CarouselImage::new(
//                 "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/d1755538-ba41-427c-ae6b-4d76e738a300/public",
//                 "Sample image 2",
//                 "Temp log, ambient temp is accurate, the brisket temp is way off so I starred the mesaurements I took. When the green probe is gone the brisket is wrapped",
//             ),
//         ],
//         "Started this cook at 12:02 AM at 225. Smoking a 14 Lbs choice Angus brisket from restaurant depot. Smoking with the fat cap down to see if I can get a better bark. Cooked at 225 until the stall. Woke up at 5 am, 6 am, 7 am and 8 am to spritz it. The bark coating didn't seem to be forming which is annoying because I put a huge amount of garlic salt in it specifically so we would get a better bark. Might have to dig into what's going on more.

// Wrapped around 8 am at a temp of 165. Forgot to put the tallow on earlier so I wasn't able to coat it in the tallow but I will do that after resting to make it a bit more juicy. I think one of the benefits of restaurant depot is that you can trim the fat yourself to get it where you want it and then you can keep the tallow for other cooks.

// Overall this brisket wasn't as good as the one last week at all. The point was amazing, super juicy with the smoke ring and bark that I want, but the flat was overcooked for sure. I put the heat up because we were at like 177 with maybe two hours left at some place in the point. I think I made the classic mistake of not trimming it so it's even so I overcooked the flat and the point was cooked perfectly. Next time I get a huge brisket from restaurant depot I'll make sure to trim it as much as possible to let it cook evenly.

// Overall I give this brisket a 5/10. The point was amazing but the flat was sooo bad I only served like half of it.",
//         vec![
//             "Fat cap down seems to work for better bark, Smoke ring was amazing.".to_string(),
//             "Trimming the brisket to be even is super important to get an even cook don't be afraid to cut it down".to_string(),
//         ],
//         None,
//     ),

// //     // Dec 13th Log
// //     BrisketLog::new(
// //         "Dec 13th",
// //         BrisketInfo {
// //             name: AttrValue::from("Dec 13th Brisket"),
// //             supplier: BrisketSupplier::WildFork,
// //             price: 57.0,
// //             weight: 10.46,
// //             rub: BrisketRub::LoneStarRub,
// //             grade: BrisketGrade::Prime,
// //             image: AttrValue::from("https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/16a1d066-d07d-4e4e-8041-7f102f079500/public"),
// //             start_time: AttrValue::from("2024-12-13T00:00:00-05:00"),
// //         },
// //         vec![
// //             CarouselImage::new(
// //                 "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/943040ab-bc0e-43a9-60e1-73ff6c768500/public",
// //                 "Sample image 1",
// //                 "Brisket after resting, definitely should have you trimmed better to get bark on that flank. Also wasn't really able to get good bark when cooking fat cap up.",
// //             ),
// //             CarouselImage::new(
// //                 "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/8a96422f-f724-467e-a95c-bfaa0cc30100/public",
// //                 "Sample image 2",
// //                 "Passing the bend test with a thick slice! Was super super juicy and flavorful",
// //             ),
// //             CarouselImage::new(
// //                 "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/98aa4f11-0fe3-4475-64b6-6cd3f077e500/public",
// //                 "Sample image 3",
// //                 "When cut in half, as you can see, super super juicy but the bark was not as good as I wanted it to be and I didn't really get a good smoke ring",
// //             ),
// //             CarouselImage::new(
// //                 "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/d7b11955-4d4a-4b61-a9ab-20c4adbcf000/public",
// //                 "Sample image 3",
// //                 "Graph of smoker temperature and meat temperature in the point. Really like the consistient curve and think I pulled at the right time.",
// //             ),
// //             CarouselImage::new(
// //                 "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/16a1d066-d07d-4e4e-8041-7f102f079500/public",
// //                 "Sample image 3",
// //                 "Front of wildfork brisket pre-seasoning",
// //             ),
// //             CarouselImage::new(
// //                 "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/09accf6c-7167-472f-f836-2d8d54e1a400/public",
// //                 "Sample image 3",
// //                 "Back of wildfork brisket pre-seasoning",
// //             ),
// //         ],
// //         "Started this cook at around 12:20 AM at 225. Checked in on it at 4:30 and then again at 6:30. It felt dry to the touch on the point when I checked on it the second time. Probably should start a little later like 3 am and then check at 6 am instead of 12 when spraying every hour is unrealistic having work at 9 am.

// // Sprayed both times I checked on it and wrapped it at 7:16 AM when it looked like the temperature was stalling out at around 2:15. I think part of this might be due to the fact it was around 20 degrees when I started. I've never seen the temp on the masterbuilt drop that fast! Cooked wrapped until 1:07 PM at ~195-200 in the point.

// // Feeling really good about it but going to let it rest for two hours before looking at the bark. Then I'll do one hour unwrapping and cutting the first slice.

// // The juices are pooling around it so hopefully it is super juice. I feel like if it feels really limp when I bring it out that's a good sign.

// // Cut it at 3:39, roughly 2:30 minutes resting. I realized I cooked in a fat cap up! This definitely makes a difference in terms of the juice, fat cap up is much juicer but I don't know how to get the bark as good. The bark was wayyy too moist so I don't know how to get around that. I think fat cap down + tallow on the bark might be the move. I also didn't get the smoke ring that I usually get when I smoke for this long.

// // Passed the tear test and bend test!

// // I also think that the flat was less juicy than the cap but not dry. I wonder if I cooked the other way around if it would have dried out.",
// //         vec![
// //             "Fat cap up is much juicer but the quality of the bark seems to suffer".to_string(),
// //             "Smoke ring seems to come more heavily from the top and suffers when the fat cap is up".to_string(),
// //             "Even for 11 lbs, starting earlier than midnight probably makes sense".to_string(),
// //             "You don't really need to spray until like 4 hours in".to_string(),
// //         ],
// //         Some(TimelineProps {
// //             events: vec![
// //                 TimelineEvent {
// //                     date: "12:20 AM".to_string(),
// //                     title: "Started the cook".to_string(),
// //                     description: "Started the cook at 225. Checked in on it at 4:30 and then again at 6:30. It felt dry to the touch on the point when I checked on it the second time.".to_string(),
// //                 },
// //                 TimelineEvent {
// //                     date: "7:16 AM".to_string(),
// //                     title: "Wrapped the brisket".to_string(),
// //                     description: "Sprayed both times I checked on it and wrapped it at 7:16 AM when it looked like the temperature was stalling out at around 150. Bumped the temp from 225 to 275".to_string(),
// //                 },
// //                 TimelineEvent {
// //                     date: "11:47 AM".to_string(),
// //                     title: "Bumped temp to 300".to_string(),
// //                     description: "After hitting a stall at around 185 I bumped the temp from 275 to 300.".to_string(),
// //                 },
// //                 TimelineEvent {
// //                     date: "1:07 PM".to_string(),
// //                     title: "Took the brisket off".to_string(),
// //                     description: "Took the brisket off at around 195 in the point.".to_string(),
// //                 },
// //                 TimelineEvent {
// //                     date: "3:39 PM".to_string(),
// //                     title: "Cut the brisket".to_string(),
// //                     description: "Cut it at 3:39, roughly 2:30 minutes resting. Was super juicy and tender.".to_string(),
// //                 },
// //             ],
// //         }),
// //     ),
// ];
// }
