#[macro_use]
mod core;
mod examples;

use crate::core::application::Loop;

use examples::e04_two_shapes::TwoShapes;
use wasm_bindgen::prelude::*;

use crate::core::web::get_canvas_by_id;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let canvas = get_canvas_by_id("canvas").unwrap();

    Loop::run(&canvas, Box::new(TwoShapes::create)).expect("Cannot run application");

    Ok(())
}
