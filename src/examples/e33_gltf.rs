use std::fmt;

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    base::{
        application::{self, Application, AsyncCreator},
        gl::diagnostic::GlDiagnostics,
        input::KeyState,
    },
    gltf::{self, core::Root},
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
        variant,
        name,
        variant.extension()
    )
}

struct Example {
    root: Root,
}

fn example_names<'a>() -> Vec<&'a str> {
    vec![
        "TriangleWithoutIndices",
        "Triangle",
        "SimpleMeshes",
        "Cameras",
        "Box",
        "BoxInterleaved",
        "BoxTextured",
        "BoxTexturedNonPowerOfTwo",
        "Box%20With%20Spaces",
        "BoxVertexColors",
        "Duck",
        "2CylinderEngine",
        "ReciprocatingSaw",
        "GearboxAssy",
        "Buggy",
    ]
}

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        debug!("{:#?}", GlDiagnostics::collect(context)?);
        let root = gltf::load::load(
            context,
            &khronos_sample(example_names()[9], Default::default()),
        )
        .await?;
        Ok(Box::new(Example { root }))
    }
}

impl Application for Example {
    fn name(&self) -> &str {
        "glTF"
    }

    fn update(&mut self, key_state: &KeyState) {
        self.root.update(key_state)
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.root.render(context);
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
