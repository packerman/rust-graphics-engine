use std::{collections::HashMap, rc::Rc};

use anyhow::{anyhow, Result};
use glm::Mat4;
use web_sys::{WebGl2RenderingContext, WebGlVertexArrayObject};

use crate::{
    core::gl,
    gltf::{program::UpdateUniform, validate},
};

use super::{
    material::Material,
    scene::Node,
    storage::{Accessor, BufferView},
};

#[derive(Debug, Clone)]
pub struct Primitive {
    vertex_array: WebGlVertexArrayObject,
    attributes: HashMap<String, Rc<Accessor>>,
    indices: Option<Rc<Accessor>>,
    material: Rc<Material>,
    mode: u32,
    vertex_count: i32,
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
        material: Rc<Material>,
        mode: u32,
    ) -> Result<Self> {
        validate::contains(&mode, &Self::MODES, |value| {
            anyhow!("Unknown mode: {}", value)
        })?;
        let vertex_array = gl::create_vertex_array(context)?;
        let count = if let Some(accessor) = indices.as_ref() {
            accessor.count
        } else {
            Self::get_vertex_count(&attributes)?
        };
        let me = Self {
            vertex_array,
            attributes,
            indices,
            material,
            mode,
            vertex_count: count,
        };
        me.set_vertex_array(context);
        Ok(me)
    }

    pub fn set_vertex_array(&self, context: &WebGl2RenderingContext) {
        let program = self.material.program();
        program.use_program(context);
        context.bind_vertex_array(Some(&self.vertex_array));
        for (attribute, accessor) in self.attributes.iter() {
            let attribute = format!("a_{}", attribute.to_lowercase());
            if let Some(location) = program.get_attribute_location(&attribute) {
                accessor.set_vertex_attribute(context, *location);
            }
        }
        if let Some(accessor) = &self.indices {
            accessor.set_indices(context);
        }
        context.bind_vertex_array(None);
        BufferView::unbind(context, self.indices.is_some());
    }

    fn render(&self, context: &WebGl2RenderingContext, node: &Node, view_projection_matrix: &Mat4) {
        let program = self.material.program();
        program.use_program(context);
        self.material.update(context);
        view_projection_matrix.update_uniform(context, "u_ViewProjectionMatrix", program);
        node.global_transform()
            .update_uniform(context, "u_ModelMatrix", program);
        self.draw(context);
    }

    fn draw(&self, context: &WebGl2RenderingContext) {
        context.bind_vertex_array(Some(&self.vertex_array));
        if let Some(indices) = &self.indices {
            context.draw_elements_with_i32(self.mode, self.vertex_count, indices.component_type, 0)
        } else {
            context.draw_arrays(self.mode, 0, self.vertex_count);
        }
        context.bind_vertex_array(None);
    }

    fn get_vertex_count(atttributes: &HashMap<String, Rc<Accessor>>) -> Result<i32> {
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

    pub fn render(
        &self,
        context: &WebGl2RenderingContext,
        node: &Node,
        view_projection_matrix: &Mat4,
    ) {
        for primitive in self.primitives.iter() {
            primitive.render(context, node, view_projection_matrix);
        }
    }
}
