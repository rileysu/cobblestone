use std::collections::HashMap;
use bevy::prelude::*;

use crate::data::compound::dimension::Dimension;

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