use crate::boundary::message::OutboundMessage;
use crate::data::packets::play::{OutboundPlay, Login, PluginMessage, SetDefaultSpawnPosition};
use crate::simulation::dimensions::Dimensions;
use crate::{boundary::main_boundary::MainBoundary, data::base::*};
use crate::utils;

pub fn send_init_messages(uuid: Uuid, main_boundary: &mut MainBoundary, dimensions: &Dimensions) {
    main_boundary.send_message(uuid, OutboundMessage::Play(OutboundPlay::Login(Login {
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
        death_location: Position {
            x: 0,
            y: 0,
            z: 0,
        },
    })));

    main_boundary.send_message(uuid, OutboundMessage::Play(OutboundPlay::PluginMessage(PluginMessage {
        channel: Identifier { namespace: "minecraft".into(), value: "brand".into() },
        data: ConsumingByteArray("cobblestone".bytes().collect()),
    })));

    main_boundary.send_message(uuid, OutboundMessage::Play(OutboundPlay::SetDefaultSpawnPosition(SetDefaultSpawnPosition {
        location: Position { x: 0, y: 0, z: 0 },
        angle: 0.0,
    })));
}