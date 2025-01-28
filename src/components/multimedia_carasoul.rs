use std::cell::Cell;
use std::ops::Add;
use std::ops::Sub;
use std::rc::Rc;
use web_sys::console;
use yew::prelude::*;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct CarouselProps {
    pub items: Vec<CarouselItem>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct CarouselItem {
    src: AttrValue,
    alt: AttrValue,
    caption: AttrValue,
    item_type: MediaType,
}

#[derive(Clone, PartialEq)]
pub enum MediaType {
    Image,
    Video,
}

impl CarouselItem {
    pub fn new_image(
        src: impl Into<AttrValue>,
        alt: impl Into<AttrValue>,
        caption: impl Into<AttrValue>,
    ) -> Self {
        Self {
            src: src.into(),
            alt: alt.into(),
            caption: caption.into(),
            item_type: MediaType::Image,
        }
    }

    pub fn new_video(
        src: impl Into<AttrValue>,
        alt: impl Into<AttrValue>,
        caption: impl Into<AttrValue>,
    ) -> Self {
        Self {
            src: src.into(),
            alt: alt.into(),
            caption: caption.into(),
            item_type: MediaType::Video,
        }
    }
}

pub struct MultiMediaCarousel {
    current_index: Rc<Cell<usize>>,
    is_modal_open: Rc<Cell<bool>>,
    items: Vec<CarouselItem>,
}

impl MultiMediaCarousel {
    fn render_media_item(&self, item: &CarouselItem) -> Html {
        match item.item_type {
            MediaType::Image => html! {
                <img
                    src={item.src.clone()}
                    alt={item.alt.clone()}
                    class="w-full h-full object-contain cursor-pointer"
                />
            },
            MediaType::Video => html! {
                <video
                    src={item.src.clone()}
                    controls=true
                    class="w-full h-full object-contain cursor-pointer"
                >
                    <track kind="captions" />
                </video>
            },
        }
    }

    fn map_images(&self, ctx: &Context<Self>) -> Vec<Html> {
        let handle_thumbnail_click = |index: usize| {
            let link = ctx.link().clone();
            move |_: MouseEvent| {
                link.send_message(Msg::MoveIndex(index));
            }
        };

        self.items
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
        let current_image = &self.items[self.current_index.get()].clone();

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

impl Component for MultiMediaCarousel {
    type Message = Msg;
    type Properties = CarouselProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let current_index = Rc::new(Cell::new(0));
        let is_modal_open = Rc::new(Cell::new(false));
        Self {
            current_index,
            is_modal_open,
            items: _ctx.props().items.clone(),
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
        let current_item = &self.items[self.current_index.get()].clone();

        let move_index_up = |index: usize| {
            let link = ctx.link().clone();
            let len = self.items.len();
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
            let len = self.items.len();
            move |_: MouseEvent| {
                if index == 0 {
                    link.send_message(Msg::MoveIndex(len.sub(1)));
                } else {
                    link.send_message(Msg::MoveIndex(index.sub(1)));
                }
            }
        };
        html! {
        <div class="w-full max-w-4xl mx-auto relative flex flex-col">
                <div class="relative h-[60vh] bg-gray-900 rounded-lg overflow-hidden">
                    <div class="absolute top-0 left-0 right-0 p-4 bg-gradient-to-b from-black/90 to-black/50 z-10">
                      <p class="text-white text-center">
                            {current_item.caption.clone()}
                      </p>
                    </div>

                    {self.render_media_item(current_item)}
                <img
                  src={current_item.src.clone()}
                  alt={current_item.alt.clone()}
                  class="w-full h-full object-contain cursor-pointer"
                />

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
