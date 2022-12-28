use std::cell::RefCell;

use crate::{
    base::{
        application::Loop,
        input::KeyState,
        math::{angle::Angle, matrix},
    },
    core::{camera::Camera, scene::Node},
    gltf::util::shared_ref::SharedRef,
};

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
            linear_speed: 5.0,
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
pub struct CameraController {
    properties: Properties,
    node: SharedRef<Node>,
    attachment: SharedRef<Node>,
}

impl CameraController {
    pub fn new(properties: Properties, node: SharedRef<Node>, attachment: SharedRef<Node>) -> Self {
        Self {
            properties,
            node,
            attachment,
        }
    }

    pub fn make_for_camera(camera: &RefCell<Camera>) -> Option<Self> {
        let node = camera.borrow().node();
        node.map(|node| {
            let attachment = Node::with_name("Attachment");
            node.borrow_mut().transfer_camera(&attachment);
            node.borrow_mut().add_child(attachment.clone());
            Self::new(Properties::default(), node, attachment)
        })
    }

    pub fn update(&self, key_state: &KeyState) {
        let linear_change = self.properties.linear_speed * (Loop::SECS_PER_UPDATE as f32);
        let angular_change = self.properties.angular_speed * (Loop::SECS_PER_UPDATE as f32);

        if Self::is_key_pressed(&self.properties.key_move_forwards, key_state) {
            self.node
                .borrow_mut()
                .apply_transform(&matrix::translation(0.0, 0.0, -linear_change))
        }
        if Self::is_key_pressed(&self.properties.key_move_backwards, key_state) {
            self.node
                .borrow_mut()
                .apply_transform(&matrix::translation(0.0, 0.0, linear_change))
        }
        if Self::is_key_pressed(&self.properties.key_move_left, key_state) {
            self.node
                .borrow_mut()
                .apply_transform(&matrix::translation(-linear_change, 0.0, 0.0))
        }
        if Self::is_key_pressed(&self.properties.key_move_right, key_state) {
            self.node
                .borrow_mut()
                .apply_transform(&matrix::translation(linear_change, 0.0, 0.0))
        }
        if Self::is_key_pressed(&self.properties.key_move_up, key_state) {
            self.node
                .borrow_mut()
                .apply_transform(&matrix::translation(0.0, linear_change, 0.0))
        }
        if Self::is_key_pressed(&self.properties.key_move_down, key_state) {
            self.node
                .borrow_mut()
                .apply_transform(&matrix::translation(0.0, -linear_change, 0.0))
        }

        if Self::is_key_pressed(&self.properties.key_turn_right, key_state) {
            self.node
                .borrow_mut()
                .apply_transform(&matrix::rotation_y(-angular_change))
        }
        if Self::is_key_pressed(&self.properties.key_turn_left, key_state) {
            self.node
                .borrow_mut()
                .apply_transform(&matrix::rotation_y(angular_change))
        }

        if Self::is_key_pressed(&self.properties.key_look_up, key_state) {
            self.attachment
                .borrow_mut()
                .apply_transform(&matrix::rotation_x(angular_change))
        }
        if Self::is_key_pressed(&self.properties.key_look_down, key_state) {
            self.attachment
                .borrow_mut()
                .apply_transform(&matrix::rotation_x(-angular_change))
        }
    }

    fn is_key_pressed(key: &Option<String>, key_state: &KeyState) -> bool {
        key.as_ref()
            .filter(|key| key_state.is_pressed(key))
            .is_some()
    }
}
