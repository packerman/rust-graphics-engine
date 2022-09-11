use std::{
    f32::consts::{FRAC_PI_2, TAU},
    ops::RangeInclusive,
};

use anyhow::Result;
use glm::Vec3;
use web_sys::WebGl2RenderingContext;

use crate::core::{
    attribute::AttributeData,
    color::Color,
    convert::FromWithContext,
    matrix::{self, Angle},
};

use super::{Geometry, Polygon};

struct ParametricSurface {
    u_range: RangeInclusive<f32>,
    u_resolution: u16,
    v_range: RangeInclusive<f32>,
    v_resolution: u16,
    function: Box<dyn Fn(f32, f32) -> Vec3>,
}

impl FromWithContext<WebGl2RenderingContext, ParametricSurface> for Geometry {
    fn from_with_context(
        context: &WebGl2RenderingContext,
        surface: ParametricSurface,
    ) -> Result<Self> {
        let mut position_data =
            Vec::with_capacity((6 * surface.u_resolution * surface.v_resolution).into());
        let mut color_data =
            Vec::with_capacity((6 * surface.u_resolution * surface.v_resolution).into());

        let u_delta =
            (surface.u_range.end() - surface.u_range.start()) / f32::from(surface.u_resolution);
        let v_delta =
            (surface.v_range.end() - surface.v_range.start()) / f32::from(surface.v_resolution);
        let mut positions = Vec::with_capacity((surface.u_resolution + 1).into());
        for u_index in 0..=surface.u_resolution {
            let mut vector = Vec::with_capacity((surface.v_resolution + 1).into());
            for v_index in 0..=surface.v_resolution {
                let u = surface.u_range.start() + f32::from(u_index) * u_delta;
                let v = surface.v_range.start() + f32::from(v_index) * v_delta;
                vector.push((surface.function)(u, v));
            }
            positions.push(vector);
        }
        let colors = [
            Color::red(),
            Color::lime(),
            Color::blue(),
            Color::aqua(),
            Color::fuchsia(),
            Color::yellow(),
        ];
        for x_index in 0..usize::from(surface.u_resolution) {
            for y_index in 0..usize::from(surface.v_resolution) {
                let p_a = positions[x_index][y_index];
                let p_b = positions[x_index + 1][y_index];
                let p_d = positions[x_index][y_index + 1];
                let p_c = positions[x_index + 1][y_index + 1];
                position_data.extend([p_a, p_b, p_c, p_a, p_c, p_d]);
                color_data.extend(colors);
            }
        }

        Geometry::from_with_context(
            context,
            [
                ("vertexPosition", AttributeData::from(&position_data)),
                ("vertexColor", AttributeData::from(&color_data)),
            ],
        )
    }
}

pub struct Plane {
    width: f32,
    height: f32,
    width_segments: u16,
    height_segments: u16,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
            width_segments: 8,
            height_segments: 8,
        }
    }
}

impl From<Plane> for ParametricSurface {
    fn from(plane: Plane) -> Self {
        ParametricSurface {
            u_range: (-plane.width / 2.0)..=(plane.width / 2.0),
            u_resolution: plane.width_segments,
            v_range: (-plane.height / 2.0)..=(plane.height / 2.0),
            v_resolution: plane.height_segments,
            function: Box::new(|u, v| glm::vec3(u, v, 0.0)),
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, Plane> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, plane: Plane) -> Result<Self> {
        Geometry::from_with_context(context, ParametricSurface::from(plane))
    }
}

pub struct Ellipsoid {
    width: f32,
    height: f32,
    depth: f32,
    radius_segments: u16,
    height_segments: u16,
}

impl Default for Ellipsoid {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
            depth: 1.0,
            radius_segments: 32,
            height_segments: 16,
        }
    }
}

impl From<Ellipsoid> for ParametricSurface {
    fn from(ellipsoid: Ellipsoid) -> Self {
        ParametricSurface {
            u_range: 0.0..=TAU,
            u_resolution: ellipsoid.radius_segments,
            v_range: -FRAC_PI_2..=FRAC_PI_2,
            v_resolution: ellipsoid.height_segments,
            function: Box::new(move |u, v| {
                glm::vec3(
                    ellipsoid.width / 2.0 * u.sin() * v.cos(),
                    ellipsoid.height / 2.0 * v.sin(),
                    ellipsoid.depth / 2.0 * u.cos() * v.cos(),
                )
            }),
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, Ellipsoid> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, ellipsoid: Ellipsoid) -> Result<Self> {
        Geometry::from_with_context(context, ParametricSurface::from(ellipsoid))
    }
}

pub struct Sphere {
    radius: f32,
    radius_segments: u16,
    height_segments: u16,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            radius: 1.0,
            radius_segments: 32,
            height_segments: 16,
        }
    }
}

