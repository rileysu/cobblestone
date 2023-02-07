use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Rotation {
    yaw: f32,
    pitch: f32,
}

impl Rotation {
    pub fn new(yaw: f32, pitch: f32) -> Self {
        Self {
            yaw,
            pitch,
        }
    }
}