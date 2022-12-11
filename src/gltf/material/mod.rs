use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::core::{convert::FromWithContext, gl};

use super::program::Program;

pub fn basic(context: &WebGl2RenderingContext) -> Result<Program> {
    gl::build_program(
        context,
        include_str!("basic.vert"),
        include_str!("basic.frag"),
    )
    .and_then(|program| Program::from_with_context(context, program))
}
