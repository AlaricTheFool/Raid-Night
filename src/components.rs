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
