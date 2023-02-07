use bevy::prelude::*;

use crate::{simulation::{resources::message_router::MessageRouter}, boundary::message::OutboundMessage, data::{packets::play::{OutboundPlay, KeepAlive}, compound::player_info::PlayerInfo}};

#[derive(Debug, StageLabel)]
pub struct KeepAliveStage;

pub fn keep_alive(message_router: ResMut<MessageRouter>, query: Query<(&PlayerInfo,)>) {
    for (player_info,) in query.iter() {
        message_router.send_message(player_info.uuid, OutboundMessage::Play(OutboundPlay::KeepAlive(KeepAlive {
            keep_alive_id: 1,
        })));
    }
}