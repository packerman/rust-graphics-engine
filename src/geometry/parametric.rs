use std::{
    f32::consts::{FRAC_PI_2, TAU},
    ops::RangeInclusive,
};

use anyhow::Result;
use glm::Vec3;
use web_sys::WebGl2RenderingContext;

use crate::{
    api::geometry::{Geometry, TypedGeometry},
    base::{
        color,
        convert::FromWithContext,
        math::{angle::Angle, matrix},
    },
};

use super::polygon::Polygon;

struct ParametricSurface {
    u_range: RangeInclusive<f32>,
    u_resolution: u16,
    v_range: RangeInclusive<f32>,
    v_resolution: u16,
    function: Box<dyn Fn(f32, f32) -> Vec3>,
    face_normal: bool,
}

impl TryFrom<ParametricSurface> for TypedGeometry {
    type Error = anyhow::Error;

    fn try_from(surface: ParametricSurface) -> Result<Self> {
        let mut position_data =
            Vec::with_capacity((6 * surface.u_resolution * surface.v_resolution).into());
        let mut color_data =
            Vec::with_capacity((6 * surface.u_resolution * surface.v_resolution).into());
        let mut texture_data =
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

        let mut uvs = Vec::with_capacity((surface.u_resolution + 1).into());
        for u_index in 0..=surface.u_resolution {
            let mut vector = Vec::with_capacity((surface.v_resolution + 1).into());
            for v_index in 0..=surface.v_resolution {
                let u = u_index as f32 / surface.u_resolution as f32;
                let v = v_index as f32 / surface.v_resolution as f32;
                vector.push(glm::vec2(u, v));
            }
            uvs.push(vector);
        }

        let mut normals = Vec::with_capacity((surface.u_resolution + 1).into());
        for u_index in 0..=surface.u_resolution {
            let mut vector = Vec::with_capacity((surface.v_resolution + 1).into());
            for v_index in 0..=surface.v_resolution {
                let u = surface.u_range.start() + f32::from(u_index) * u_delta;
                let v = surface.v_range.start() + f32::from(v_index) * v_delta;
                let h = 0.0001;
                let p0 = (surface.function)(u, v);
                let p1 = (surface.function)(u + h, v);
                let p2 = (surface.function)(u, v + h);
                let normal_vector = calc_normal(&p0, &p1, &p2);
                vector.push(normal_vector);
            }
            normals.push(vector);
        }

        let colors = [
            color::red(),
            color::lime(),
            color::blue(),
            color::aqua(),
            color::fuchsia(),
            color::yellow(),
        ];

        let mut vertex_normal_data =
            Vec::with_capacity((6 * surface.u_resolution * surface.v_resolution).into());
        let mut face_normal_data =
            Vec::with_capacity((6 * surface.u_resolution * surface.v_resolution).into());

        for x_index in 0..usize::from(surface.u_resolution) {
            for y_index in 0..usize::from(surface.v_resolution) {
                let p_a = positions[x_index][y_index];
                let p_b = positions[x_index + 1][y_index];
                let p_d = positions[x_index][y_index + 1];
                let p_c = positions[x_index + 1][y_index + 1];
                position_data.extend([p_a, p_b, p_c, p_a, p_c, p_d]);

                color_data.extend(colors);

                let uv_a = uvs[x_index][y_index];
                let uv_b = uvs[x_index + 1][y_index];
                let uv_d = uvs[x_index][y_index + 1];
                let uv_c = uvs[x_index + 1][y_index + 1];
                texture_data.extend([uv_a, uv_b, uv_c, uv_a, uv_c, uv_d]);

                let n_a = normals[x_index][y_index];
                let n_b = normals[x_index + 1][y_index];
                let n_d = normals[x_index][y_index + 1];
                let n_c = normals[x_index + 1][y_index + 1];
                vertex_normal_data.extend([n_a, n_b, n_c, n_a, n_c, n_d]);

                let fn_0 = calc_normal(&p_a, &p_b, &p_c);
                let fn_1 = calc_normal(&p_a, &p_c, &p_d);
                face_normal_data.extend([fn_0, fn_0, fn_0, fn_1, fn_1, fn_1]);
            }
        }

        TypedGeometry::new(
            position_data,
            Some(texture_data),
            Some(if surface.face_normal {
                face_normal_data
            } else {
                vertex_normal_data
            }),
            Some(color_data),
        )
    }
}

