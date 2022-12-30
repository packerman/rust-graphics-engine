use web_sys::WebGl2RenderingContext;

use crate::{
    base::color::{self, Color},
    core::{
        material::GenericMaterial,
        program::{Program, UpdateProgramUniforms, UpdateUniform},
    },
};

#[derive(Debug)]
pub struct BasicMaterial {
    pub base_color: Color,
    pub use_vertex_colors: bool,
}

impl Default for BasicMaterial {
    fn default() -> Self {
        Self {
            base_color: color::white(),
            use_vertex_colors: false,
        }
    }
}

impl GenericMaterial for BasicMaterial {
    fn vertex_shader(&self) -> &str {
        include_str!("vertex.glsl")
    }

    fn fragment_shader(&self) -> &str {
        include_str!("fragment.glsl")
    }
}

impl UpdateProgramUniforms for BasicMaterial {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.base_color
            .update_uniform(context, "baseColor", program);
        self.use_vertex_colors
            .update_uniform(context, "useVertexColors", program);
    }
}

#[derive(Debug)]
pub struct PointMaterial {
    pub basic: BasicMaterial,
    pub point_size: f32,
}

impl Default for PointMaterial {
    fn default() -> Self {
        Self {
            basic: Default::default(),
            point_size: 8.0,
        }
    }
}

impl GenericMaterial for PointMaterial {
    fn vertex_shader(&self) -> &str {
        self.basic.vertex_shader()
    }

    fn fragment_shader(&self) -> &str {
        self.basic.fragment_shader()
    }

    fn preferred_mode(&self) -> Option<u32> {
        Some(WebGl2RenderingContext::POINTS)
    }
}

impl UpdateProgramUniforms for PointMaterial {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.basic.update_program_uniforms(context, program);
        self.point_size
            .update_uniform(context, "pointSize", program);
    }
}

#[derive(Debug)]
pub enum LineType {
    Connected,
    Segments,
}

#[derive(Debug)]
pub struct LineMaterial {
    pub basic: BasicMaterial,
    pub line_width: f32,
    pub line_type: LineType,
}

impl Default for LineMaterial {
    fn default() -> Self {
        Self {
            basic: Default::default(),
            line_width: 1.0,
            line_type: LineType::Connected,
        }
    }
}

impl UpdateProgramUniforms for LineMaterial {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.basic.update_program_uniforms(context, program);
        context.line_width(self.line_width);
    }
}

impl GenericMaterial for LineMaterial {
    fn vertex_shader(&self) -> &str {
        self.basic.vertex_shader()
    }

    fn fragment_shader(&self) -> &str {
        self.basic.fragment_shader()
    }

    fn preferred_mode(&self) -> Option<u32> {
        Some(match self.line_type {
            LineType::Connected => WebGl2RenderingContext::LINE_STRIP,
            LineType::Segments => WebGl2RenderingContext::LINES,
        })
    }
}

#[derive(Debug)]
pub struct SurfaceMaterial {
    pub basic: BasicMaterial,
    pub double_side: bool,
}

impl Default for SurfaceMaterial {
    fn default() -> Self {
        Self {
            basic: BasicMaterial::default(),
            double_side: false,
        }
    }
}

impl UpdateProgramUniforms for SurfaceMaterial {
    fn update_program_uniforms(&self, context: &WebGl2RenderingContext, program: &Program) {
        self.basic.update_program_uniforms(context, program);
    }
}

impl GenericMaterial for SurfaceMaterial {
    fn vertex_shader(&self) -> &str {
        self.basic.vertex_shader()
    }

    fn fragment_shader(&self) -> &str {
        self.basic.fragment_shader()
    }

    fn preferred_mode(&self) -> Option<u32> {
        Some(WebGl2RenderingContext::TRIANGLES)
    }

    fn double_sided(&self) -> bool {
        self.double_side
    }
}
