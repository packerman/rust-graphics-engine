#[macro_use]
mod core;
mod base_test;
mod hexagon_points;
mod point;

use crate::core::application::Loop;

use base_test::TestApp;
use hexagon_points::HexagonPoints;
use point::PointApp;
use wasm_bindgen::{prelude::*, JsCast};

use crate::core::web::get_canvas_by_id;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let canvas = get_canvas_by_id("canvas").unwrap();

    Loop::run(&canvas, Box::new(HexagonPoints::create)).expect("Cannot run application");

    Ok(())
}
