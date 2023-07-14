use yew::prelude::*;

use super::flash_card;
use super::login;

#[function_component(App)]
pub fn app() -> Html {
  html! {
    <>
    <login::Login/>
    <flash_card::FlashCard/>
    </>
  }
}
