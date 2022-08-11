use anyhow::{anyhow, Result};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use crate::{
    core::application::{Application, Loop},
    examples::{
        e00_base_test::TestApp, e01_point::PointApp, e02_hexagon_lines::HexagonLines,
        e03_two_shapes::TwoShapes, e04_vertex_colors::VertexColors,
        e05_two_triangles::TwoTriangles, e06_animate_triangle::AnimateTriangle,
        e07_animate_triangle_time::AnimateTriangleTime, e08_keyboard_input::KeyboardInput,
        e09_move_triangle::MoveTriangle,
    },
};

pub fn run_example(canvas: &HtmlCanvasElement) -> Result<()> {
    run_example_by_index(canvas, None)
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
        Box::new(KeyboardInput::create),
        Box::new(MoveTriangle::create),
    ]
}

fn run_example_by_index(canvas: &HtmlCanvasElement, index: Option<usize>) -> Result<()> {
    let examples = examples();

    Loop::run(
        &canvas,
        Box::new(move |context| {
            let example = get_element(&examples, index).expect("Cannot get example");
            example(context)
        }),
    )
}

fn get_element<'a, T>(vec: &'a Vec<T>, index: Option<usize>) -> Result<&'a T> {
    match index {
        Some(index) => vec
            .get(index)
            .ok_or_else(|| anyhow!("Cannot find element at index {:#?}", index)),
        None => vec.last().ok_or_else(|| anyhow!("Vector is empty")),
    }
}
