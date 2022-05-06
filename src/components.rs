use crate::prelude::*;

pub struct Player;

pub struct Enemy;

pub struct Initiative {
    pub init_mod: i32,
    pub priority: i32,
}

pub struct Name {
    pub val: String,
}

#[derive(Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

pub struct ActionPoints {
    pub current: i32,
    pub max: i32,
}

impl ActionPoints {
    pub fn new(max: i32) -> Self {
        Self { current: max, max }
    }
}

pub struct Message;

pub struct Source {
    pub entity: Entity,
}

pub struct ActionDeclarationFinished;

pub struct Round;
