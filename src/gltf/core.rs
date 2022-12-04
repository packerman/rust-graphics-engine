use web_sys::WebGlBuffer;

pub struct GlBuffer {
    buffer: WebGlBuffer,
    target: u32,
}

impl GlBuffer {
    pub fn new(buffer: WebGlBuffer, target: u32) -> Self {
        Self { buffer, target }
    }
}
