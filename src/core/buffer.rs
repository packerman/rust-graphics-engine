use js_sys::{ArrayBuffer, Float32Array, Uint16Array};

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
