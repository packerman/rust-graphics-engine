use std::{mem::size_of, rc::Rc};

use anyhow::{anyhow, Result};
use js_sys::{ArrayBuffer, DataView, Float32Array, Uint16Array};
use web_sys::{WebGl2RenderingContext, WebGlBuffer};

use crate::{core::gl, gltf::util::validate};

#[derive(Debug, Clone)]
pub struct Buffer {
    array_buffer: ArrayBuffer,
    byte_length: usize,
}

impl Buffer {
    pub fn new(array_buffer: ArrayBuffer, byte_length: usize) -> Self {
        Self {
            array_buffer,
            byte_length,
        }
    }

    pub fn get_float32_array(&self, byte_offset: u32, length: u32) -> Float32Array {
        Float32Array::new_with_byte_offset_and_length(&self.array_buffer, byte_offset, length)
    }

    pub fn get_uint16_array(&self, byte_offset: u32, length: u32) -> Uint16Array {
        Uint16Array::new_with_byte_offset_and_length(&self.array_buffer, byte_offset, length)
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

    pub fn get_float32_array(&self, byte_offset: u32, length: u32) -> Float32Array {
        self.data_buffer
            .get_float32_array(self.byte_offset + byte_offset, length)
    }

    pub fn get_uint16_array(&self, byte_offset: u32, length: u32) -> Uint16Array {
        self.data_buffer
            .get_uint16_array(self.byte_offset + byte_offset, length)
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

#[derive(Debug, Clone, Copy)]
pub enum AccessorType {
    Vec { size: i32 },
    Scalar,
}

impl AccessorType {
    pub fn vec(size: i32) -> AccessorType {
        Self::Vec { size }
    }

    pub fn scalar() -> AccessorType {
        Self::Scalar
    }

    pub fn size(&self) -> i32 {
        match self {
            Self::Vec { size } => *size,
            Self::Scalar => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AccessorProperties {
    pub byte_offset: u32,
    pub component_type: u32,
    pub count: i32,
    pub accessor_type: AccessorType,
    pub min: Option<Vec<f32>>,
    pub max: Option<Vec<f32>>,
    pub normalized: bool,
}

#[derive(Debug, Clone)]
pub struct Accessor {
    buffer_view: Option<Rc<BufferView>>,
    byte_offset: u32,
    pub component_type: u32,
    pub count: i32,
    accessor_type: AccessorType,
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
        properties: AccessorProperties,
    ) -> Result<Self> {
        validate::contains(
            &properties.component_type,
            &Self::COMPONENT_TYPES,
            |value| anyhow!("Unknown component type: {}", value),
        )?;
        Ok(Self {
            buffer_view,
            byte_offset: properties.byte_offset,
            component_type: properties.component_type,
            count: properties.count,
            accessor_type: properties.accessor_type,
            min: properties.min,
            max: properties.max,
            normalized: properties.normalized,
        })
    }

    pub fn set_vertex_attribute(&self, context: &WebGl2RenderingContext, location: u32) {
        if let Some(buffer_view) = &self.buffer_view {
            self.buffer_data(context, buffer_view);
            context.vertex_attrib_pointer_with_i32(
                location,
                self.accessor_type.size(),
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
            self.buffer_data(context, buffer_view);
        }
    }

    fn buffer_data(&self, context: &WebGl2RenderingContext, buffer_view: &BufferView) {
        let array_length = self.get_array_length(buffer_view);
        match self.component_type {
            WebGl2RenderingContext::UNSIGNED_SHORT => {
                let view = buffer_view.get_uint16_array(self.byte_offset, array_length as u32);
                buffer_view.buffer_data(context, &view);
            }
            WebGl2RenderingContext::FLOAT => {
                let view = buffer_view.get_float32_array(self.byte_offset, array_length as u32);
                buffer_view.buffer_data(context, &view);
            }
            _ => panic!("Unknown accessor component type: {}", self.component_type),
        }
    }

    fn get_array_length(&self, buffer_view: &BufferView) -> i32 {
        let size = self.accessor_type.size();
        if buffer_view.byte_stride > 0 {
            buffer_view.byte_stride / (self.component_byte_length() as i32) * (self.count - 1)
                + size
        } else {
            self.count * size
        }
    }

    fn component_byte_length(&self) -> usize {
        match self.component_type {
            WebGl2RenderingContext::BYTE => size_of::<i8>(),
            WebGl2RenderingContext::UNSIGNED_BYTE => size_of::<u8>(),
            WebGl2RenderingContext::SHORT => size_of::<i16>(),
            WebGl2RenderingContext::UNSIGNED_SHORT => size_of::<u16>(),
            WebGl2RenderingContext::UNSIGNED_INT => size_of::<u32>(),
            WebGl2RenderingContext::FLOAT => size_of::<f32>(),
            _ => panic!("Unknown accessor component type: {}", self.component_type),
        }
    }
}
