use bevy::prelude::*;

use crate::{simulation::{resources::{message_router::MessageRouter, dimensions::Dimensions, players::Players}}, data::{component::{position::Position, rotation::Rotation, on_ground::OnGround}, compound::player_info::PlayerInfo}, utils};

pub fn init_connection_handler(mut commands: Commands, message_router: ResMut<MessageRouter>, dimensions: Res<Dimensions>, mut players: ResMut<Players>) {
    for ident_message in message_router.get_all_init_messages() {
            info!("Player Connected: {:?}", ident_message.uuid);

            players.spawn_player(
                ident_message.uuid,
                (
                    PlayerInfo { uuid: ident_message.uuid }, 
                    Position::new(0.0, 0.0, 0.0), 
                    Rotation::new(0.0, 0.0), 
                    OnGround(false)
                ),
                &mut commands
            );

            utils::send_init_messages(ident_message.uuid, &message_router, &dimensions);
    }
}

pub fn term_connection_handler(mut commands: Commands, message_router: ResMut<MessageRouter>, mut players: ResMut<Players>) {
    for ident_message in message_router.get_all_term_messages() {
            info!("Player Disconnected: {:?}", ident_message.uuid);

            players.despawn_player(ident_message.uuid, &mut commands);
    }
}