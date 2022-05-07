use crate::prelude::*;
use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    const X: Self = Self { x: 1, y: 0 };

    const Y: Self = Self { x: 0, y: 1 };
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<i32> for Coordinate {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        let x = self.x * other;
        let y = self.y * other;
        Self { x, y }
    }
}

impl From<Direction> for Coordinate {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => Self::Y * -1,
            Direction::Down => Self::Y,
            Direction::Right => Self::X,
            Direction::Left => Self::X * -1,
        }
    }
}