impl FromWithContext<WebGl2RenderingContext, ParametricSurface> for Geometry {
    fn from_with_context(
        context: &WebGl2RenderingContext,
        surface: ParametricSurface,
    ) -> Result<Self> {
        let typed_geometry = TypedGeometry::try_from(surface)?;
        Geometry::from_with_context(context, typed_geometry)
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
            face_normal: false,
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, Plane> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, plane: Plane) -> Result<Self> {
        Self::from_with_context(context, ParametricSurface::from(plane))
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
            face_normal: false,
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, Ellipsoid> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, ellipsoid: Ellipsoid) -> Result<Self> {
        Self::from_with_context(context, ParametricSurface::from(ellipsoid))
    }
}

pub struct Sphere {
    pub radius: f32,
    pub radius_segments: u16,
    pub height_segments: u16,
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
            width: 2.0 * sphere.radius,
            height: 2.0 * sphere.radius,
            depth: 2.0 * sphere.radius,
            radius_segments: sphere.radius_segments,
            height_segments: sphere.height_segments,
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, Sphere> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, sphere: Sphere) -> Result<Self> {
        Self::from_with_context(context, Ellipsoid::from(sphere))
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
            face_normal: false,
        }
    }
}

impl FromWithContext<WebGl2RenderingContext, Cylindrical> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, cylinder: Cylindrical) -> Result<Self> {
        let mut cylinder_geometry = TypedGeometry::try_from(ParametricSurface::from(cylinder))?;

        if cylinder.closed_top {
            let mut top_geometry = TypedGeometry::try_from(Polygon::new(
                cylinder.radial_segments,
                cylinder.radius_top,
            ))?;
            let transform = matrix::translation(0.0, cylinder.height / 2.0, 0.0)
                * matrix::rotation_y(-Angle::RIGHT)
                * matrix::rotation_x(-Angle::RIGHT);
            top_geometry.transform_mut(&transform);
            cylinder_geometry.concat_mut(&top_geometry)?;
        }
        if cylinder.closed_bottom {
            let mut bottom_geometry = TypedGeometry::try_from(Polygon::new(
                cylinder.radial_segments,
                cylinder.radius_bottom,
            ))?;
            let transform = matrix::translation(0.0, -cylinder.height / 2.0, 0.0)
                * matrix::rotation_y(-Angle::RIGHT)
                * matrix::rotation_x(Angle::RIGHT);
            bottom_geometry.transform_mut(&transform);
            cylinder_geometry.concat_mut(&bottom_geometry)?;
        }
        Geometry::from_with_context(context, cylinder_geometry)
    }
}

pub struct Cylinder {
    pub radius: f32,
    pub height: f32,
    pub radial_segments: u16,
    pub height_segments: u16,
    pub closed: bool,
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
        Self::from_with_context(context, Cylindrical::from(cylinder))
    }
}

pub struct Prism {
    pub radius: f32,
    pub height: f32,
    pub sides: u16,
    pub height_segments: u16,
    pub closed: bool,
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
        Self::from_with_context(context, Cylindrical::from(prism))
    }
}

pub struct Cone {
    pub radius: f32,
    pub height: f32,
    pub radial_segments: u16,
    pub height_segments: u16,
    pub closed: bool,
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
        Self::from_with_context(context, Cylindrical::from(cone))
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
        Self::from_with_context(context, Cylindrical::from(pyramid))
    }
}

fn calc_normal(p0: &Vec3, p1: &Vec3, p2: &Vec3) -> Vec3 {
    let v1 = p1 - p0;
    let v2 = p2 - p0;
    let normal = glm::cross(&v1, &v2);
    glm::normalize(&normal)
}
