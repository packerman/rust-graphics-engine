use std::rc::Rc;

use anyhow::Result;
use js_sys::Float32Array;
use na::SVector;
use web_sys::WebGl2RenderingContext;

use crate::{
    base::convert::FromWithContext,
    core::{
        accessor::{Accessor, AccessorProperties, AccessorType},
        buffer::Buffer,
        buffer_view::BufferView,
    },
};

impl<const N: usize, const K: usize> FromWithContext<WebGl2RenderingContext, &[[f32; N]; K]>
    for Accessor
{
    fn from_with_context(context: &WebGl2RenderingContext, data: &[[f32; N]; K]) -> Result<Self> {
        fn flatten_array<T: Clone, const N: usize, const K: usize>(data: &[[T; N]; K]) -> Vec<T> {
            data.iter().flat_map(|item| item.to_vec()).collect()
        }
        self::create_accessor(context, flatten_array(data), N, data.len())
    }
}

impl<const N: usize> FromWithContext<WebGl2RenderingContext, &Vec<[f32; N]>> for Accessor {
    fn from_with_context(context: &WebGl2RenderingContext, data: &Vec<[f32; N]>) -> Result<Self> {
        fn flatten_array<T: Clone, const N: usize>(data: &[[T; N]]) -> Vec<T> {
            data.iter().flat_map(|item| item.to_vec()).collect()
        }
        self::create_accessor(context, flatten_array(data), N, data.len())
    }
}

impl<const N: usize> FromWithContext<WebGl2RenderingContext, &Vec<SVector<f32, N>>> for Accessor {
    fn from_with_context(
        context: &WebGl2RenderingContext,
        data: &Vec<SVector<f32, N>>,
    ) -> Result<Self> {
        fn flatten_vector<T: Copy, const N: usize>(data: &[SVector<T, N>]) -> Vec<T> {
            data.iter()
                .flat_map(|item| item.iter().copied().collect::<Vec<T>>())
                .collect()
        }
        self::create_accessor(context, flatten_vector(data), N, data.len())
    }
}

impl<const N: usize, const K: usize> FromWithContext<WebGl2RenderingContext, &[SVector<f32, N>; K]>
    for Accessor
{
    fn from_with_context(
        context: &WebGl2RenderingContext,
        data: &[SVector<f32, N>; K],
    ) -> Result<Self> {
        fn flatten_vector<T: Copy, const N: usize>(data: &[SVector<T, N>]) -> Vec<T> {
            data.iter()
                .flat_map(|item| item.iter().copied().collect::<Vec<T>>())
                .collect()
        }
        self::create_accessor(context, flatten_vector(data), N, data.len())
    }
}

fn create_accessor(
    context: &WebGl2RenderingContext,
    data: Vec<f32>,
    size: usize,
    length: usize,
) -> Result<Accessor> {
    let array = Float32Array::new_with_length(data.len());
    array.copy_from(&data);
    let byte_length = array.byte_length();
    let buffer = Rc::new(Buffer::from(array));
    let buffer_view = Rc::new(BufferView::new(
        buffer,
        0,
        byte_length,
        None,
        Some(WebGl2RenderingContext::ARRAY_BUFFER),
    )?);
    let properties = AccessorProperties {
        byte_offset: 0,
        component_type: WebGl2RenderingContext::FLOAT,
        count: length,
        accessor_type: AccessorType::vec(size),
        min: None,
        max: None,
        normalized: false,
    };
    Accessor::initialize(context, Some(buffer_view), properties)
}
