use std::collections::HashMap;

use crate::boundary::message::OutboundMessage;
use crate::data::compound::discrete_position::DiscretePosition;
use crate::data::compound::nbt::NBTValue;
use crate::data::packets::play::{OutboundPlay, Login, PluginMessage, SetDefaultSpawnPosition, SetHeldItem, ChunkDataAndUpdateLight};
use crate::simulation::resources::dimensions::Dimensions;
use crate::simulation::resources::message_router::MessageRouter;
use crate::data::base::*;
use crate::utils;

pub fn send_init_messages(uuid: Uuid, message_router: &MessageRouter, dimensions: &Dimensions) {
    message_router.send_message(uuid, OutboundMessage::Play(OutboundPlay::Login(Login {
        entity_id: 0,
        is_hardcore: false,
        gamemode: 0,
        previous_gamemode: 0,
        dimension_names: LengthPrefixArray(vec![Identifier { namespace: "minecraft".into(), value: "overworld".into() }]),
        registry_codec: utils::registry_codec::generate_registry_codec(&dimensions),
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

    let height_data = vec![0i64; 256];

    height_data[0] = (2.0 + 1.0 as f32).log2().ceil() as i64;

    let chunk_data: Vec<u8> = vec!(0x00, 0x01);

    message_router.send_message(uuid, OutboundMessage::Play(OutboundPlay::ChunkDataAndUpdateLight(ChunkDataAndUpdateLight { 
        chunk_x: 0, 
        chunk_z: 0, 
        heightmaps: NBTValue::Compound(HashMap::from([
            (
                "MOTION_BLOCKING".into(),
                NBTValue::LongArray(height_data),
            )
        ])), 
        data: LengthPrefixByteArray(vec!(0)), 
        num_block_ents: (), 
        trust_edges: (), 
        sky_light_mask: (), 
        block_light_mask: (), 
        empty_sky_light_mask: (), 
        empty_block_light_mask: (), 
        sky_light_array: (), 
        block_light_array: () 
    })))
}