use std::collections::HashMap;

use web_sys::WebGlProgram;

use super::uniform::Uniform;

pub struct Material<'a> {
    program: WebGlProgram,
    uniforms: HashMap<String, Uniform<'a>>,
}

impl Material<'_> {
    pub fn program(&self) -> &WebGlProgram {
        &self.program
    }
}
