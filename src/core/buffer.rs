use js_sys::{ArrayBuffer, Float32Array, Uint16Array};

#[derive(Debug, Clone)]
pub struct Buffer {
    byffer_type: BufferType,
    byte_length: usize,
}

impl Buffer {
    pub fn new(array_buffer: ArrayBuffer, byte_length: usize) -> Self {
        Self {
            byffer_type: BufferType::ArrayBuffer(array_buffer),
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

pub enum BufferType {
    ArrayBuffer(ArrayBuffer),
    Float32Array(Float32Array),
}

impl From<Float32Array> for Buffer {
    fn from(array: Float32Array) -> Self {
        Self {
            byffer_type: BufferType::Float32Array(array),
            byte_length: array.byte_length(),
        }
    }
}
