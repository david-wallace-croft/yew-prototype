pub mod components;

use components::app::App;
use yew::Renderer;

fn main() {
  Renderer::<App>::new().render();
}
