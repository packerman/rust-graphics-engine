use std::{collections::HashMap, rc::Rc};

use anyhow::{anyhow, Result};
use glm::Mat4;
use js_sys::{ArrayBuffer, DataView};
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlVertexArrayObject};

use crate::core::gl;

use super::{
    program::{Program, UpdateUniform},
    validate,
};

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

    pub fn copy_data(
        &self,
        context: &WebGl2RenderingContext,
        target: u32,
        byte_offset: u32,
        byte_length: u32,
    ) {
        let data_view = DataView::new(
            &self.array_buffer,
            byte_offset as usize,
            byte_length as usize,
        );
        context.buffer_data_with_array_buffer_view(
            target,
            &data_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
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
        let me = Self {
            gl_buffer: if target.is_some() {
                Some(gl::create_buffer(context)?)
            } else {
                None
            },
            data_buffer: buffer,
            byte_offset,
            byte_length,
            target,
            byte_stride: byte_stride.unwrap_or_default(),
        };
        me.copy_data(context);
        Ok(me)
    }

    pub fn bind(&self, context: &WebGl2RenderingContext) {
        if let Some(target) = self.target {
            context.bind_buffer(target, self.gl_buffer.as_ref());
        }
    }

    pub fn copy_data(&self, context: &WebGl2RenderingContext) {
        if let Some(target) = self.target {
            context.bind_buffer(target, self.gl_buffer.as_ref());
            self.data_buffer
                .copy_data(context, target, self.byte_offset, self.byte_length);
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
    component_type: u32,
    count: i32,
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
            buffer_view.bind(context);
            context.vertex_attrib_pointer_with_i32(
                location,
                self.size,
                self.component_type,
                self.normalized,
                buffer_view.byte_stride,
                self.byte_offset,
            );
            context.enable_vertex_attrib_array(location);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Primitive {
    vertex_array: WebGlVertexArrayObject,
    attributes: HashMap<String, Rc<Accessor>>,
    indices: Option<Rc<Accessor>>,
    program: Rc<Program>,
    mode: u32,
    count: i32,
}

impl Primitive {
    const MODES: [u32; 7] = [
        WebGl2RenderingContext::POINTS,
        WebGl2RenderingContext::LINES,
        WebGl2RenderingContext::LINE_LOOP,
        WebGl2RenderingContext::LINE_STRIP,
        WebGl2RenderingContext::TRIANGLES,
        WebGl2RenderingContext::TRIANGLE_STRIP,
        WebGl2RenderingContext::TRIANGLE_STRIP,
    ];

    pub fn new(
        context: &WebGl2RenderingContext,
        attributes: HashMap<String, Rc<Accessor>>,
        indices: Option<Rc<Accessor>>,
        program: Rc<Program>,
        mode: u32,
    ) -> Result<Self> {
        validate::contains(&mode, &Self::MODES, |value| {
            anyhow!("Unknown mode: {}", value)
        })?;
        let vertex_array = gl::create_vertex_array(context)?;
        let count = Self::get_count(&attributes)?;
        let me = Self {
            vertex_array,
            attributes,
            indices,
            program,
            mode,
            count,
        };
        me.set_vertex_array(context);
        Ok(me)
    }

    pub fn set_vertex_array(&self, context: &WebGl2RenderingContext) {
        self.program.use_program(context);
        context.bind_vertex_array(Some(&self.vertex_array));
        for (attribute, accessor) in self.attributes.iter() {
            let attribute = format!("a_{}", attribute.to_lowercase());
            if let Some(location) = self.program.get_attribute_location(&attribute) {
                accessor.set_vertex_attribute(context, *location);
            }
        }
        if let Some(indices) = &self.indices {
            if let Some(buffer_view) = &indices.buffer_view {
                buffer_view.bind(context);
            }
        }
        context.bind_vertex_array(None);
        BufferView::unbind(context, self.indices.is_some());
    }

    fn render(&self, context: &WebGl2RenderingContext, model_matrix: &Mat4) {
        self.program.use_program(context);
        model_matrix.update_uniform(context, "u_ModelMatrix", &self.program);
        context.bind_vertex_array(Some(&self.vertex_array));
        if let Some(indices) = &self.indices {
            context.draw_elements_with_i32(
                self.mode,
                self.count,
                indices.component_type,
                indices.byte_offset,
            )
        } else {
            context.draw_arrays(self.mode, 0, self.count);
        }
        context.bind_vertex_array(None);
    }

    fn get_count(atttributes: &HashMap<String, Rc<Accessor>>) -> Result<i32> {
        let counts: Vec<_> = atttributes
            .values()
            .map(|accessor| accessor.count)
            .collect();
        if counts.is_empty() {
            Err(anyhow!("Attributes map is empty"))
        } else {
            let count = counts[0];
            if counts.into_iter().all(|value| value == count) {
                Ok(count)
            } else {
                Err(anyhow!("All accessors count have to be equal"))
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mesh {
    primitives: Vec<Primitive>,
}

impl Mesh {
    pub fn new(primitives: Vec<Primitive>) -> Self {
        Self { primitives }
    }

    pub fn render(&self, context: &WebGl2RenderingContext, model_matrix: &Mat4) {
        for primitive in self.primitives.iter() {
            primitive.render(context, model_matrix);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    matrix: Mat4,
    mesh: Option<Rc<Mesh>>,
}

impl Node {
    pub fn new(matrix: Mat4, mesh: Option<Rc<Mesh>>) -> Self {
        Self { matrix, mesh }
    }

    pub fn render(&self, context: &WebGl2RenderingContext) {
        if let Some(mesh) = &self.mesh {
            mesh.render(context, &self.matrix);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Scene {
    nodes: Vec<Rc<Node>>,
}

impl Scene {
    pub fn new(nodes: Vec<Rc<Node>>) -> Self {
        Self { nodes }
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        for node in self.nodes.iter() {
            node.render(context);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Root {
    scenes: Vec<Scene>,
    scene: Option<usize>,
}

impl Root {
    pub fn new(scenes: Vec<Scene>, scene: Option<usize>) -> Self {
        Self { scenes, scene }
    }

    pub fn render(&self, context: &WebGl2RenderingContext) {
        if let Some(scene) = self.scene {
            self.scenes[scene].render(context);
        }
    }
}
