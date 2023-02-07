use bevy::prelude::*;

use crate::data::base::Uuid;

#[derive(Debug, Component)]
pub struct PlayerInfo {
    pub uuid: Uuid,
}

