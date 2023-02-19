use std::collections::HashMap;

use bevy::prelude::*;

use crate::codec_data::base::Uuid;

#[derive(Resource)]
pub struct Players(pub HashMap<Uuid, Entity>);

impl Players {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn spawn_player(&mut self, uuid: Uuid, bundle: impl Bundle, commands: &mut Commands) {
        self.0.insert(uuid, commands.spawn(bundle).id());
    }

    pub fn despawn_player(&mut self, uuid: Uuid, commands: &mut Commands) {
        if let Some(entity) = self.0.remove(&uuid) {
            commands.entity(entity).despawn();
        }
    }

    pub fn get_entity(&self, uuid: Uuid) -> Option<Entity> {
        self.0.get(&uuid).copied()
    }
}