#[macro_use]
mod core;
mod examples;

use crate::core::application::Application;

use anyhow::{anyhow, Result};
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

use crate::core::application::Loop;
use crate::core::web::get_canvas_by_id;
use examples::e00_base_test::TestApp;
use examples::e01_point::PointApp;
use examples::e02_hexagon_lines::HexagonLines;
use examples::e03_two_shapes::TwoShapes;
use examples::e04_vertex_colors::VertexColors;
use examples::e05_two_triangles::TwoTriangles;
use examples::e06_animate_triangle::AnimateTriangle;
use examples::e07_animate_triangle_time::AnimateTriangleTime;

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let canvas = get_canvas_by_id("canvas").unwrap();

    let examples = examples();

    Loop::run(
        &canvas,
        Box::new(move |context| {
            let example = get_element(&examples, None).expect("Cannot get example");
            example(context)
        }),
    )
    .expect("Cannot run application");

    Ok(())
}

fn get_element<'a, T>(vec: &'a Vec<T>, at: Option<usize>) -> Result<&'a T> {
    match at {
        Some(index) => vec
            .get(index)
            .ok_or_else(|| anyhow!("Cannot find element at index {:#?}", index)),
        None => vec.last().ok_or_else(|| anyhow!("Vector is empty")),
    }
}

fn examples() -> Vec<Box<dyn Fn(&WebGl2RenderingContext) -> Result<Box<dyn Application>>>> {
    vec![
        Box::new(TestApp::create),
        Box::new(PointApp::create),
        Box::new(HexagonLines::create),
        Box::new(TwoShapes::create),
        Box::new(VertexColors::create),
        Box::new(TwoTriangles::create),
        Box::new(AnimateTriangle::create),
        Box::new(AnimateTriangleTime::create),
    ]
}