impl From<Sphere> for Ellipsoid {
    fn from(sphere: Sphere) -> Self {
        Ellipsoid {
            width: sphere.radius,
            height: sphere.radius,
            depth: sphere.radius,
            radius_segments: sphere.radius_segments,
            height_segments: sphere.height_segments,
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, Sphere> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, sphere: Sphere) -> Result<Self> {
        Geometry::from_with_context(context, Ellipsoid::from(sphere))
    }
}

#[derive(Clone, Copy)]
pub struct Cylindrical {
    pub radius_top: f32,
    pub radius_bottom: f32,
    pub height: f32,
    pub radial_segments: u16,
    pub height_segments: u16,
    pub closed_top: bool,
    pub closed_bottom: bool,
}

impl Default for Cylindrical {
    fn default() -> Self {
        Self {
            radius_top: 1.0,
            radius_bottom: 1.0,
            height: 1.0,
            radial_segments: 32,
            height_segments: 4,
            closed_top: true,
            closed_bottom: true,
        }
    }
}

impl Cylindrical {
    fn function(&self, u: f32, v: f32) -> Vec3 {
        glm::vec3(
            glm::lerp_scalar(self.radius_bottom, self.radius_top, v) * u.sin(),
            self.height * (v - 0.5),
            glm::lerp_scalar(self.radius_bottom, self.radius_top, v) * u.cos(),
        )
    }
}

impl From<Cylindrical> for ParametricSurface {
    fn from(cylinder: Cylindrical) -> Self {
        ParametricSurface {
            u_range: 0.0..=TAU,
            u_resolution: cylinder.radial_segments,
            v_range: 0.0..=1.0,
            v_resolution: cylinder.height_segments,
            function: Box::new(move |u, v| cylinder.function(u, v)),
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, Cylindrical> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, cylinder: Cylindrical) -> Result<Self> {
        let mut geometry = Geometry::from_with_context(context, ParametricSurface::from(cylinder))?;

        if cylinder.closed_top {
            let mut top_geometry = Geometry::from_with_context(
                context,
                Polygon::new(cylinder.radial_segments, cylinder.radius_top),
            )?;
            let transform = matrix::translation(0.0, cylinder.height / 2.0, 0.0)
                * matrix::rotation_y(-Angle::RIGHT)
                * matrix::rotation_x(-Angle::RIGHT);
            top_geometry.appply_matrix_mut(context, &transform, "vertexPosition")?;
            geometry.merge_mut(context, top_geometry)?;
        }
        if cylinder.closed_bottom {
            let mut bottom_geometry = Geometry::from_with_context(
                context,
                Polygon::new(cylinder.radial_segments, cylinder.radius_bottom),
            )?;
            let transform = matrix::translation(0.0, -cylinder.height / 2.0, 0.0)
                * matrix::rotation_y(-Angle::RIGHT)
                * matrix::rotation_x(Angle::RIGHT);
            bottom_geometry.appply_matrix_mut(context, &transform, "vertexPosition")?;
            geometry.merge_mut(context, bottom_geometry)?;
        }
        Ok(geometry)
    }
}

pub struct Cylinder {
    radius: f32,
    height: f32,
    radial_segments: u16,
    height_segments: u16,
    closed: bool,
}

impl Default for Cylinder {
    fn default() -> Self {
        Self {
            radius: 1.0,
            height: 1.0,
            radial_segments: 32,
            height_segments: 4,
            closed: true,
        }
    }
}

impl From<Cylinder> for Cylindrical {
    fn from(cylinder: Cylinder) -> Self {
        Self {
            radius_top: cylinder.radius,
            radius_bottom: cylinder.radius,
            height: cylinder.height,
            radial_segments: cylinder.radial_segments,
            height_segments: cylinder.height_segments,
            closed_top: cylinder.closed,
            closed_bottom: cylinder.closed,
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, Cylinder> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, cylinder: Cylinder) -> Result<Self> {
        Geometry::from_with_context(context, Cylindrical::from(cylinder))
    }
}

pub struct Prism {
    radius: f32,
    height: f32,
    sides: u16,
    height_segments: u16,
    closed: bool,
}

impl Default for Prism {
    fn default() -> Self {
        Self {
            radius: 1.0,
            height: 1.0,
            sides: 6,
            height_segments: 4,
            closed: true,
        }
    }
}

impl From<Prism> for Cylindrical {
    fn from(prism: Prism) -> Self {
        Self {
            radius_top: prism.radius,
            radius_bottom: prism.radius,
            height: prism.height,
            radial_segments: prism.sides,
            height_segments: prism.height_segments,
            closed_top: prism.closed,
            closed_bottom: prism.closed,
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, Prism> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, prism: Prism) -> Result<Self> {
        Geometry::from_with_context(context, Cylindrical::from(prism))
    }
}

pub struct Cone {
    radius: f32,
    height: f32,
    radial_segments: u16,
    height_segments: u16,
    closed: bool,
}

impl Default for Cone {
    fn default() -> Self {
        Self {
            radius: 1.0,
            height: 1.0,
            radial_segments: 32,
            height_segments: 4,
            closed: true,
        }
    }
}

impl From<Cone> for Cylindrical {
    fn from(cone: Cone) -> Self {
        Self {
            radius_top: 0.0,
            radius_bottom: cone.radius,
            height: cone.height,
            radial_segments: cone.radial_segments,
            height_segments: cone.height_segments,
            closed_top: false,
            closed_bottom: cone.closed,
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, Cone> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, cone: Cone) -> Result<Self> {
        Geometry::from_with_context(context, Cylindrical::from(cone))
    }
}

pub struct Pyramid {
    radius: f32,
    height: f32,
    sides: u16,
    height_segments: u16,
    closed: bool,
}

impl Default for Pyramid {
    fn default() -> Self {
        Self {
            radius: 1.0,
            height: 1.0,
            sides: 4,
            height_segments: 4,
            closed: true,
        }
    }
}

impl From<Pyramid> for Cylindrical {
    fn from(pyramid: Pyramid) -> Self {
        Self {
            radius_top: 0.0,
            radius_bottom: pyramid.radius,
            height: pyramid.height,
            radial_segments: pyramid.sides,
            height_segments: pyramid.height_segments,
            closed_top: false,
            closed_bottom: pyramid.closed,
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, Pyramid> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, pyramid: Pyramid) -> Result<Self> {
        Geometry::from_with_context(context, Cylindrical::from(pyramid))
    }
}
