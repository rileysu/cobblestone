use bevy::prelude::*;

use crate::codec_data::compound::position::{DiscretePosition, ContinuousPosition};

#[derive(Component, Debug, Clone, Copy)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x,
            y,
            z,
        }
    }
}

impl From<DiscretePosition> for Position {
    fn from(value: DiscretePosition) -> Self {
        Self {
            x: value.x as f64,
            y: value.y as f64,
            z: value.z as f64,
        }
    }
}

impl From<ContinuousPosition> for Position {
    fn from(value: ContinuousPosition) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}