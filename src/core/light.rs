pub mod shadow;

use std::cell::RefCell;

use glm::Vec3;

use super::{
    color::Color,
    node::Node,
    uniform::{data::Data, Uniform, UpdateUniform},
};

#[derive(Debug, Clone, Copy)]
pub enum LightType {
    Directional { direction: Vec3 },
    Point { position: Vec3 },
}

impl LightType {
    pub fn directional(direction: Vec3) -> Self {
        Self::Directional { direction }
    }

    pub fn point(position: Vec3) -> Self {
        Self::Point { position }
    }

    pub fn is_directional(&self) -> bool {
        matches!(self, Self::Directional { .. })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub light_type: Option<LightType>,
    pub color: Color,
    pub attenuation: Attenuation,
}

impl Light {
    pub const NONE_TYPE: i32 = 0;
    pub const DIRECTIONAL_TYPE: i32 = 1;
    pub const POINT_TYPE: i32 = 2;

    pub const LIGHT_TYPE_MEMBER: &str = "lightType";
    pub const COLOR_MEMBER: &str = "color";
    pub const DIRECTION_MEMBER: &str = "direction";
    pub const POSITION_MEMBER: &str = "position";
    pub const ATTENUATION_MEMBER: &str = "attenuation";

    pub fn directional(color: Color, direction: Vec3) -> RefCell<Self> {
        RefCell::new(Self {
            light_type: LightType::directional(direction).into(),
            color,
            ..Default::default()
        })
    }

    pub fn point(color: Color, position: Vec3) -> RefCell<Self> {
        RefCell::new(Self {
            light_type: LightType::point(position).into(),
            color,
            attenuation: Attenuation(1.0, 0.0, 0.1),
        })
    }

    pub fn update_from_node(&mut self, node: &Node) {
        if let Some(light_type) = &mut self.light_type {
            match light_type {
                LightType::Directional { direction } => {
                    *direction = node.direction();
                }
                LightType::Point { position } => {
                    *position = node.position();
                }
            }
        }
    }

    pub fn update_node(&self, node: &Node) {
        if let Some(light_type) = &self.light_type {
            match light_type {
                LightType::Directional { direction } => {
                    node.set_direction(direction);
                }
                LightType::Point { position } => {
                    node.set_position(position);
                }
            }
        }
    }

    pub fn is_directional(&self) -> bool {
        self.light_type
            .map_or(false, |light_type| light_type.is_directional())
    }
}

impl Default for Light {
    fn default() -> Self {
        Self {
            light_type: None,
            color: Color::white(),
            attenuation: Default::default(),
        }
    }
}

impl UpdateUniform for Light {
    fn create_data() -> Data {
        Data::from([
            (Self::LIGHT_TYPE_MEMBER, Data::default::<i32>()),
            (Self::COLOR_MEMBER, Data::from(Color::white())),
            (Self::DIRECTION_MEMBER, Data::default::<Vec3>()),
            (Self::POSITION_MEMBER, Data::default::<Vec3>()),
            (Self::ATTENUATION_MEMBER, Data::default::<Vec3>()),
        ])
    }

    fn update_uniform(&self, uniform: &Uniform) {
        if let Some(uniform) = uniform.get_struct() {
            if let Some(light_type) = self.light_type {
                match light_type {
                    LightType::Directional { direction } => {
                        uniform.set_int_member(Self::LIGHT_TYPE_MEMBER, Self::DIRECTIONAL_TYPE);
                        uniform.set_vec3_member(Self::DIRECTION_MEMBER, direction);
                    }
                    LightType::Point { position } => {
                        uniform.set_int_member(Self::LIGHT_TYPE_MEMBER, Self::POINT_TYPE);
                        uniform.set_vec3_member(Self::POSITION_MEMBER, position);
                    }
                }
                uniform.set_color_member(Self::COLOR_MEMBER, self.color);
                uniform.set_vec3_member(Self::ATTENUATION_MEMBER, self.attenuation.into());
            } else {
                uniform.set_int_member(Self::LIGHT_TYPE_MEMBER, Self::NONE_TYPE);
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Attenuation(f32, f32, f32);

impl Default for Attenuation {
    fn default() -> Self {
        Self(1.0, 0.0, 0.0)
    }
}

impl From<Attenuation> for Vec3 {
    fn from(attenuation: Attenuation) -> Self {
        glm::vec3(attenuation.0, attenuation.1, attenuation.2)
    }
}
