// =============================================================================
//! - App component for CroftSoft Yew Prototype
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
