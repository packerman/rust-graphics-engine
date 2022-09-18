use anyhow::{anyhow, Result};

use crate::examples::{
    e00_base_test, e01_point, e02_hexagon_lines, e03_two_shapes, e04_vertex_colors,
    e05_two_triangles, e06_animate_triangle, e07_animate_triangle_time, e08_keyboard_input,
    e09_move_triangle, e0a_spinning_cube, e0b_axes_grid, e0c_movement_rig, e0d_texture,
    e0e_more_textures, e0f_spinning_textured_cube, e0g_spinning_textured_sphere, e0h_skysphere,
    e0i_wave_texture, e0j_blend_textures, e0k_distort_texture,
};

pub fn run_example() {
    run_example_by_index(None)
}

fn examples() -> Vec<Box<dyn Fn()>> {
    vec![
        e00_base_test::example(),
        e01_point::example(),
        e02_hexagon_lines::example(),
        e03_two_shapes::example(),
        e04_vertex_colors::example(),
        e05_two_triangles::example(),
        e06_animate_triangle::example(),
        e07_animate_triangle_time::example(),
        e08_keyboard_input::example(),
        e09_move_triangle::example(),
        e0a_spinning_cube::example(),
        e0b_axes_grid::example(),
        e0c_movement_rig::example(),
        e0d_texture::example(),
        e0e_more_textures::example(),
        e0f_spinning_textured_cube::example(),
        e0g_spinning_textured_sphere::example(),
        e0h_skysphere::example(),
        e0i_wave_texture::example(),
        e0j_blend_textures::example(),
        e0k_distort_texture::example(),
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
