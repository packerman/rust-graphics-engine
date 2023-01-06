pub mod shadow;

use std::cell::RefCell;

use glm::Vec3;

use crate::{
    base::{
        color::{self, Color},
        util::{level::Level, shared_ref::SharedRef},
    },
    core::{
        node::Node,
        program::{self, Program, UpdateUniform},
    },
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

    pub fn as_directional(&self) -> Option<&Vec3> {
        if let Self::Directional { direction } = self {
            Some(direction)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Light {
    pub light_type: Option<LightType>,
    pub color: Color,
    pub attenuation: Attenuation,
    pub node: SharedRef<Node>,
}

impl Light {
    const NONE_TYPE: i32 = 0;
    const DIRECTIONAL_TYPE: i32 = 1;
    const POINT_TYPE: i32 = 2;

    const LIGHT_TYPE_MEMBER: &str = "lightType";
    const COLOR_MEMBER: &str = "color";
    const DIRECTION_MEMBER: &str = "direction";
    const POSITION_MEMBER: &str = "position";
    const ATTENUATION_MEMBER: &str = "attenuation";

    pub fn directional(node: SharedRef<Node>, color: Color, direction: Vec3) -> RefCell<Self> {
        RefCell::new(Self {
            light_type: LightType::directional(direction).into(),
            color,
            node,
            attenuation: Attenuation::default(),
        })
    }

    pub fn point(node: SharedRef<Node>, color: Color, position: Vec3) -> RefCell<Self> {
        RefCell::new(Self {
            light_type: LightType::point(position).into(),
            color,
            node,
            attenuation: Attenuation(1.0, 0.0, 0.1),
        })
    }

    pub fn update_from_node(&mut self) {
        if let Some(light_type) = &mut self.light_type {
            match light_type {
                LightType::Directional { direction } => {
                    *direction = self.node.borrow().direction();
                }
                LightType::Point { position } => {
                    *position = self.node.borrow().position();
                }
            }
        }
    }

    pub fn update_node(&self) {
        if let Some(light_type) = &self.light_type {
            match light_type {
                LightType::Directional { direction } => {
                    self.node.borrow_mut().set_direction(direction);
                }
                LightType::Point { position } => {
                    self.node.borrow_mut().set_position(position);
                }
            }
        }
    }

    pub fn is_directional(&self) -> bool {
        self.light_type
            .map_or(false, |light_type| light_type.is_directional())
    }

    pub fn as_directional(&self) -> Option<&Vec3> {
        self.light_type
            .as_ref()
            .and_then(|light_type| light_type.as_directional())
    }

    pub fn add_child(&self, child: SharedRef<Node>) {
        self.node.borrow_mut().add_child(child)
    }
}

impl UpdateUniform for Light {
    fn update_uniform_with_level(
        &self,
        context: &web_sys::WebGl2RenderingContext,
        name: &str,
        program: &Program,
        level: Level,
    ) {
        if let Some(light_type) = &self.light_type {
            match light_type {
                LightType::Directional { direction } => {
                    Self::DIRECTIONAL_TYPE.update_uniform_with_level(
                        context,
                        &program::join_name(name, Self::LIGHT_TYPE_MEMBER),
                        program,
                        level,
                    );
                    direction.update_uniform_with_level(
                        context,
                        &program::join_name(name, Self::DIRECTION_MEMBER),
                        program,
                        level,
                    );
                }
                LightType::Point { position } => {
                    Self::POINT_TYPE.update_uniform_with_level(
                        context,
                        &program::join_name(name, Self::LIGHT_TYPE_MEMBER),
                        program,
                        level,
                    );
                    position.update_uniform_with_level(
                        context,
                        &program::join_name(name, Self::POSITION_MEMBER),
                        program,
                        level,
                    )
                }
            }
            self.color.update_uniform_with_level(
                context,
                &program::join_name(name, Self::COLOR_MEMBER),
                program,
                level,
            );
            Vec3::from(self.attenuation).update_uniform_with_level(
                context,
                &program::join_name(name, Self::ATTENUATION_MEMBER),
                program,
                level,
            );
        } else {
            Self::NONE_TYPE.update_uniform_with_level(
                context,
                &program::join_name(name, Self::LIGHT_TYPE_MEMBER),
                program,
                level,
            );
        }
    }
}

impl Default for Light {
    fn default() -> Self {
        Self {
            light_type: None,
            color: color::white(),
            attenuation: Attenuation::default(),
            node: Node::empty(),
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
