use std::time::Instant;
use rand::random;
use bevy::prelude::*;

use crate::{simulation::{resources::{message_router::MessageRouter, players::Players}, components::{player_info::PlayerInfo, connection::{SentKeepAlive, Ping}}}, boundary::message::OutboundMessage};
use crate::codec_data::packets::play::OutboundPlay;
use crate::codec_data::packets::play::KeepAlive;

#[derive(Debug, StageLabel)]
pub struct KeepAliveStage;

pub fn keep_alive_sender(mut commands: Commands, message_router: ResMut<MessageRouter>, query: Query<(Entity, &PlayerInfo, Option<&SentKeepAlive>)>) {
    for (entity, player_info, sent_keep_alive) in query.iter() {
        if sent_keep_alive.is_none() {
            let keep_alive_id: i64 = random();
            
            message_router.send_message(player_info.uuid, OutboundMessage::Play(OutboundPlay::KeepAlive(KeepAlive {
                keep_alive_id,
            })));

            info!("Sent keep alive: {}", keep_alive_id);

            commands.entity(entity).insert(SentKeepAlive {
                id: keep_alive_id,
                time: Instant::now(),
            });
        }
    }
}

pub fn keep_alive_response_handler(mut commands: Commands, message_router: Res<MessageRouter>, players: Res<Players>, query: Query<(&SentKeepAlive, With<PlayerInfo>)>) {
    for (uuid, message) in message_router.get_all_keep_alive_messages() {
        match players.get_entity(*uuid) {
            Some(entity) => {
                if let Ok((sent_keep_alive, _)) = query.get(entity) {
                    if sent_keep_alive.id == message.keep_alive_id {
                        let ping = Ping(sent_keep_alive.time.elapsed().as_millis() as i32);

                        info!("Got response: {:?}, Ping: {:?}", message, ping);

                        commands.entity(entity).insert(ping);
    
                        commands.entity(entity).remove::<SentKeepAlive>();
                    } else {
                        //TODO
                    }
                } else {
                    //TODO
                };
            },
            None => panic!("Unknown entity sent packet"),
        }
    }
}