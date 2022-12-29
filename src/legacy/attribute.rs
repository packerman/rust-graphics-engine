use anyhow::{anyhow, Result};
use glm::{Mat3, Mat4};
use js_sys::Float32Array;
use na::SVector;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram};

use crate::base::gl;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataType {
    size: i32,
    gl_type: u32,
}

impl DataType {
    const fn new(size: i32, gl_type: u32) -> DataType {
        DataType { size, gl_type }
    }
}

#[derive(Debug, Clone)]
pub struct AttributeData {
    data: Vec<f32>,
    data_type: DataType,
    count: i32,
}

impl AttributeData {
    fn buffer_data(&self, context: &WebGl2RenderingContext) {
        unsafe {
            let buffer_view = Float32Array::view(&self.data);
            Self::buffer_data_with_object(context, &buffer_view);
        }
    }

    fn buffer_data_with_object(context: &WebGl2RenderingContext, buffer_view: &js_sys::Object) {
        context.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            buffer_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    fn vertex_attrib_pointer(&self, context: &WebGl2RenderingContext, location: u32) {
        context.vertex_attrib_pointer_with_i32(
            location,
            self.data_type.size,
            self.data_type.gl_type,
            false,
            0,
            0,
        );
    }

    pub fn apply_matrix(&mut self, matrix: &Mat4) {
        let default_vec4 = glm::vec4(0.0, 0.0, 0.0, 1.0);
        let size = self.data_type.size.try_into().unwrap();
        let get_elem = |base: usize, offset: usize| {
            if offset < size {
                self.data[base + offset]
            } else {
                default_vec4[offset]
            }
        };
        let mut new_data = Vec::with_capacity(self.data.len());
        for i in (0..self.data.len()).step_by(size) {
            let new_vec4 = matrix
                * glm::vec4(
                    get_elem(i, 0),
                    get_elem(i, 1),
                    get_elem(i, 2),
                    get_elem(i, 3),
                );
            for j in 0..size {
                new_data.push(new_vec4[j]);
            }
        }
        self.data = new_data;
    }

    pub fn apply_matrix3(&mut self, matrix: &Mat3) {
        assert_eq!(self.data_type.size, 3);
        let mut new_data = Vec::with_capacity(self.data.len());
        for i in (0..self.data.len()).step_by(3) {
            let new_vec3 = matrix * glm::vec3(self.data[i], self.data[i + 1], self.data[i + 2]);
            new_data.extend(&new_vec3);
        }
        self.data = new_data;
    }

    pub fn concat_mut(&mut self, other: &AttributeData) -> Result<()> {
        if self.data_type == other.data_type {
            self.data.extend(other.data.iter());
            self.count += other.count;
            Ok(())
        } else {
            Err(anyhow!(
                "Cannot concat attribute {:?} and {:?}",
                self.data_type,
                other.data_type
            ))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Attribute {
    data: AttributeData,
    buffer: WebGlBuffer,
}

impl Attribute {
    pub fn new_with_data(
        context: &WebGl2RenderingContext,
        data: AttributeData,
    ) -> Result<Attribute> {
        let buffer = gl::create_buffer(context)?;

        let attribute = Attribute { data, buffer };
        attribute.upload_data(context);
        Ok(attribute)
    }

    pub fn count(&self) -> i32 {
        self.data.count
    }

    fn upload_data(&self, context: &WebGl2RenderingContext) {
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer));
        self.data.buffer_data(context);
    }

    pub fn associate_variable(
        &self,
        context: &WebGl2RenderingContext,
        program: &WebGlProgram,
        variable: &str,
    ) {
        if let Some(location) = gl::get_attrib_location(context, program, variable) {
            context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&self.buffer));
            self.data.vertex_attrib_pointer(context, location);
            context.enable_vertex_attrib_array(location);
        }
    }

    pub fn apply_matrix(&mut self, context: &WebGl2RenderingContext, matrix: &Mat4) {
        self.data.apply_matrix(matrix);
        self.upload_data(context);
    }

    pub fn apply_matrix3(&mut self, context: &WebGl2RenderingContext, matrix: &Mat3) {
        self.data.apply_matrix3(matrix);
        self.upload_data(context);
    }

    pub fn concat_mut(
        &mut self,
        context: &WebGl2RenderingContext,
        other: &Attribute,
    ) -> Result<()> {
        self.concat_data_mut(context, &other.data)
    }

    pub fn concat_data_mut(
        &mut self,
        context: &WebGl2RenderingContext,
        data: &AttributeData,
    ) -> Result<()> {
        self.data.concat_mut(data)?;
        self.upload_data(context);
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn extend_by_vec3_works() {
        let mut vector = vec![1.0, 2.0];
        let vec3 = glm::vec3(3.0, 4.0, 5.0);
        vector.extend(&vec3);
        assert_eq!(vector, vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    }
}
