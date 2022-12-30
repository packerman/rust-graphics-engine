use std::mem;

use js_sys::{ArrayBuffer, Float32Array, Uint16Array};

#[derive(Debug, Clone)]
pub struct Buffer {
    buffer_type: BufferType,
    byte_length: usize,
}

impl Buffer {
    pub fn new(array_buffer: ArrayBuffer, byte_length: usize) -> Self {
        Self {
            buffer_type: BufferType::ArrayBuffer(array_buffer),
            byte_length,
        }
    }

    pub fn get_float32_array(&self, byte_offset: u32, length: u32) -> Float32Array {
        self.buffer_type.get_float32_array(byte_offset, length)
    }

    pub fn get_uint16_array(&self, byte_offset: u32, length: u32) -> Uint16Array {
        self.buffer_type.get_uint16_array(byte_offset, length)
    }
}

#[derive(Debug, Clone)]
pub enum BufferType {
    ArrayBuffer(ArrayBuffer),
    Float32Array(Float32Array),
}

impl BufferType {
    const SIZE_OF_F32: u32 = mem::size_of::<f32>() as u32;

    pub fn get_float32_array(&self, byte_offset: u32, length: u32) -> Float32Array {
        match self {
            Self::ArrayBuffer(array_buffer) => {
                Float32Array::new_with_byte_offset_and_length(array_buffer, byte_offset, length)
            }
            Self::Float32Array(array) => {
                assert!(byte_offset % Self::SIZE_OF_F32 == 0);
                array.subarray(byte_offset / Self::SIZE_OF_F32, length)
            }
        }
    }

    pub fn get_uint16_array(&self, byte_offset: u32, length: u32) -> Uint16Array {
        match self {
            Self::ArrayBuffer(array_buffer) => {
                Uint16Array::new_with_byte_offset_and_length(array_buffer, byte_offset, length)
            }
            Self::Float32Array(array) => {
                Self::panic_forbidden_conversion("Float32Array", "Uint16Array")
            }
        }
    }

    fn panic_forbidden_conversion(from: &str, to: &str) -> ! {
        panic!("Cannot convert {} to {}", from, to)
    }
}

impl From<Float32Array> for Buffer {
    fn from(array: Float32Array) -> Self {
        Self {
            buffer_type: BufferType::Float32Array(array),
            byte_length: array.byte_length().try_into().unwrap(),
        }
    }
}
