mod app;
mod components;

use app::App;
use wasm_logger::Config;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
    wasm_logger::init(Config::default());
}
