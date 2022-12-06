use web_sys::{WebGl2RenderingContext, WebGlBuffer};

pub enum Attribute {
    Position,
}

pub struct GlBuffer {
    buffer: WebGlBuffer,
    target: u32,
}

impl GlBuffer {
    pub fn new(buffer: WebGlBuffer, target: u32) -> Self {
        Self { buffer, target }
    }

    pub fn bind(&self, context: &WebGl2RenderingContext) {
        context.bind_buffer(self.target, Some(&self.buffer));
    }

    pub fn unbind(&self, context: &WebGl2RenderingContext) {
        context.bind_buffer(self.target, None);
    }
}
