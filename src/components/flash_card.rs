// =============================================================================
//! - Flash Card component for CroftSoft Yew Prototype
//!
//! # Metadata
//! - Copyright: &copy; 2023 [`CroftSoft Inc`]
//! - Author: [`David Wallace Croft`]
//! - Updated: 2023-07-29
//!
//! [`CroftSoft Inc`]: https://www.croftsoft.com/
//! [`David Wallace Croft`]: https://www.croftsoft.com/people/david/
// =============================================================================

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
    class={classes!("bg-blue-500", "hover:bg-blue-700", "text-white", "font-bold", "py-2", "px-4", "rounded")}
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
