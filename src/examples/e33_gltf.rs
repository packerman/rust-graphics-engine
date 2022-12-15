use std::fmt;

use anyhow::Result;
use async_trait::async_trait;
use web_sys::WebGl2RenderingContext;

use crate::{
    core::{
        application::{self, Application, AsyncCreator},
        color,
        gl::{self, diagnostic::GlDiagnostics},
        input::KeyState,
        web,
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

const EXAMPLE_NAMES: [&str; 4] = [
    "TriangleWithoutIndices",
    "Triangle",
    "SimpleMeshes",
    "Cameras",
];

#[async_trait(?Send)]
impl AsyncCreator for Example {
    async fn create(context: &WebGl2RenderingContext) -> Result<Box<Self>> {
        debug!("{:#?}", GlDiagnostics::collect(context)?);
        let root = gltf::load::load(
            context,
            &khronos_sample(EXAMPLE_NAMES[3], Default::default()),
        )
        .await?;
        log!("{:#?}", root);
        Ok(Box::new(Example { root }))
    }
}

impl Application for Example {
    fn update(&mut self, _key_state: &KeyState) {}

    fn render(&self, context: &WebGl2RenderingContext) {
        let canvas = web::get_canvas(context).unwrap();
        let size = web::canvas_size(&canvas);
        context.viewport(0, 0, size.0 as i32, size.1 as i32);
        gl::set_clear_color(context, &color::black());
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        self.root.render(context);
    }
}

pub fn example() -> Box<dyn Fn()> {
    Box::new(application::spawn::<Example>)
}
