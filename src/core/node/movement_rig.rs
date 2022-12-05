use std::rc::Rc;

use crate::core::{application::Loop, input::KeyState, math::angle::Angle, node::Node};

#[derive(Debug, Clone)]
pub struct Properties {
    pub linear_speed: f32,
    pub angular_speed: Angle,
    pub key_move_forwards: Option<String>,
    pub key_move_backwards: Option<String>,
    pub key_move_left: Option<String>,
    pub key_move_right: Option<String>,
    pub key_move_up: Option<String>,
    pub key_move_down: Option<String>,
    pub key_turn_left: Option<String>,
    pub key_turn_right: Option<String>,
    pub key_look_up: Option<String>,
    pub key_look_down: Option<String>,
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            linear_speed: 1.0,
            angular_speed: Angle::from_degrees(60.0),
            key_move_forwards: Some("KeyW".into()),
            key_move_backwards: Some("KeyS".into()),
            key_move_left: Some("KeyA".into()),
            key_move_right: Some("KeyD".into()),
            key_move_up: Some("KeyR".into()),
            key_move_down: Some("KeyF".into()),
            key_turn_left: Some("KeyQ".into()),
            key_turn_right: Some("KeyE".into()),
            key_look_up: Some("KeyT".into()),
            key_look_down: Some("KeyG".into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MovementRig {
    properties: Properties,
    look_attachment: Rc<Node>,
}

impl MovementRig {
    pub fn new(properties: Properties, look_attachment: Rc<Node>) -> Self {
        Self {
            properties,
            look_attachment,
        }
    }

    pub fn update(&self, key_state: &KeyState, node: &Node) {
        let linear_change = self.properties.linear_speed * (Loop::SECS_PER_UPDATE as f32);
        let angular_change = self.properties.angular_speed * (Loop::SECS_PER_UPDATE as f32);

        if Self::is_key_pressed(&self.properties.key_move_forwards, key_state) {
            node.translate(0.0, 0.0, -linear_change, Default::default())
        }
        if Self::is_key_pressed(&self.properties.key_move_backwards, key_state) {
            node.translate(0.0, 0.0, linear_change, Default::default())
        }
        if Self::is_key_pressed(&self.properties.key_move_left, key_state) {
            node.translate(-linear_change, 0.0, 0.0, Default::default())
        }
        if Self::is_key_pressed(&self.properties.key_move_right, key_state) {
            node.translate(linear_change, 0.0, 0.0, Default::default())
        }
        if Self::is_key_pressed(&self.properties.key_move_up, key_state) {
            node.translate(0.0, linear_change, 0.0, Default::default())
        }
        if Self::is_key_pressed(&self.properties.key_move_down, key_state) {
            node.translate(0.0, -linear_change, 0.0, Default::default())
        }

        if Self::is_key_pressed(&self.properties.key_turn_right, key_state) {
            node.rotate_y(-angular_change, Default::default())
        }
        if Self::is_key_pressed(&self.properties.key_turn_left, key_state) {
            node.rotate_y(angular_change, Default::default())
        }

        if Self::is_key_pressed(&self.properties.key_look_up, key_state) {
            node.rotate_x(angular_change, Default::default())
        }
        if Self::is_key_pressed(&self.properties.key_look_down, key_state) {
            node.rotate_x(-angular_change, Default::default())
        }
    }

    fn is_key_pressed(key: &Option<String>, key_state: &KeyState) -> bool {
        key.as_ref()
            .filter(|key| key_state.is_pressed(key))
            .is_some()
    }

    pub fn add_child(&self, child: &Rc<Node>) {
        self.look_attachment.add_child(child)
    }
}
