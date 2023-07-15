use yew::prelude::*;

#[function_component(FlashCard)]
pub fn flash_card() -> Html {
  html! {
  <>
  <button
    type="button">
    {"Show"}
  </button>
  <h1>{ "8 x 6 = ?" }</h1>
  <button
    type="button">
  {"16"}
  </button>
  <button
    type="button">
  {"48"}
  </button>
  <button
    type="button">
  {"108"}
  </button>
  </>
  }
}