use std::collections::HashMap;
use bevy::prelude::*;

use crate::simulation::data::dimension::Dimension;

struct BiomeConfig {
    precipitation: String,
    depth: Option<f32>,
    temperature: f32,
    scale: Option<f32>,
    downfall: f32,
    category: Option<String>,
    temperature_modifier: Option<String>,
    sky_color: i32,
    water_fog_color: i32,
    fog_color: i32,
    water_color: i32,
    foliage_color: Option<i32>,
    grass_color: Option<i32>,
    grass_color_modifier: Option<String>,
}

#[derive(Debug, Resource)]
pub struct Dimensions(pub HashMap<String, Dimension>);

impl Dimensions {
    pub fn new() -> Self {
        Self(
            HashMap::from([
                ("minecraft:overworld".into(), Dimension::default()),
            ])
        )
    }
}