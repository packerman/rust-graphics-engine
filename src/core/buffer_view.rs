use std::rc::Rc;

use anyhow::{anyhow, Result};
use js_sys::{Float32Array, Uint16Array};
use web_sys::WebGl2RenderingContext;

use crate::base::util::validate;

use super::buffer::Buffer;

#[derive(Debug, Clone)]
pub struct BufferView {
    buffer: Rc<Buffer>,
    byte_offset: u32,
    byte_length: u32,
    pub byte_stride: i32,
    pub target: Option<u32>,
}

impl BufferView {
    const TARGETS: [u32; 2] = [
        WebGl2RenderingContext::ARRAY_BUFFER,
        WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
    ];

    pub fn new(
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
        Ok(Self {
            buffer,
            byte_offset,
            byte_length,
            target,
            byte_stride: byte_stride.unwrap_or_default(),
        })
    }

    pub fn get_float32_array(&self, byte_offset: u32, length: u32) -> Float32Array {
        self.buffer
            .get_float32_array(self.byte_offset + byte_offset, length)
    }

    pub fn get_uint16_array(&self, byte_offset: u32, length: u32) -> Uint16Array {
        self.buffer
            .get_uint16_array(self.byte_offset + byte_offset, length)
    }

    pub fn unbind(context: &WebGl2RenderingContext, has_indices: bool) {
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
        if has_indices {
            context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, None);
        }
    }
}
