use std::{
    collections::{hash_map, HashMap},
    f32::consts::{FRAC_PI_2, TAU},
    ops::RangeInclusive,
};

use anyhow::Result;
use glm::{Mat4, Vec4};

use super::{
    attribute::{Attribute, AttributeFactory},
    color::Color,
    convert::FromWithContext,
    matrix::{self, Angle},
};

pub struct Geometry {
    attributes: HashMap<String, Attribute>,
}

impl Geometry {
    fn new() -> Self {
        Geometry {
            attributes: HashMap::new(),
        }
    }

    fn from_attributes<const N: usize>(attributes: [(&str, Attribute); N]) -> Self {
        let mut map = HashMap::new();
        for (name, attribute) in attributes {
            map.insert(String::from(name), attribute);
        }
        Geometry { attributes: map }
    }

    pub fn attributes(&self) -> hash_map::Iter<String, Attribute> {
        self.attributes.iter()
    }

    pub fn count_vertices(&self) -> usize {
        self.attributes
            .values()
            .next()
            .expect("Expected at least one attribute")
            .vertex_count
    }
}

struct Rectangle {
    width: f32,
    height: f32,
}

impl Default for Rectangle {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
        }
    }
}

impl FromWithContext<AttributeFactory<'_>, Rectangle> for Geometry {
    fn from_with_context(factory: &AttributeFactory, rectangle: Rectangle) -> Result<Self> {
        let points = [
            [-rectangle.width / 2.0, -rectangle.height / 2.0, 0.0],
            [rectangle.width / 2.0, -rectangle.height / 2.0, 0.0],
            [-rectangle.width / 2.0, rectangle.height / 2.0, 0.0],
            [rectangle.width / 2.0, rectangle.height / 2.0, 0.0],
        ];
        let colors = [
            Color::white().into(),
            Color::red().into(),
            Color::lime().into(),
            Color::blue().into(),
        ];
        let position_data = util::select_by_indices(&points, [0, 1, 3, 0, 3, 2]);
        let color_data = util::select_by_indices(&colors, [0, 1, 3, 0, 3, 2]);
        let geometry = Geometry::from_attributes([
            ("vertexPosition", factory.with_array(&position_data)?),
            ("vertexColor", factory.with_array(&color_data)?),
        ]);
        Ok(geometry)
    }
}

struct BoxGeometry {
    width: f32,
    height: f32,
    depth: f32,
}

impl Default for BoxGeometry {
    fn default() -> Self {
        Self {
            width: 1.0,
            height: 1.0,
            depth: 1.0,
        }
    }
}

impl FromWithContext<AttributeFactory<'_>, BoxGeometry> for Geometry {
    fn from_with_context(factory: &AttributeFactory, box_geometry: BoxGeometry) -> Result<Self> {
        let points = [
            [
                -box_geometry.width / 2.0,
                -box_geometry.height / 2.0,
                -box_geometry.depth / 2.0,
            ],
            [
                box_geometry.width / 2.0,
                -box_geometry.height / 2.0,
                -box_geometry.depth / 2.0,
            ],
            [
                -box_geometry.width / 2.0,
                box_geometry.height / 2.0,
                -box_geometry.depth / 2.0,
            ],
            [
                box_geometry.width / 2.0,
                box_geometry.height / 2.0,
                -box_geometry.depth / 2.0,
            ],
            [
                -box_geometry.width / 2.0,
                -box_geometry.height / 2.0,
                box_geometry.depth / 2.0,
            ],
            [
                box_geometry.width / 2.0,
                -box_geometry.height / 2.0,
                box_geometry.depth / 2.0,
            ],
            [
                -box_geometry.width / 2.0,
                box_geometry.height / 2.0,
                box_geometry.depth / 2.0,
            ],
            [
                box_geometry.width / 2.0,
                box_geometry.height / 2.0,
                box_geometry.depth / 2.0,
            ],
        ];
        let colors = [
            Color::light_coral().into(),
            Color::maroon().into(),
            Color::light_green().into(),
            Color::green().into(),
            Color::medium_slate_blue().into(),
            Color::navy().into(),
        ];
        let position_data = util::select_by_indices(
            &points,
            [
                5, 1, 3, 5, 3, 7, 0, 4, 6, 0, 6, 2, 6, 7, 3, 6, 3, 2, 0, 1, 5, 0, 5, 4, 4, 5, 7, 4,
                7, 6, 1, 0, 2, 1, 3, 3,
            ],
        );
        let color_data =
            util::select_by_indices(&colors, (0..=5).flat_map(|i| util::replicate(6, i)));
        let geometry = Geometry::from_attributes([
            ("vertexPosition", factory.with_array(&position_data)?),
            ("vertexColor", factory.with_array(&color_data)?),
        ]);
        Ok(geometry)
    }
}

struct Polygon {
    sides: u16,
    radius: f32,
}

