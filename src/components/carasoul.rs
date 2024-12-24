use std::cell::Cell;
use std::ops::Add;
use std::ops::Sub;
use std::rc::Rc;
use web_sys::console;
use yew::prelude::*;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CarouselImage {
    src: AttrValue,
    alt: AttrValue,
    caption: AttrValue,
}
impl CarouselImage {
    fn new(
        src: impl Into<AttrValue>,
        alt: impl Into<AttrValue>,
        caption: impl Into<AttrValue>,
    ) -> Self {
        Self {
            src: src.into(),
            alt: alt.into(),
            caption: caption.into(),
        }
    }
}

pub struct Carousel {
    current_index: Rc<Cell<usize>>,
    is_modal_open: Rc<Cell<bool>>,
    images: Vec<CarouselImage>,
}

impl Carousel {
    fn map_images(&self, ctx: &Context<Self>) -> Vec<Html> {
        let handle_thumbnail_click = |index: usize| {
            let link = ctx.link().clone();
            move |_: MouseEvent| {
                link.send_message(Msg::MoveIndex(index));
            }
        };

        self.images
            .iter()
            .enumerate()
            .map(|(index, image)| {
                html! {
                  <button
                    key={index}
                    onclick={handle_thumbnail_click(index)}
                    class={
                        format!{"w-16 h-16 rounded-lg overflow-hidden transition-all {}",
                    if index == self.current_index.get() {"ring-2 ring-blue-500"} else { "opacity-50 hover:opacity-75" }
                } }
                  >
                    <img
                      src={image.src.clone()}
                      alt={format!("Thumbnail {:?}", index + 1) }
                      class="w-full h-full object-cover"
                    />
                  </button>
                    }
            })
            .collect::<Vec<_>>()
    }

    fn modal(&self, ctx: &Context<Self>) -> Html {
        let current_image = &self.images[self.current_index.get()].clone();

        let switch_modal = {
            let link = ctx.link().clone();
            let is_modal_open = self.is_modal_open.clone();
            move |_| {
                is_modal_open.set(!is_modal_open.get());
                link.send_message(Msg::SwitchModal);
            }
        };

        let close_modal = {
            let is_modal_open = self.is_modal_open.clone();
            let link = ctx.link().clone();
            move |_| {
                is_modal_open.set(false);
                link.send_message(Msg::SwitchModal);
            }
        };

        html! {
            <div
              class="fixed inset-0 bg-black/90 z-50 flex items-center justify-center"
              onclick={close_modal}
            >
              <div class="max-w-7xl max-h-[90vh] relative">
                <img
                  src={current_image.src.clone()}
                  alt={current_image.alt.clone()}
                  class="max-w-full max-h-[90vh] object-contain"
                />
                <button
                  onclick = {switch_modal.clone()}
                  class="absolute top-4 right-4 p-2 bg-black/50 rounded-full text-white hover:bg-black/70"
                >
                  <svg class="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                    <path d="M18 6L6 18M6 6l12 12" strokeLinecap="round" strokeLinejoin="round"/>
                  </svg>
                </button>
              </div>
            </div>
        }
    }
}

pub enum Msg {
    MoveIndex(usize),
    SwitchModal,
}

