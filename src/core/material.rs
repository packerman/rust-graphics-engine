use web_sys::WebGlProgram;

pub struct Material {
    program: WebGlProgram,
}

impl Material {
    pub fn program(&self) -> &WebGlProgram {
        &self.program
    }
}
