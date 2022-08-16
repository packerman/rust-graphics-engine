use std::collections::{hash_map, HashMap};

use super::attribute::{Attribute, AttributeData};

pub struct Geometry {
    attributes: HashMap<String, Attribute<Box<dyn AttributeData>>>,
}

impl Geometry {
    pub fn attributes(&self) -> hash_map::Iter<String, Attribute<&dyn AttributeData>> {
        // self.attributes.iter()
        todo!()
    }
}
