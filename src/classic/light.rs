use std::{cell::RefCell, rc::Rc};

use glm::Vec3;

use crate::{
    base::{
        color::{self, Color},
        util::{level::Level, shared_ref::SharedRef},
    },
    core::{
        node::Node,
        program::{self, Program, UpdateUniform},
        scene::Scene,
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

#[derive(Debug)]
pub struct Light {
    pub light_type: Option<LightType>,
    pub color: Color,
    pub attenuation: Attenuation,
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

    pub fn directional(color: Color, direction: Vec3) -> Self {
        Self {
            light_type: LightType::directional(direction).into(),
            color,
            attenuation: Attenuation::default(),
        }
    }

    pub fn point(color: Color, position: Vec3) -> Self {
        Self::new(
            color,
            LightType::point(position).into(),
            Attenuation(1.0, 0.0, 0.1),
        )
    }

    fn new(color: Color, light_type: LightType, attenuation: Attenuation) -> Self {
        Self {
            light_type: light_type.into(),
            color,
            attenuation,
        }
    }

    pub fn update_from_node(&mut self, node: &RefCell<Node>) {
        if let Some(light_type) = &mut self.light_type {
            match light_type {
                LightType::Directional { direction } => {
                    *direction = node.borrow().direction();
                }
                LightType::Point { position } => {
                    *position = node.borrow().position();
                }
            }
        }
    }

    pub fn update_node(&self, node: &RefCell<Node>) {
        if let Some(light_type) = &self.light_type {
            match light_type {
                LightType::Directional { direction } => {
                    node.borrow_mut().set_direction(direction);
                }
                LightType::Point { position } => {
                    node.borrow_mut().set_position(position);
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

#[derive(Debug)]
pub struct Lights {
    light_nodes: Vec<Rc<LightNode>>,
}

impl Lights {
    pub fn new() -> Self {
        Self {
            light_nodes: Vec::new(),
        }
    }

    pub fn create_node(&mut self, light: Light) -> Rc<LightNode> {
        let light_node = LightNode::initialize(Node::new_empty(), RefCell::new(light));
        self.light_nodes.push(Rc::clone(&light_node));
        light_node
    }

    pub fn update(&self) {
        for light_node in self.light_nodes.iter() {
            light_node.update_light();
        }
    }

    #[allow(dead_code)]
    pub fn ensure_light_count(&mut self, count: usize) {
        self.light_nodes.resize_with(count, || {
            LightNode::initialize(Node::new_empty(), RefCell::new(Light::default()))
        });
    }

    pub fn for_each_light_indexed<F>(&self, f: F)
    where
        F: Fn((usize, &RefCell<Light>)),
    {
        for (index, light_node) in self.light_nodes.iter().enumerate() {
            f((index, light_node.light()))
        }
    }
}

impl Default for Lights {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct LightNode {
    node: SharedRef<Node>,
    light: RefCell<Light>,
}

impl LightNode {
    pub fn initialize(node: SharedRef<Node>, light: RefCell<Light>) -> Rc<Self> {
        let me = Self { node, light };
        me.update_node();
        Rc::new(me)
    }

    pub fn add_child(&self, child: SharedRef<Node>) {
        self.node.borrow_mut().add_child(child)
    }

    pub fn light(&self) -> &RefCell<Light> {
        &self.light
    }

    pub fn node(&self) -> &SharedRef<Node> {
        &self.node
    }

    pub fn is_directional(&self) -> bool {
        self.light.borrow().is_directional()
    }

    pub fn as_directional(&self) -> Option<Vec3> {
        self.light.borrow().as_directional().copied()
    }

    pub fn add_to_scene(&self, scene: &mut Scene) {
        scene.add_node(Rc::clone(self.node()))
    }

    pub fn set_position(&self, position: &Vec3) {
        self.node.borrow_mut().set_position(position);
    }

    pub fn set_direction(&self, direction: &Vec3) {
        self.node.borrow_mut().set_direction(direction);
    }

    fn update_node(&self) {
        self.light.borrow().update_node(&self.node);
    }

    fn update_light(&self) {
        self.light.borrow_mut().update_from_node(&self.node);
    }
}
