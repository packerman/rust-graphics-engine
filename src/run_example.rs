use anyhow::{anyhow, Result};

use crate::examples::{
    e10_spinning_cube, e11_axes_grid, e12_movement_rig, e13_texture, e14_more_textures,
    e15_spinning_textured_cube, e16_spinning_textured_sphere, e17_skysphere, e18_wave_texture,
    e19_blend_textures, e20_distort_texture, e21_procedural_texture, e22_text_texture,
    e23_billboarding, e24_sprite_material, e25_heads_up_display, e26_render_to_texture,
    e27_compound_effect, e28_lights, e29_bump_mapping, e30_bloom_effect, e31_glow_effect,
};

pub fn run_example() {
    run_example_by_index(None)
}

fn examples() -> Vec<Box<dyn Fn()>> {
    vec![
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
        e25_heads_up_display::example(),
        e26_render_to_texture::example(),
        e27_compound_effect::example(),
        e28_lights::example(),
        e29_bump_mapping::example(),
        e30_bloom_effect::example(),
        e31_glow_effect::example(),
        // e32_shadows::example(),
        // e33_gltf::example(),
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
