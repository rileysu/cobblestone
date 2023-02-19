use bevy::prelude::*;

use crate::codec_data::base::Uuid;

#[derive(Debug, Component)]
pub struct PlayerInfo {
    pub uuid: Uuid,
}

