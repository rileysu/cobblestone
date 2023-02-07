use std::collections::HashMap;
use bevy::prelude::*;
use crate::{simulation::{resources::message_router::MessageRouter}, data::{component::{position::Position, rotation::Rotation, on_ground::OnGround}, base::Uuid, packets::play::InboundPlay, compound::player_info::PlayerInfo}, boundary::message::InboundMessage};

pub fn movement_handler(message_router: ResMut<MessageRouter>, mut query: Query<(&mut PlayerInfo, &mut Position, &mut Rotation, &mut OnGround)>) {
    let mut coalesced_positions: HashMap<Uuid, Position> = HashMap::new();
    let mut coalesced_rotations: HashMap<Uuid, Rotation> = HashMap::new();
    let mut coalesced_on_grounds: HashMap<Uuid, OnGround> = HashMap::new();

    for ident_message in message_router.get_all_movement_messages() {
        match &ident_message.message {
            InboundMessage::Play(InboundPlay::SetPlayerPosition(set_player_position)) => {
                coalesced_positions.insert(ident_message.uuid, Position::new(set_player_position.x, set_player_position.feet_y, set_player_position.z));

                coalesced_on_grounds.insert(ident_message.uuid, OnGround(set_player_position.on_ground));
            },
            InboundMessage::Play(InboundPlay::SetPlayerPositionAndRotation(set_player_position_and_rotation)) => {
                coalesced_positions.insert(ident_message.uuid, Position::new(
                    set_player_position_and_rotation.x, 
                    set_player_position_and_rotation.feet_y, 
                    set_player_position_and_rotation.z)
                );

                coalesced_rotations.insert(ident_message.uuid, Rotation::new(
                    set_player_position_and_rotation.yaw, 
                    set_player_position_and_rotation.pitch,)
                );

                coalesced_on_grounds.insert(ident_message.uuid, OnGround(set_player_position_and_rotation.on_ground));
            },
            InboundMessage::Play(InboundPlay::SetPlayerRotation(set_player_rotation)) => {
                coalesced_rotations.insert(ident_message.uuid, Rotation::new(
                    set_player_rotation.yaw, 
                    set_player_rotation.pitch,)
                );
            },
            InboundMessage::Play(InboundPlay::SetPlayerOnGround(set_player_on_ground)) => {
                coalesced_on_grounds.insert(ident_message.uuid, OnGround(set_player_on_ground.on_ground));
            },
            _ => {},
        }
    }

    for (player_info, mut position, mut rotation, mut on_ground) in &mut query {
        if let Some(new_position) = coalesced_positions.get(&player_info.uuid) {
            *position = *new_position;
        }

        if let Some(new_rotation) = coalesced_rotations.get(&player_info.uuid) {
            *rotation = *new_rotation;
        }

        if let Some(new_on_ground) = coalesced_on_grounds.get(&player_info.uuid) {
            *on_ground = *new_on_ground;
        }

        info!("{:?}, {:?}, {:?}, {:?}", player_info, position, rotation, on_ground);
    }
}