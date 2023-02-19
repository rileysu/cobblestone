use crate::boundary::message::OutboundMessage;
use crate::codec_data::compound::position::DiscretePosition;
use crate::codec_data::packets::play::{OutboundPlay, Login, PluginMessage, SetDefaultSpawnPosition, SetHeldItem};
use crate::simulation::resources::dimensions::Dimensions;
use crate::simulation::resources::message_router::MessageRouter;
use crate::codec_data::base::*;
use crate::simulation::utils;

pub fn send_init_messages(uuid: Uuid, message_router: &MessageRouter, dimensions: &Dimensions) {
    message_router.send_message(uuid, OutboundMessage::Play(OutboundPlay::Login(Login {
        entity_id: 0,
        is_hardcore: false,
        gamemode: 0,
        previous_gamemode: 0,
        dimension_names: LengthPrefixArray(vec![Identifier { namespace: "minecraft".into(), value: "overworld".into() }]),
        registry_codec: utils::generate_registry_codec(&dimensions),
        dimension_type: Identifier { namespace: "minecraft".into(), value: "overworld".into() },
        dimension_name: Identifier { namespace: "minecraft".into(), value: "overworld".into() },
        hashed_seed: 0,
        max_players: VarInt(100),
        view_distance: VarInt(16),
        simulation_distance: VarInt(16),
        reduced_debug_info: false,
        enabled_respawn_screen: true,
        is_debug: true,
        is_flat: true,
        has_death_location: true,
        death_dimension_name: Identifier { namespace: "minecraft".into(), value: "overworld".into() },
        death_location: DiscretePosition {
            x: 0,
            y: 0,
            z: 0,
        },
    })));

    //This triggers the client to send its brand first
    message_router.send_message(uuid, OutboundMessage::Play(OutboundPlay::PluginMessage(PluginMessage {
        channel: Identifier { namespace: "minecraft".into(), value: "brand".into() },
        data: ConsumingByteArray(vec![]),
    })));

    message_router.send_message(uuid, OutboundMessage::Play(OutboundPlay::SetHeldItem(SetHeldItem {
        slot: 0,
    })));

    message_router.send_message(uuid, OutboundMessage::Play(OutboundPlay::SetDefaultSpawnPosition(SetDefaultSpawnPosition {
        location: DiscretePosition { x: 0, y: 0, z: 0 },
        angle: 0.0,
    })));
}