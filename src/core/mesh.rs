use std::{collections::HashMap, rc::Rc};

use anyhow::{anyhow, Result};
use glm::Mat4;
use web_sys::{WebGl2RenderingContext, WebGlVertexArrayObject};

use crate::base::{
    gl,
    util::{level::Level, validate},
};

use super::{
    accessor::Accessor,
    buffer_view::BufferView,
    material::Material,
    node::Node,
    program::{UpdateProgramUniforms, UpdateUniform},
};

#[derive(Debug, Clone)]
pub struct Mesh {
    primitives: Vec<Primitive>,
    #[allow(dead_code)]
    name: Option<String>,
}

impl Mesh {
    pub fn new(primitives: Vec<Primitive>, name: Option<String>) -> Self {
        Self { primitives, name }
    }

    pub fn primitive(
        context: &WebGl2RenderingContext,
        attributes: HashMap<String, Rc<Accessor>>,
        indices: Option<Rc<Accessor>>,
        material: Rc<Material>,
        mode: u32,
    ) -> Result<Self> {
        let primitive = Primitive::new(context, attributes, indices, material, mode)?;
        Ok(Self::new(vec![primitive], None))
    }

    pub fn update_uniform<T>(
        &self,
        context: &WebGl2RenderingContext,
        name: &str,
        value: &T,
        level: Level,
    ) where
        T: UpdateUniform,
    {
        for primitive in &self.primitives {
            primitive.update_uniform(context, name, value, level);
        }
    }

    pub fn render(
        &self,
        context: &WebGl2RenderingContext,
        node: &Node,
        view_projection_matrix: &Mat4,
        global_uniform_updater: &dyn UpdateProgramUniforms,
    ) {
        for primitive in self.primitives.iter() {
            primitive.render(
                context,
                node,
                view_projection_matrix,
                global_uniform_updater,
            );
        }
    }

    pub fn render_triangle_based(
        &self,
        context: &WebGl2RenderingContext,
        node: &Node,
        global_uniform_updater: &dyn UpdateProgramUniforms,
        material: &Material,
    ) {
        for primitive in self.primitives.iter() {
            if primitive.is_triangle_based() {
                primitive.render_with_material(context, node, global_uniform_updater, material)
            }
        }
    }

    pub fn has_uniform(&self, name: &str) -> bool {
        self.primitives
            .iter()
            .any(|primitive| primitive.has_uniform(name))
    }
}

#[derive(Debug, Clone)]
pub struct Primitive {
    vertex_array: WebGlVertexArrayObject,
    attributes: HashMap<String, Rc<Accessor>>,
    indices: Option<Rc<Accessor>>,
    material: Rc<Material>,
    mode: u32,
    vertex_count: i32,
}

pub const POSITION_ATTRIBUTE: &str = "POSITION";
pub const NORMAL_ATTRIBUTE: &str = "NORMAL";
pub const TEXCOORD_0_ATTRIBUTE: &str = "TEXCOORD_0";
pub const COLOR_0_ATTRIBUTE: &str = "COLOR_0";

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
        validate::assert(attributes.contains_key(POSITION_ATTRIBUTE), || {
            anyhow!("Missing attribute {}", POSITION_ATTRIBUTE)
        })?;
        let vertex_array = gl::create_vertex_array(context)?;
        let effective_mode = material.preferred_mode().unwrap_or(mode);
        let vertex_count = Self::get_vertex_count(&attributes)?;
        let me = Self {
            vertex_array,
            attributes,
            indices,
            material,
            mode: effective_mode,
            vertex_count,
        };
        me.set_vertex_array(context);
        Ok(me)
    }

    pub fn set_vertex_array(&self, context: &WebGl2RenderingContext) {
        let program = self.material.program();
        program.use_program(context);
        context.bind_vertex_array(Some(&self.vertex_array));
        for (attribute, accessor) in self.attributes.iter() {
            let attribute = Self::attribute_to_variable_name(attribute);
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

    pub fn has_attribute(&self, name: &str) -> bool {
        self.attributes.contains_key(name)
    }

    pub fn has_uniform(&self, name: &str) -> bool {
        self.material.has_uniform(name)
    }

    pub fn update_uniform<T>(
        &self,
        context: &WebGl2RenderingContext,
        name: &str,
        value: &T,
        level: Level,
    ) where
        T: UpdateUniform,
    {
        self.material.update_uniform(context, name, value, level);
    }

    fn render(
        &self,
        context: &WebGl2RenderingContext,
        node: &Node,
        view_projection_matrix: &Mat4,
        global_uniform_updater: &dyn UpdateProgramUniforms,
    ) {
        self.material.use_program(context);
        self.material.update_uniform(
            context,
            "u_ViewProjectionMatrix",
            view_projection_matrix,
            Level::Ignore,
        );
        self.render_generic(context, node, global_uniform_updater, &self.material)
    }

    fn render_with_material(
        &self,
        context: &WebGl2RenderingContext,
        node: &Node,
        global_uniform_updater: &dyn UpdateProgramUniforms,
        material: &Material,
    ) {
        material.use_program(context);
        self.render_generic(context, node, global_uniform_updater, material)
    }

    fn render_generic(
        &self,
        context: &WebGl2RenderingContext,
        node: &Node,
        global_uniform_updater: &dyn UpdateProgramUniforms,
        material: &Material,
    ) {
        let program = material.program();
        global_uniform_updater.update_program_uniforms(context, program);
        material.update(context);
        node.global_transform()
            .update_uniform(context, "u_ModelMatrix", program);
        node.normal_transform()
            .update_uniform(context, "u_NormalMatrix", program);
        self.has_attribute(COLOR_0_ATTRIBUTE)
            .update_uniform(context, "u_UseColor_0", program);
        self.draw(context);
    }

    fn draw(&self, context: &WebGl2RenderingContext) {
        context.bind_vertex_array(Some(&self.vertex_array));
        if let Some(indices) = &self.indices {
            context.draw_elements_with_i32(self.mode, indices.count, indices.component_type, 0);
        } else {
            context.draw_arrays(self.mode, 0, self.vertex_count);
        }
        context.bind_vertex_array(None);
    }

    fn is_triangle_based(&self) -> bool {
        self.mode == WebGl2RenderingContext::TRIANGLES
            || self.mode == WebGl2RenderingContext::TRIANGLE_STRIP
            || self.mode == WebGl2RenderingContext::TRIANGLE_STRIP
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

    fn attribute_to_variable_name(attribute: &str) -> String {
        format!("a_{}", attribute.to_lowercase())
    }
}
