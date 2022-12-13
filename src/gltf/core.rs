use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

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
                &data,
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
            buffer_view.bind(context);
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
            indices.set_indices(context);
        }
        context.bind_vertex_array(None);
        BufferView::unbind(context, self.indices.is_some());
    }

    fn render(&self, context: &WebGl2RenderingContext, model_matrix: &Mat4) {
        self.program.use_program(context);
        model_matrix.update_uniform(context, "u_ModelMatrix", &self.program);
        self.draw(context);
    }

    fn draw(&self, context: &WebGl2RenderingContext) {
        context.bind_vertex_array(Some(&self.vertex_array));
        if let Some(indices) = &self.indices {
            context.draw_elements_with_i32(self.mode, self.count, indices.component_type, 0)
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
    me: Weak<Node>,
    children: RefCell<Vec<Rc<Node>>>,
    local_transform: RefCell<Mat4>,
    mesh: Option<Rc<Mesh>>,
    parent: RefCell<Weak<Node>>,
}

impl Node {
    pub fn new(local_transform: Mat4, mesh: Option<Rc<Mesh>>) -> Rc<Self> {
        Rc::new_cyclic(|me| Self {
            me: Weak::clone(&me),
            children: RefCell::new(vec![]),
            local_transform: RefCell::new(local_transform),
            mesh,
            parent: RefCell::new(Weak::new()),
        })
    }

    pub fn render(&self, context: &WebGl2RenderingContext) {
        if let Some(mesh) = &self.mesh {
            mesh.render(context, &self.global_transform());
        }
    }

    pub fn add_child(&self, node: Rc<Node>) {
        *node.parent.borrow_mut() = Weak::clone(&self.me);
        self.children.borrow_mut().push(node);
    }

    pub fn global_transform(&self) -> Mat4 {
        if let Some(parent) = self.parent.borrow().upgrade() {
            parent.global_transform() * *self.local_transform.borrow()
        } else {
            *self.local_transform.borrow()
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