impl Polygon {
    fn new(sides: u16, radius: f32) -> Self {
        Self { sides, radius }
    }

    fn hexagon(radius: f32) -> Self {
        Polygon { sides: 6, radius }
    }

    pub fn copy_into_vectors(
        &self,
        position_data: &mut Vec<Vec4>,
        color_data: &mut Vec<Color>,
        matrix: &Mat4,
    ) {
        let angle = Angle::from_radians(TAU) / self.sides.into();
        for n in 0..self.sides {
            util::push_transformed(position_data, &glm::vec4(0.0, 0.0, 0.0, 1.0), matrix);
            util::push_transformed(
                position_data,
                &glm::vec4(
                    self.radius * (angle * n.into()).cos(),
                    self.radius * (angle * n.into()).sin(),
                    0.0,
                    1.0,
                ),
                matrix,
            );
            util::push_transformed(
                position_data,
                &glm::vec4(
                    self.radius * (angle * (n + 1).into()).cos(),
                    self.radius * (angle * (n + 1).into()).sin(),
                    0.0,
                    1.0,
                ),
                matrix,
            );
            color_data.push(Color::white());
            color_data.push(Color::red());
            color_data.push(Color::blue());
        }
    }
}

impl Default for Polygon {
    fn default() -> Self {
        Self {
            sides: 3,
            radius: 1.0,
        }
    }
}

impl FromWithContext<AttributeFactory<'_>, Polygon> for Geometry {
    fn from_with_context(factory: &AttributeFactory, polygon: Polygon) -> Result<Self> {
        let mut position_data = Vec::with_capacity((3 * polygon.sides).into());
        let mut color_data = Vec::with_capacity((3 * polygon.sides).into());
        polygon.copy_into_vectors(&mut position_data, &mut color_data, &matrix::identity());
        let geometry = Geometry::from_attributes([
            ("vertexPosition", factory.with_vector_array(&position_data)?),
            ("vertexColor", factory.with_rgba_color_array(&color_data)?),
        ]);
        Ok(geometry)
    }
}

struct ParametricSurface {
    u_range: RangeInclusive<f32>,
    u_resolution: u16,
    v_range: RangeInclusive<f32>,
    v_resolution: u16,
    function: Box<dyn Fn(f32, f32) -> Vec4>,
}

impl ParametricSurface {
    pub fn copy_into_vectors(
        &self,
        position_data: &mut Vec<Vec4>,
        color_data: &mut Vec<Color>,
        matrix: &Mat4,
    ) {
        let u_delta = (self.u_range.end() - self.u_range.start()) / f32::from(self.u_resolution);
        let v_delta = (self.v_range.end() - self.v_range.start()) / f32::from(self.v_resolution);
        let mut positions = Vec::with_capacity((self.u_resolution + 1).into());
        for u_index in 0..=self.u_resolution {
            let mut vector = Vec::with_capacity((self.v_resolution + 1).into());
            for v_index in 0..=self.v_resolution {
                let u = self.u_range.start() + f32::from(u_index) * u_delta;
                let v = self.v_range.start() + f32::from(v_index) * v_delta;
                util::push_transformed(&mut vector, &(self.function)(u, v), matrix)
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
        for x_index in 0..usize::from(self.u_resolution) {
            for y_index in 0..usize::from(self.v_resolution) {
                let p_a = positions[x_index][y_index];
                let p_b = positions[x_index + 1][y_index];
                let p_d = positions[x_index][y_index + 1];
                let p_c = positions[x_index + 1][y_index + 1];
                position_data.extend([p_a, p_b, p_c, p_a, p_c, p_d]);
                color_data.extend(colors);
            }
        }
    }
}

impl FromWithContext<AttributeFactory<'_>, ParametricSurface> for Geometry {
    fn from_with_context(factory: &AttributeFactory, surface: ParametricSurface) -> Result<Self> {
        let mut position_data: Vec<Vec4> =
            Vec::with_capacity((6 * surface.u_resolution * surface.v_resolution).into());
        let mut color_data: Vec<Color> =
            Vec::with_capacity((6 * surface.u_resolution * surface.v_resolution).into());
        surface.copy_into_vectors(&mut position_data, &mut color_data, &matrix::identity());
        let geometry = Geometry::from_attributes([
            ("vertexPosition", factory.with_vector_array(&position_data)?),
            ("vertexColor", factory.with_rgba_color_array(&color_data)?),
        ]);
        Ok(geometry)
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
            function: Box::new(|u, v| glm::vec4(u, v, 0.0, 1.0)),
        }
    }
}

