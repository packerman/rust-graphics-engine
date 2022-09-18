use anyhow::{anyhow, Result};

use crate::{
    core::application,
    examples::{
        e00_base_test, e01_point, e02_hexagon_lines, e03_two_shapes, e04_vertex_colors,
        e05_two_triangles, e06_animate_triangle, e07_animate_triangle_time, e08_keyboard_input,
        e09_move_triangle, e0a_spinning_cube, e0b_axes_grid, e0c_movement_rig, e0d_texture,
        e0e_more_textures, e0f_spinning_textured_cube, e0g_spinning_textured_sphere, e0h_skysphere,
        e0i_wave_texture, e0j_blend_textures, e0k_distort_texture,
    },
};

pub fn run_example() {
    run_example_by_index(None)
}

fn examples() -> Vec<Box<dyn Fn()>> {
    vec![
        Box::new(application::spawn::<e00_base_test::Example>),
        Box::new(application::spawn::<e01_point::Example>),
        Box::new(application::spawn::<e02_hexagon_lines::Example>),
        Box::new(application::spawn::<e03_two_shapes::Example>),
        Box::new(application::spawn::<e04_vertex_colors::Example>),
        Box::new(application::spawn::<e05_two_triangles::Example>),
        Box::new(application::spawn::<e06_animate_triangle::Example>),
        Box::new(application::spawn::<e07_animate_triangle_time::Example>),
        Box::new(application::spawn::<e08_keyboard_input::Example>),
        Box::new(application::spawn::<e09_move_triangle::Example>),
        Box::new(application::spawn::<e0a_spinning_cube::Example>),
        Box::new(application::spawn::<e0b_axes_grid::Example>),
        Box::new(application::spawn::<e0c_movement_rig::Example>),
        Box::new(application::spawn::<e0d_texture::Example>),
        Box::new(application::spawn::<e0e_more_textures::Example>),
        Box::new(application::spawn::<e0f_spinning_textured_cube::Example>),
        Box::new(application::spawn::<e0g_spinning_textured_sphere::Example>),
        Box::new(application::spawn::<e0h_skysphere::Example>),
        Box::new(application::spawn::<e0i_wave_texture::Example>),
        Box::new(application::spawn::<e0j_blend_textures::Example>),
        Box::new(application::spawn::<e0k_distort_texture::Example>),
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
