use std::rc::Rc;

use anyhow::Result;
use glm::{Mat4, Vec3};
use web_sys::WebGl2RenderingContext;

use crate::{
    base::math::resolution::Resolution,
    core::texture::TextureUnit,
    legacy::{
        camera::{Camera, Ortographic},
        material::Material,
        node::Node,
        render_target::RenderTarget,
        uniform::{
            data::{CreateDataFromValue, Data, Sampler2D},
            Uniform, UpdateUniform,
        },
    },
    material,
};

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

impl From<CameraBounds> for Ortographic {
    fn from(camera_bounds: CameraBounds) -> Self {
        Ortographic {
            left: camera_bounds.min.x,
            right: camera_bounds.max.x,
            bottom: camera_bounds.min.y,
            top: camera_bounds.max.y,
            near: camera_bounds.min.z,
            far: camera_bounds.max.z,
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

#[derive(Debug, Clone)]
pub struct Shadow {
    light_source: Rc<Node>,
    resolution: Resolution,
    options: ShadowOptions,
    camera: Rc<Node>,
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
        light_source: Rc<Node>,
        resolution: Resolution,
        texture_unit: TextureUnit,
        options: ShadowOptions,
    ) -> Result<Self> {
        assert!(Self::is_directional_light(&light_source));
        let camera = Node::new_camera(Camera::new_ortographic(options.camera_bounds.into()));
        light_source.add_child(&camera);

        let render_target = RenderTarget::initialize(context, resolution)?;

        let material = material::depth::create(context)?;

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

    pub fn update(&self) {
        self.camera.update();
        if let Some(camera) = self.camera.as_camera() {
            let camera = camera.borrow();
            self.material.set_view_matrix(*camera.view_matrix());
            self.material
                .set_projection_matrix(camera.projection_matrix());
        }
    }

    pub fn bind(&self, context: &WebGl2RenderingContext) {
        self.render_target.bind(context);
        context.viewport(0, 0, self.resolution.width, self.resolution.height);
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    fn light_direction(&self) -> Vec3 {
        self.light_source
            .as_light()
            .and_then(|light| light.borrow().as_directional().copied())
            .unwrap()
    }

    fn projection_matrix(&self) -> Mat4 {
        self.camera
            .as_camera()
            .map(|camera| camera.borrow().projection_matrix())
            .unwrap()
    }

    fn view_matrix(&self) -> Mat4 {
        self.camera
            .as_camera()
            .map(|camera| *camera.borrow().view_matrix())
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

    fn is_directional_light(node: &Node) -> bool {
        node.as_light()
            .map_or(false, |light| light.borrow().is_directional())
    }
}

impl CreateDataFromValue for Shadow {
    fn create_data(&self) -> Data {
        Data::from([
            (Self::LIGHT_DIRECTION_MEMBER, self.light_direction().into()),
            (
                Self::PROJECTION_MATRIX_MEMBER,
                self.projection_matrix().into(),
            ),
            (Self::VIEW_MATRIX_MEMBER, self.view_matrix().into()),
            (Self::DEPTH_TEXTURE_MEMBER, self.get_sampler().into()),
            (Self::STRENGTH_MEMBER, self.strength().into()),
            (Self::BIAS_MEMBER, self.bias().into()),
        ])
    }
}

impl UpdateUniform for Shadow {
    fn update_uniform(&self, uniform: &Uniform) {
        if let Some(uniform) = uniform.as_struct() {
            uniform.set_vec3_member(Self::LIGHT_DIRECTION_MEMBER, self.light_direction());
            uniform.set_mat4_member(Self::PROJECTION_MATRIX_MEMBER, self.projection_matrix());
            uniform.set_mat4_member(Self::VIEW_MATRIX_MEMBER, self.view_matrix());
            uniform.set_float_member(Self::STRENGTH_MEMBER, self.strength());
            uniform.set_float_member(Self::BIAS_MEMBER, self.bias());
        }
    }
}
