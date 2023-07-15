use yew::prelude::*;

use super::flash_card;
use super::login;

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <>
    <p
      class={classes!("bg-red-100")}>
      {"Hello, World!"}
    </p>
    <login::Login/>
    <flash_card::FlashCard/>
    </>
  }
}
