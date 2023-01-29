use bevy::prelude::*;

use crate::data::base::Uuid;

#[derive(Component)]
pub struct PlayerInfo {
    pub uuid: Uuid,
}

