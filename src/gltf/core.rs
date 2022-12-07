use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlVertexArrayObject};

pub struct BufferView {
    buffer: WebGlBuffer,
    target: u32,
    pub byte_stride: i32,
}

impl BufferView {
    pub fn new(buffer: WebGlBuffer, target: u32, byte_stride: i32) -> Self {
        Self {
            buffer,
            target,
            byte_stride,
        }
    }

    pub fn bind(&self, context: &WebGl2RenderingContext) {
        context.bind_buffer(self.target, Some(&self.buffer));
    }

    pub fn unbind(&self, context: &WebGl2RenderingContext) {
        context.bind_buffer(self.target, None);
    }
}

pub struct Primitive {
    vertex_array: WebGlVertexArrayObject,
    count: i32,
}

impl Primitive {
    pub fn new(vertex_array: WebGlVertexArrayObject, count: i32) -> Self {
        Self {
            vertex_array,
            count,
        }
    }
}

pub struct Mesh {
    primitives: Vec<Primitive>,
}

impl Mesh {
    pub fn new(primitives: Vec<Primitive>) -> Self {
        Self { primitives }
    }
}
