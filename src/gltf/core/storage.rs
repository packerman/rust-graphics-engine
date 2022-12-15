use std::rc::Rc;

use anyhow::{anyhow, Result};
use js_sys::{ArrayBuffer, DataView};
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

use crate::{core::gl, gltf::validate};

#[derive(Debug, Clone)]
pub struct Buffer {
    array_buffer: ArrayBuffer,
    byte_length: u32,
}

impl Buffer {
    pub fn new(array_buffer: ArrayBuffer, byte_length: u32) -> Self {
        Self {
            array_buffer,
            byte_length,
        }
    }

    pub fn get_data_view(&self, byte_offset: usize, byte_length: usize) -> DataView {
        DataView::new(&self.array_buffer, byte_offset, byte_length)
    }
}

#[derive(Debug, Clone)]
pub struct BufferView {
    gl_buffer: Option<WebGlBuffer>,
    data_buffer: Rc<Buffer>,
    byte_offset: u32,
    byte_length: u32,
    pub byte_stride: i32,
    target: Option<u32>,
}

impl BufferView {
    const TARGETS: [u32; 2] = [
        WebGl2RenderingContext::ARRAY_BUFFER,
        WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
    ];

    pub fn new(
        context: &WebGl2RenderingContext,
        buffer: Rc<Buffer>,
        byte_offset: u32,
        byte_length: u32,
        byte_stride: Option<i32>,
        target: Option<u32>,
    ) -> Result<Self> {
        validate::optional(&target, |target| {
            validate::contains(target, &Self::TARGETS, |value| {
                anyhow!("Unknown target: {}", value)
            })
        })?;
        let gl_buffer = if target.is_some() {
            Some(gl::create_buffer(context)?)
        } else {
            None
        };
        Ok(Self {
            gl_buffer,
            data_buffer: buffer,
            byte_offset,
            byte_length,
            target,
            byte_stride: byte_stride.unwrap_or_default(),
        })
    }

    pub fn bind(&self, context: &WebGl2RenderingContext) {
        if let Some(target) = self.target {
            context.bind_buffer(target, self.gl_buffer.as_ref());
        }
    }

    pub fn get_data_view(&self, byte_offset: u32) -> DataView {
        self.data_buffer.get_data_view(
            (self.byte_offset + byte_offset) as usize,
            self.byte_length as usize,
        )
    }

    pub fn buffer_data(&self, context: &WebGl2RenderingContext, data: &js_sys::Object) {
        if let Some(target) = self.target {
            context.bind_buffer(target, self.gl_buffer.as_ref());
            context.buffer_data_with_array_buffer_view(
                target,
                data,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
    }

    pub fn unbind(context: &WebGl2RenderingContext, has_indices: bool) {
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
        if has_indices {
            context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Accessor {
    buffer_view: Option<Rc<BufferView>>,
    byte_offset: i32,
    pub component_type: u32,
    pub count: i32,
    size: i32,
    min: Option<Vec<f32>>,
    max: Option<Vec<f32>>,
    pub normalized: bool,
}

impl Accessor {
    const COMPONENT_TYPES: [u32; 6] = [
        WebGl2RenderingContext::BYTE,
        WebGl2RenderingContext::UNSIGNED_BYTE,
        WebGl2RenderingContext::SHORT,
        WebGl2RenderingContext::UNSIGNED_SHORT,
        WebGl2RenderingContext::UNSIGNED_INT,
        WebGl2RenderingContext::FLOAT,
    ];

    pub fn new(
        buffer_view: Option<Rc<BufferView>>,
        byte_offset: i32,
        component_type: u32,
        count: i32,
        size: i32,
        min: Option<Vec<f32>>,
        max: Option<Vec<f32>>,
        normalized: bool,
    ) -> Result<Self> {
        validate::contains(&component_type, &Self::COMPONENT_TYPES, |value| {
            anyhow!("Unknown component type: {}", value)
        })?;
        Ok(Self {
            buffer_view,
            byte_offset,
            component_type,
            count,
            size,
            min,
            max,
            normalized,
        })
    }

    pub fn set_vertex_attribute(&self, context: &WebGl2RenderingContext, location: u32) {
        if let Some(buffer_view) = &self.buffer_view {
            let data_view = buffer_view.get_data_view(self.byte_offset as u32);
            buffer_view.buffer_data(context, &data_view);
            context.vertex_attrib_pointer_with_i32(
                location,
                self.size,
                self.component_type,
                self.normalized,
                buffer_view.byte_stride,
                0,
            );
            context.enable_vertex_attrib_array(location);
        }
    }

    pub fn set_indices(&self, context: &WebGl2RenderingContext) {
        if let Some(buffer_view) = &self.buffer_view {
            let data_view = buffer_view.get_data_view(self.byte_offset as u32);
            buffer_view.buffer_data(context, &data_view);
        }
    }
}
