use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct Rotation {
    pub yaw: f32,
    pub pitch: f32,
}

impl Rotation {
    pub fn new(yaw: f32, pitch: f32) -> Self {
        Self {
            yaw,
            pitch,
        }
    }
}