use crate::prelude::*;

pub struct CardZones {
    pub deck: Vec<Entity>,
    pub discard: Vec<Entity>,
    pub hand: Vec<Entity>,
}

impl CardZones {
    pub fn new() -> Self {
        Self {
            deck: Vec::new(),
            discard: Vec::new(),
            hand: Vec::new(),
        }
    }
}
