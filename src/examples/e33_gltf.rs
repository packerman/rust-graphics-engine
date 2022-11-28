use std::fmt;

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    core::{
        application::{self, Application, AsyncCreator},
        input::KeyState,
    },
    gltf,
};

enum Variant {
    Gltf,
}

impl Variant {
    const GLTF: &str = "gltf";

    fn extension(&self) -> &str {
        Self::GLTF
    }
}

impl Default for Variant {
    fn default() -> Self {
        Self::Gltf
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Variant::Gltf => write!(f, "glTF"),
        }
    }
}

fn khronos_sample(name: &str, variant: Variant) -> String {
    format!(
        "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0/{}/{}/{}.{}",
        name,
        variant.to_string(),
        name,
        variant.extension()
    )
}

struct Example {}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(_context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        let gltf = gltf::read::fetch_gltf(&khronos_sample(
            "TriangleWithoutIndices",
            Default::default(),
        ))
        .await?;
        debug!("{:#?}", gltf);
        Ok(Box::new(Example {}))
    }
}

impl Application for Example {
    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, _context: &WebGl2RenderingContext) {}
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
