extern crate nalgebra_glm as glm;

#[macro_use]
mod core;
mod examples;
mod run_example;

use crate::core::web;

use wasm_bindgen::prelude::*;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let canvas = web::get_canvas_by_id("canvas").expect("Cannot find canvas");

    run_example::run_example(&canvas).expect("Cannot run application");

    Ok(())
}
