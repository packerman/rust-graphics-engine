use anyhow::{anyhow, Result};

use crate::examples::{
    e00_base_test, e01_point, e02_hexagon_lines, e03_two_shapes, e04_vertex_colors,
    e05_two_triangles, e06_animate_triangle, e07_animate_triangle_time, e08_keyboard_input,
    e09_move_triangle, e10_spinning_cube, e11_axes_grid, e12_movement_rig, e13_texture,
    e14_more_textures, e15_spinning_textured_cube, e16_spinning_textured_sphere, e17_skysphere,
    e18_wave_texture, e19_blend_textures, e20_distort_texture, e21_procedural_texture,
    e22_text_texture, e23_billboarding, e24_sprite_material,
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
        e10_spinning_cube::example(),
        e11_axes_grid::example(),
        e12_movement_rig::example(),
        e13_texture::example(),
        e14_more_textures::example(),
        e15_spinning_textured_cube::example(),
        e16_spinning_textured_sphere::example(),
        e17_skysphere::example(),
        e18_wave_texture::example(),
        e19_blend_textures::example(),
        e20_distort_texture::example(),
        e21_procedural_texture::example(),
        e22_text_texture::example(),
        e23_billboarding::example(),
        e24_sprite_material::example(),
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
