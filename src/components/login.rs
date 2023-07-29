// =============================================================================
//! - Login component for CroftSoft Yew Prototype
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

#[function_component(Login)]
pub fn login() -> Html {
  html! {
  <>
  <form>
  <label>
  {"Username"}
  </label>
  <input
    type="text"/>
  <label>
  {"Password"}
  </label>
  <input
    type="password"/>
  <button
    type="button">
  {"Login"}
  </button>
  </form>
  </>
  }
}
