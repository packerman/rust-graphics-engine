use std::{
    collections::{hash_map, HashMap},
    f32::consts::{FRAC_PI_2, TAU},
    ops::RangeInclusive,
};

use anyhow::Result;
use glm::Vec3;
use web_sys::WebGl2RenderingContext;

use super::{attribute::Attribute, color::Color, convert::FromWithContext, matrix::Angle};

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

impl FromWithContext<WebGl2RenderingContext, Rectangle> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, rectangle: Rectangle) -> Result<Self> {
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
            (
                "vertexPosition",
                Attribute::from_array(context, &position_data)?,
            ),
            ("vertexColor", Attribute::from_array(context, &color_data)?),
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

impl FromWithContext<WebGl2RenderingContext, BoxGeometry> for Geometry {
    fn from_with_context(
        context: &WebGl2RenderingContext,
        box_geometry: BoxGeometry,
    ) -> Result<Self> {
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
            (
                "vertexPosition",
                Attribute::from_array(context, &position_data)?,
            ),
            ("vertexColor", Attribute::from_array(context, &color_data)?),
        ]);
        Ok(geometry)
    }
}

struct Polygon {
    sides: u16,
    radius: f32,
}

impl Polygon {
    fn hexagon(radius: f32) -> Self {
        Polygon { sides: 6, radius }
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

impl FromWithContext<WebGl2RenderingContext, Polygon> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, polygon: Polygon) -> Result<Self> {
        let angle = Angle::from_radians(TAU) / polygon.sides.into();
        let mut position_data = Vec::with_capacity((3 * polygon.sides).into());
        let mut color_data = Vec::with_capacity((3 * polygon.sides).into());
        for n in 0..polygon.sides {
            position_data.push([0.0, 0.0, 0.0]);
            position_data.push([
                polygon.radius * (angle * n.into()).cos(),
                polygon.radius * (angle * n.into()).sin(),
                0.0,
            ]);
            position_data.push([
                polygon.radius * (angle * (n + 1).into()).cos(),
                polygon.radius * (angle * (n + 1).into()).sin(),
                0.0,
            ]);
            color_data.push(Color::white().into());
            color_data.push(Color::red().into());
            color_data.push(Color::blue().into());
        }
        let geometry = Geometry::from_attributes([
            (
                "vertexPosition",
                Attribute::from_array(context, &position_data)?,
            ),
            ("vertexColor", Attribute::from_array(context, &color_data)?),
        ]);
        Ok(geometry)
    }
}

pub fn parametric_surface(
    context: &WebGl2RenderingContext,
    u_range: RangeInclusive<f32>,
    u_resolution: u16,
    v_range: RangeInclusive<f32>,
    v_resolution: u16,
    function: Box<dyn Fn(f32, f32) -> Vec3>,
) -> Result<Geometry> {
    let u_delta = (u_range.end() - u_range.start()) / f32::from(u_resolution);
    let v_delta = (v_range.end() - v_range.start()) / f32::from(v_resolution);
    let mut positions = Vec::with_capacity((u_resolution + 1).into());
    for u_index in 0..=u_resolution {
        let mut vector = Vec::with_capacity((v_resolution + 1).into());
        for v_index in 0..=v_resolution {
            let u = u_range.start() + f32::from(u_index) * u_delta;
            let v = v_range.start() + f32::from(v_index) * v_delta;
            vector.push(function(u, v));
        }
        positions.push(vector);
    }
    let mut position_data: Vec<Vec3> = Vec::with_capacity((6 * u_resolution * v_resolution).into());
    let mut color_data: Vec<Color> = Vec::with_capacity((6 * u_resolution * v_resolution).into());
    let colors = [
        Color::red(),
        Color::lime(),
        Color::blue(),
        Color::aqua(),
        Color::fuchsia(),
        Color::yellow(),
    ];
    for x_index in 0..usize::from(u_resolution) {
        for y_index in 0..usize::from(v_resolution) {
            let p_a = positions[x_index][y_index];
            let p_b = positions[x_index + 1][y_index];
            let p_d = positions[x_index][y_index + 1];
            let p_c = positions[x_index + 1][y_index + 1];
            position_data.extend([p_a, p_b, p_c, p_a, p_c, p_d]);
            color_data.extend(colors);
        }
    }
    let geometry = Geometry::from_attributes([
        (
            "vertexPosition",
            Attribute::from_vector_array(context, &position_data)?,
        ),
        (
            "vertexColor",
            Attribute::from_rgb_color_array(context, &color_data)?,
        ),
    ]);
    Ok(geometry)
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

impl FromWithContext<WebGl2RenderingContext, Plane> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, plane: Plane) -> Result<Self> {
        parametric_surface(
            context,
            (-plane.width / 2.0)..=(plane.width / 2.0),
            plane.width_segments,
            (-plane.height / 2.0)..=(plane.height / 2.0),
            plane.height_segments,
            Box::new(|u, v| glm::vec3(u, v, 0.0)),
        )
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

impl FromWithContext<WebGl2RenderingContext, Ellipsoid> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, ellipsoid: Ellipsoid) -> Result<Self> {
        parametric_surface(
            context,
            0.0..=TAU,
            ellipsoid.radius_segments,
            -FRAC_PI_2..=FRAC_PI_2,
            ellipsoid.height_segments,
            Box::new(move |u, v| {
                glm::vec3(
                    ellipsoid.width / 2.0 * u.sin() * v.cos(),
                    ellipsoid.height / 2.0 * v.sin(),
                    ellipsoid.depth / 2.0 * u.cos() * v.sin(),
                )
            }),
        )
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

impl FromWithContext<WebGl2RenderingContext, Sphere> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, sphere: Sphere) -> Result<Self> {
        Geometry::from_with_context(context, Ellipsoid::from(sphere))
    }
}

struct Cylinder {
    radius_top: f32,
    radius_bottom: f32,
    height: f32,
    radial_segments: u16,
    height_segments: u16,
    closed_top: bool,
    closed_bottom: bool,
}

impl Default for Cylinder {
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

impl Cylinder {
    fn function(&self, u: f32, v: f32) -> Vec3 {
        glm::vec3(
            glm::lerp_scalar(self.radius_bottom, self.radius_top, v) * u.sin(),
            self.height * (v - 0.5),
            glm::lerp_scalar(self.radius_bottom, self.radius_top, v) * u.cos(),
        )
    }
}

impl FromWithContext<WebGl2RenderingContext, Cylinder> for Geometry {
    fn from_with_context(context: &WebGl2RenderingContext, cylinder: Cylinder) -> Result<Self> {
        parametric_surface(
            context,
            0.0..=TAU,
            cylinder.radial_segments,
            0.0..=1.0,
            cylinder.height_segments,
            Box::new(move |u, v| cylinder.function(u, v)),
        )
    }
}

mod util {
    use std::{
        iter::{self, Repeat, Take},
        ops::Index,
    };

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
}