impl Component for Carousel {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let current_index = Rc::new(Cell::new(0));
        let is_modal_open = Rc::new(Cell::new(false));
        let images = vec![
            CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/943040ab-bc0e-43a9-60e1-73ff6c768500/public",
                "Sample image 1",
                "Brisket after resting, definitely should have you trimmed better to get bark on that flank. Also wasn't really able to get good bark when cooking fat cap up.",
            ),
            CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/8a96422f-f724-467e-a95c-bfaa0cc30100/public",
                "Sample image 2",
                "Passing the bend test with a thick slice! Was super super juicy and flavorful",
            ),
            CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/98aa4f11-0fe3-4475-64b6-6cd3f077e500/public",
                "Sample image 3",
                "When cut in half, as you can see, super super juicy but the bark was not as good as I wanted it to be and I didn't really get a good smoke ring",
            ),
            CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/d7b11955-4d4a-4b61-a9ab-20c4adbcf000/public",
                "Sample image 3",
                "Graph of smoker temperature and meat temperature in the point. Really like the consistient curve and think I pulled at the right time. I think that bumping anytime you hit a slight stall and then pulling at 195-200 based on the probe tenderness works well for me.",
            ),
            CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/16a1d066-d07d-4e4e-8041-7f102f079500/public",
                "Sample image 3",
                "Front of wildfork brisket pre-seasoning",
            ),
            CarouselImage::new(
                "https://imagedelivery.net/cY9nr3ozwYWtSYKWdfK7tg/09accf6c-7167-472f-f836-2d8d54e1a400/public",
                "Sample image 3",
                "Back of wildfork brisket pre-seasoning",
            ),
        ];

        Self {
            current_index,
            is_modal_open,
            images,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MoveIndex(index) => {
                self.current_index.set(index);
                true // Return true to trigger re-render
            }
            Msg::SwitchModal => {
                self.is_modal_open.set(!self.is_modal_open.get());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let current_image = &self.images[self.current_index.get()].clone();

        let move_index_up = |index: usize| {
            let link = ctx.link().clone();
            let len = self.images.len();
            move |_: MouseEvent| {
                if index == len.sub(1) {
                    link.send_message(Msg::MoveIndex(0));
                } else {
                    link.send_message(Msg::MoveIndex(index.add(1)));
                }
            }
        };

        let move_index_down = |index: usize| {
            let link = ctx.link().clone();
            let len = self.images.len();
            move |_: MouseEvent| {
                if index == 0 {
                    link.send_message(Msg::MoveIndex(len.sub(1)));
                } else {
                    link.send_message(Msg::MoveIndex(index.sub(1)));
                }
            }
        };
        html! {
                    <div class="max-w-4xl mx-auto relative">

              <div class="relative aspect-[4/3] bg-gray-900 rounded-lg overflow-hidden">
                <img
                  src={current_image.src.clone()}
                  alt={current_image.alt.clone()}
                  class="w-full h-full object-contain cursor-pointer"
                />

                <div class="absolute bottom-0 left-0 right-0 p-4 bg-gradient-to-t from-black/90 to-black/50">
                  <p class="text-white text-center">
                        {current_image.caption.clone()} // Added the caption here
                  </p>
                </div>

                <button
                  class="absolute top-4 right-4 p-2 bg-black/50 rounded-full text-white hover:bg-black/70 transition-colors"
                >
                  <svg class="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                    <path d="M15 3h6v6M9 21H3v-6M21 3l-7 7M3 21l7-7" strokeLinecap="round" strokeLinejoin="round"/>
                  </svg>
                </button>

                <button
                  onclick={move_index_down(self.current_index.get())}
                  class="absolute left-4 top-1/2 -translate-y-1/2 p-2 bg-black/50 rounded-full text-white hover:bg-black/70 transition-colors"
                >
                  <svg class="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                    <path d="M15 19l-7-7 7-7" strokeLinecap="round" strokeLinejoin="round"/>
                  </svg>
                </button>
                <button
                  onclick={move_index_up(self.current_index.get())}
                  class="absolute right-4 top-1/2 -translate-y-1/2 p-2 bg-black/50 rounded-full text-white hover:bg-black/70 transition-colors"
                >
                  <svg class="w-6 h-6" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                    <path d="M9 5l7 7-7 7" strokeLinecap="round" strokeLinejoin="round"/>
                  </svg>
                </button>
              </div>
                    <>{self.map_images(ctx)}</>
                            {

                if self.is_modal_open.get() {
                self.modal(ctx)
            } else {
                html!{}
            }
        }

            </div>

        }
    }
}
