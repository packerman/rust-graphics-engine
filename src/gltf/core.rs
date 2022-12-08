use std::{collections::HashMap, rc::Rc};

use anyhow::{anyhow, Result};
use js_sys::{ArrayBuffer, DataView, Object, SharedArrayBuffer};
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlVertexArrayObject};

use crate::core::{gl, material::Material};

#[derive(Debug, Clone)]
pub struct Buffer {
    array_buffer: ArrayBuffer,
}

impl Buffer {
    pub fn new(array_buffer: ArrayBuffer) -> Self {
        Self { array_buffer }
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
    pub fn new(
        context: &WebGl2RenderingContext,
        buffer: Rc<Buffer>,
        byte_offset: u32,
        byte_length: u32,
        byte_stride: Option<i32>,
        target: Option<u32>,
    ) -> Result<Self> {
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

    pub fn unbind(&self, context: &WebGl2RenderingContext) {
        if let Some(target) = self.target {
            context.bind_buffer(target, None);
        }
    }

    pub fn copy_data(&self, context: &WebGl2RenderingContext) {
        if let Some(target) = self.target {
            context.bind_buffer(target, self.gl_buffer.as_ref());
            self.data_buffer
                .copy_data(context, target, self.byte_offset, self.byte_length);
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
    min: Option<Vec<f64>>,
    max: Option<Vec<f64>>,
    pub normalized: bool,
}

impl Accessor {
    pub fn new(
        buffer_view: Option<Rc<BufferView>>,
        byte_offset: i32,
        component_type: u32,
        count: i32,
        size: i32,
        min: Option<Vec<f64>>,
        max: Option<Vec<f64>>,
        normalized: bool,
    ) -> Self {
        Self {
            buffer_view,
            byte_offset,
            component_type,
            count,
            size,
            min,
            max,
            normalized,
        }
    }

    pub fn copy_data(&self, context: &WebGl2RenderingContext, location: u32) {
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
    material: Rc<Material>,
    mode: u32,
    count: i32,
}

impl Primitive {
    pub fn new(
        context: &WebGl2RenderingContext,
        attributes: HashMap<String, Rc<Accessor>>,
        material: Rc<Material>,
        mode: u32,
    ) -> Result<Self> {
        let vertex_array = gl::create_vertex_array(context)?;
        let count = Self::get_count(&attributes)?;
        let me = Self {
            vertex_array,
            attributes,
            material,
            mode,
            count,
        };
        me.copy_data(context);
        Ok(me)
    }

    pub fn copy_data(&self, context: &WebGl2RenderingContext) {
        self.material.use_program(context);
        context.bind_vertex_array(Some(&self.vertex_array));
        for (attribute, accessor) in self.attributes.iter() {
            let attribute = format!("a_{}", attribute.to_lowercase());
            if let Some(location) =
                gl::get_attrib_location(context, self.material.program(), &attribute)
            {
                accessor.copy_data(context, location);
            }
        }
        context.bind_vertex_array(None);
    }

    fn render(&self, context: &WebGl2RenderingContext) {
        self.material.use_program(context);
        context.bind_vertex_array(Some(&self.vertex_array));
        self.material.upload_uniform_data(context);
        context.draw_arrays(self.mode, 0, self.count);
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

    pub fn render(&self, context: &WebGl2RenderingContext) {
        for primitive in self.primitives.iter() {
            primitive.render(context);
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    mesh: Option<Rc<Mesh>>,
}

impl Node {
    pub fn new(mesh: Option<Rc<Mesh>>) -> Self {
        Self { mesh }
    }

    pub fn render(&self, context: &WebGl2RenderingContext) {
        if let Some(mesh) = &self.mesh {
            mesh.render(context);
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
