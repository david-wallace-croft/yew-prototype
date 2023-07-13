use yew::prelude::*;

pub mod pages;

#[function_component(App)]
fn app() -> Html {
  html! {
    <>
    <pages::flash_card::FlashCard />
    </>
  }
}

fn main() {
  yew::Renderer::<App>::new().render();
}
