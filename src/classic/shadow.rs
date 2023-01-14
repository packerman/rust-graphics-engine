use std::rc::Rc;

use anyhow::Result;
use glm::{Mat4, Vec3};
use web_sys::WebGl2RenderingContext;

use crate::{
    base::{
        convert::FromWithContext,
        math::resolution::Resolution,
        util::{
            level::Level,
            shared_ref::{self, SharedRef},
        },
    },
    classic::{render_target::RenderTarget, texture::Sampler2D},
    core::{
        camera::{Camera, Orthographic},
        material::Material,
        node::Node,
        program::{self, Program, UpdateUniform},
        texture::TextureUnit,
    },
    material::depth::DepthMaterial,
};

use super::light::LightNode;

#[derive(Debug, Clone, Copy)]
pub struct CameraBounds {
    pub min: Vec3,
    pub max: Vec3,
}

impl Default for CameraBounds {
    fn default() -> Self {
        Self {
            min: glm::vec3(-5.0, -5.0, 0.0),
            max: glm::vec3(5.0, 5.0, 20.0),
        }
    }
}

impl From<CameraBounds> for Orthographic {
    fn from(camera_bounds: CameraBounds) -> Self {
        Orthographic {
            x_left: camera_bounds.min.x,
            x_right: camera_bounds.max.x,
            y_bottom: camera_bounds.min.y,
            y_top: camera_bounds.max.y,
            z_near: camera_bounds.min.z,
            z_far: camera_bounds.max.z,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ShadowOptions {
    pub strength: f32,
    pub camera_bounds: CameraBounds,
    pub bias: f32,
}

impl Default for ShadowOptions {
    fn default() -> Self {
        Self {
            strength: 0.5,
            camera_bounds: Default::default(),
            bias: 0.01,
        }
    }
}

#[derive(Debug)]
pub struct Shadow {
    light_source: LightNode,
    resolution: Resolution,
    options: ShadowOptions,
    camera: SharedRef<Node>,
    render_target: RenderTarget,
    texture_unit: TextureUnit,
    material: Rc<Material>,
}

impl Shadow {
    const LIGHT_DIRECTION_MEMBER: &str = "lightDirection";
    const PROJECTION_MATRIX_MEMBER: &str = "projectionMatrix";
    const VIEW_MATRIX_MEMBER: &str = "viewMatrix";
    const DEPTH_TEXTURE_MEMBER: &str = "depthTexture";
    const STRENGTH_MEMBER: &str = "strength";
    const BIAS_MEMBER: &str = "bias";

    pub fn initialize(
        context: &WebGl2RenderingContext,
        light_source: LightNode,
        resolution: Resolution,
        texture_unit: TextureUnit,
        options: ShadowOptions,
    ) -> Result<Self> {
        assert!(light_source.is_directional());
        let camera = Node::new_with_camera(Camera::new(Orthographic::from(options.camera_bounds)));
        light_source.add_child(Rc::clone(&camera));
        let render_target = RenderTarget::initialize(context, resolution)?;
        let material = <Rc<Material>>::from_with_context(context, shared_ref::new(DepthMaterial))?;
        Ok(Self {
            light_source,
            resolution,
            options,
            camera,
            render_target,
            texture_unit,
            material,
        })
    }

    // TODO check whether this is needed somehow
    // pub fn update(&self) {
    //     self.camera.update();
    //     if let Some(camera) = self.camera.as_camera() {
    //         let camera = camera.borrow();
    //         self.material.set_view_matrix(*camera.view_matrix());
    //         self.material
    //             .set_projection_matrix(camera.projection_matrix());
    //     }
    // }

    pub fn bind(&self, context: &WebGl2RenderingContext) {
        self.render_target.bind(context);
        context.viewport(0, 0, self.resolution.width, self.resolution.height);
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    fn light_direction(&self) -> Vec3 {
        self.light_source.as_directional().unwrap()
    }

    fn projection_matrix(&self) -> Mat4 {
        self.camera
            .borrow()
            .camera()
            .map(|camera| camera.borrow().projection_matrix())
            .unwrap()
    }

    fn view_matrix(&self) -> Mat4 {
        self.camera
            .borrow()
            .camera()
            .map(|camera| camera.borrow().view_matrix())
            .unwrap()
    }

    fn get_sampler(&self) -> Sampler2D {
        Sampler2D::new(self.render_target.texture(), self.texture_unit)
    }

    fn strength(&self) -> f32 {
        self.options.strength
    }

    fn bias(&self) -> f32 {
        self.options.bias
    }
}

impl UpdateUniform for Shadow {
    fn update_uniform_with_level(
        &self,
        context: &WebGl2RenderingContext,
        name: &str,
        program: &Program,
        level: Level,
    ) {
        self.light_direction().update_uniform_with_level(
            context,
            &program::join_name(name, Self::LIGHT_DIRECTION_MEMBER),
            program,
            level,
        );
        self.projection_matrix().update_uniform_with_level(
            context,
            &program::join_name(name, Self::PROJECTION_MATRIX_MEMBER),
            program,
            level,
        );
        self.view_matrix().update_uniform_with_level(
            context,
            &program::join_name(name, Self::VIEW_MATRIX_MEMBER),
            program,
            level,
        );
        self.get_sampler().update_uniform_with_level(
            context,
            &program::join_name(name, Self::DEPTH_TEXTURE_MEMBER),
            program,
            level,
        );
        self.strength().update_uniform_with_level(
            context,
            &program::join_name(name, Self::STRENGTH_MEMBER),
            program,
            level,
        );
        self.bias().update_uniform_with_level(
            context,
            &program::join_name(name, Self::BIAS_MEMBER),
            program,
            level,
        );
    }
}
