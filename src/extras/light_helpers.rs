use std::rc::Rc;

use anyhow::Result;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::{Geometry, TypedGeometry},
    base::{color, convert::FromWithContext, util::shared_ref},
    core::{material::Material, mesh::Mesh},
    geometry::parametric::Sphere,
    legacy::light::Light,
    material::basic::{BasicMaterial, SurfaceMaterial},
};

use super::grid_helper::GridHelper;

pub struct DirectionalLightHelper {
    pub size: f32,
    pub divisions: u16,
}

impl Default for DirectionalLightHelper {
    fn default() -> Self {
        Self {
            size: 1.0,
            divisions: 4,
        }
    }
}

impl DirectionalLightHelper {
    pub fn create_mesh(self, context: &WebGl2RenderingContext, light: &Light) -> Result<Mesh> {
        assert!(light.is_directional());
        let color = light.color;
        let grid_helper = GridHelper {
            size: self.size,
            divisions: self.divisions,
            grid_color: color,
            center_color: color::white(),
            ..Default::default()
        };
        let mut typed_geometry = TypedGeometry::try_from(grid_helper)?;
        let append = TypedGeometry::new(
            vec![glm::vec3(0.0, 0.0, 0.0), glm::vec3(0.0, 0.0, -10.0)],
            None,
            None,
            Some(vec![color, color]),
        )?;
        typed_geometry.concat_mut(&append);
        let material = Rc::new(Material::from_with_context(context, grid_helper)?);
        Geometry::from_with_context(context, typed_geometry)?.create_mesh(context, material)
    }
}

pub struct PointLightHelper {
    pub size: f32,
}

impl Default for PointLightHelper {
    fn default() -> Self {
        Self { size: 0.1 }
    }
}

impl PointLightHelper {
    pub fn create_mesh(self, context: &WebGl2RenderingContext, light: &Light) -> Result<Mesh> {
        let color = light.color;
        let geometry = Geometry::from_with_context(
            context,
            Sphere {
                radius: self.size,
                radius_segments: 4,
                height_segments: 2,
            },
        )?;
        let material = Rc::new(Material::from_with_context(
            context,
            shared_ref::strong(SurfaceMaterial {
                basic: BasicMaterial {
                    base_color: color,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )?);
        geometry.create_mesh(context, material)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test {
        value: Rc<i32>,
    }

    impl Test {
        fn new(a: Rc<i32>) -> Self {
            Self { value: a }
        }

        fn value(&self) -> &i32 {
            &self.value
        }

        fn value_mut(&mut self) -> Option<&mut i32> {
            Rc::get_mut(&mut self.value)
        }
    }

    #[test]
    fn rc_get_mut_works() {
        let mut a = Test::new(Rc::new(1));
        assert_eq!(a.value(), &1);
        *a.value_mut().unwrap() = 2;
        assert_eq!(a.value(), &2);
        *a.value_mut().unwrap() = 3;
        assert_eq!(a.value(), &3);
    }
}
