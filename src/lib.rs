extern crate nalgebra as na;
extern crate nalgebra_glm as glm;

#[macro_use]
mod core;
mod examples;
mod run_example;

use wasm_bindgen::prelude::*;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    run_example::run_example();

    Ok(())
}