impl FromWithContext<AttributeFactory<'_>, Plane> for Geometry {
    fn from_with_context(factory: &AttributeFactory, plane: Plane) -> Result<Self> {
        Geometry::from_with_context(factory, ParametricSurface::from(plane))
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
                glm::vec4(
                    ellipsoid.width / 2.0 * u.sin() * v.cos(),
                    ellipsoid.height / 2.0 * v.sin(),
                    ellipsoid.depth / 2.0 * u.cos() * v.sin(),
                    1.0,
                )
            }),
        }
    }
}

impl FromWithContext<AttributeFactory<'_>, Ellipsoid> for Geometry {
    fn from_with_context(factory: &AttributeFactory, ellipsoid: Ellipsoid) -> Result<Self> {
        Geometry::from_with_context(factory, ParametricSurface::from(ellipsoid))
    }
}

struct Sphere {
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

impl FromWithContext<AttributeFactory<'_>, Sphere> for Geometry {
    fn from_with_context(factory: &AttributeFactory, sphere: Sphere) -> Result<Self> {
        Geometry::from_with_context(factory, Ellipsoid::from(sphere))
    }
}

#[derive(Clone, Copy)]
struct Cylindrical {
    radius_top: f32,
    radius_bottom: f32,
    height: f32,
    radial_segments: u16,
    height_segments: u16,
    closed_top: bool,
    closed_bottom: bool,
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
    fn function(&self, u: f32, v: f32) -> Vec4 {
        glm::vec4(
            glm::lerp_scalar(self.radius_bottom, self.radius_top, v) * u.sin(),
            self.height * (v - 0.5),
            glm::lerp_scalar(self.radius_bottom, self.radius_top, v) * u.cos(),
            1.0,
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

impl FromWithContext<AttributeFactory<'_>, Cylindrical> for Geometry {
    fn from_with_context(factory: &AttributeFactory, cylinder: Cylindrical) -> Result<Self> {
        let mut position_data: Vec<Vec4> = Vec::new();
        let mut color_data: Vec<Color> = Vec::new();

        let surface = ParametricSurface::from(cylinder);
        surface.copy_into_vectors(&mut position_data, &mut color_data, &matrix::identity());

        if cylinder.closed_top {
            let top = Polygon::new(cylinder.radial_segments, cylinder.radius_top);
            let transform = matrix::translation(0.0, cylinder.height / 2.0, 0.0)
                * matrix::rotation_y(-Angle::RIGHT)
                * matrix::rotation_x(-Angle::RIGHT);
            top.copy_into_vectors(&mut position_data, &mut color_data, &transform);
        }
        if cylinder.closed_bottom {
            let bottom = Polygon::new(cylinder.radial_segments, cylinder.radius_top);
            let transform = matrix::translation(0.0, -cylinder.height / 2.0, 0.0)
                * matrix::rotation_y(-Angle::RIGHT)
                * matrix::rotation_x(Angle::RIGHT);
            bottom.copy_into_vectors(&mut position_data, &mut color_data, &transform);
        }

        let geometry = Geometry::from_attributes([
            ("vertexPosition", factory.with_vector_array(&position_data)?),
            ("vertexColor", factory.with_rgba_color_array(&color_data)?),
        ]);
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

impl FromWithContext<AttributeFactory<'_>, Cylinder> for Geometry {
    fn from_with_context(factory: &AttributeFactory<'_>, cylinder: Cylinder) -> Result<Self> {
        Geometry::from_with_context(factory, Cylindrical::from(cylinder))
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

impl FromWithContext<AttributeFactory<'_>, Prism> for Geometry {
    fn from_with_context(factory: &AttributeFactory<'_>, prism: Prism) -> Result<Self> {
        Geometry::from_with_context(factory, Cylindrical::from(prism))
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

impl FromWithContext<AttributeFactory<'_>, Cone> for Geometry {
    fn from_with_context(factory: &AttributeFactory<'_>, cone: Cone) -> Result<Self> {
        Geometry::from_with_context(factory, Cylindrical::from(cone))
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

impl FromWithContext<AttributeFactory<'_>, Pyramid> for Geometry {
    fn from_with_context(factory: &AttributeFactory<'_>, pyramid: Pyramid) -> Result<Self> {
        Geometry::from_with_context(factory, Cylindrical::from(pyramid))
    }
}

mod util {
    use std::{
        iter::{self, Repeat, Take},
        ops::Index,
    };

    use glm::{Mat4, Vec4};

    pub fn select_by_indices<M, K, V, I>(indexed: &M, indices: I) -> Vec<V>
    where
        M: Index<K, Output = V>,
        I: IntoIterator<Item = K>,
        V: Copy,
    {
        indices.into_iter().map(|k| indexed[k]).collect()
    }

    pub fn replicate<T>(n: usize, t: T) -> Take<Repeat<T>>
    where
        T: Clone,
    {
        iter::repeat(t).take(n)
    }

    pub fn push_transformed(vector: &mut Vec<Vec4>, elem: &Vec4, matrix: &Mat4) {
        vector.push(matrix * elem);
    }
}
