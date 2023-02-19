use std::io::Cursor;

use bevy::prelude::*;

use crate::{simulation::resources::message_router::MessageRouter, codec_data::{base::{Identifier, ConsumingByteArray}, packets::play::{OutboundPlay, PluginMessage}, codec::Codec}, boundary::message::{OutboundMessage}};

pub fn plugin_communication_handler(message_router: ResMut<MessageRouter>) {
    for (uuid, plugin_message) in message_router.get_all_plugin_messages() {
        match &plugin_message.channel {
            Identifier { namespace, value } if namespace == "minecraft" && value == "brand" => {
                let mut buf = Cursor::<Vec<u8>>::new(vec![]);
                String::from("cobblestone").encode(&mut buf).unwrap();

                message_router.send_message(*uuid, OutboundMessage::Play(OutboundPlay::PluginMessage(PluginMessage {
                    channel: Identifier::new("minecraft".into(), "brand".into()),
                    data: ConsumingByteArray(buf.into_inner()),
                })))
            },
            _ => {},
        }
    }
}