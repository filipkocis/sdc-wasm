mod visual;
mod ai;
mod world;
mod utils;
mod game;

pub use world::*;
pub use ai::*;
pub use visual::*;
pub use utils::*;

use game::Game;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    panic_utils::set_panic_hook();

    let game = Game::new();
    game.start();

    Ok(()) 
}
