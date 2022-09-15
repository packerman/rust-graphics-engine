use anyhow::{anyhow, Result};

use crate::{
    core::application,
    examples::{
        e00_base_test::TestExample, e01_point::PointExample,
        e02_hexagon_lines::HexagonLinesExample, e03_two_shapes::TwoShapesExample,
        e04_vertex_colors::VertexColorsExample, e05_two_triangles::TwoTrianglesExample,
        e06_animate_triangle::AnimateTriangleExample,
        e07_animate_triangle_time::AnimateTriangleTimeExample,
        e08_keyboard_input::KeyboardInputExample, e09_move_triangle::MoveTriangleExample,
        e0a_spinning_cube::SpinningCubeExample, e0b_axes_grid::AxesGridExample,
        e0c_movement_rig::MovementRigExample, e0d_texture::TextureExample,
        e0e_more_textures::MoreTexturesExample,
    },
};

pub fn run_example() {
    run_example_by_index(None)
}

fn examples() -> Vec<Box<dyn Fn()>> {
    vec![
        Box::new(application::spawn::<TestExample>),
        Box::new(application::spawn::<PointExample>),
        Box::new(application::spawn::<HexagonLinesExample>),
        Box::new(application::spawn::<TwoShapesExample>),
        Box::new(application::spawn::<VertexColorsExample>),
        Box::new(application::spawn::<TwoTrianglesExample>),
        Box::new(application::spawn::<AnimateTriangleExample>),
        Box::new(application::spawn::<AnimateTriangleTimeExample>),
        Box::new(application::spawn::<KeyboardInputExample>),
        Box::new(application::spawn::<MoveTriangleExample>),
        Box::new(application::spawn::<SpinningCubeExample>),
        Box::new(application::spawn::<AxesGridExample>),
        Box::new(application::spawn::<MovementRigExample>),
        Box::new(application::spawn::<TextureExample>),
        Box::new(application::spawn::<MoreTexturesExample>),
    ]
}

fn run_example_by_index(index: Option<usize>) {
    let examples = examples();
    let example = get_element(&examples, index).unwrap();

    example();
}

fn get_element<T>(vec: &[T], index: Option<usize>) -> Result<&T> {
    match index {
        Some(index) => vec
            .get(index)
            .ok_or_else(|| anyhow!("Cannot find element at index {:#?}", index)),
        None => vec.last().ok_or_else(|| anyhow!("Vector is empty")),
    }
}
