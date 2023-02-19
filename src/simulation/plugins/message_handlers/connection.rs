use bevy::prelude::*;

use crate::simulation::{components::{player_info::PlayerInfo, position::Position, rotation::Rotation, on_ground::OnGround}, resources::{message_router::MessageRouter, dimensions::Dimensions, players::Players}, utils};

pub fn init_connection_handler(mut commands: Commands, message_router: ResMut<MessageRouter>, dimensions: Res<Dimensions>, mut players: ResMut<Players>) {
    for uuid in message_router.get_all_init_messages() {
            info!("Player Connected: {:?}", uuid);

            players.spawn_player(
                *uuid,
                (
                    PlayerInfo { uuid: *uuid }, 
                    Position::new(0.0, 0.0, 0.0), 
                    Rotation::new(0.0, 0.0), 
                    OnGround(false)
                ),
                &mut commands
            );

            utils::send_init_messages(*uuid, &message_router, &dimensions);
    }
}

pub fn term_connection_handler(mut commands: Commands, message_router: ResMut<MessageRouter>, mut players: ResMut<Players>) {
    for uuid in message_router.get_all_term_messages() {
            info!("Player Disconnected: {:?}", uuid);

            players.despawn_player(*uuid, &mut commands);
    }
}