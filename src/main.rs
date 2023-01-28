mod connector;
mod boundary;
mod data;
mod simulation;
mod utils;

use std::sync::Arc;
use std::time::{Instant};
use std::{thread::sleep, time::Duration};

use boundary::main_boundary::MainBoundary;
use boundary::message::OutboundMessage;
use data::base::*;
use data::packets::play::{PluginMessage, KeepAlive, SetDefaultSpawnPosition};
use data::{packets::play::{InboundPlay, OutboundPlay, Login}};
use connector::connection_handler::ConnectionHandler;
use simulation::server_state::ServerState;

fn handle_init(uuid: Uuid, server_state: &mut ServerState, main_boundary: &mut MainBoundary) {
    main_boundary.send_message(uuid, OutboundMessage::Play(OutboundPlay::Login(Login {
        entity_id: 0,
        is_hardcore: false,
        gamemode: 0,
        previous_gamemode: 0,
        dimension_names: LengthPrefixArray(vec![Identifier { namespace: "minecraft".into(), value: "overworld".into() }]),
        registry_codec: utils::registry_codec::generate_registry_codec(&server_state.dimensions.iter().collect()),
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
    })))
}

fn handle_play(uuid: Uuid, packet: InboundPlay, server_state: &mut ServerState, main_boundary: &mut MainBoundary) {
    match packet {
        InboundPlay::ClientInformation(client_information) => {

        },
        InboundPlay::PluginMessage(plugin_message) => {
            
        },
        InboundPlay::SetPlayerPositionAndRotation(set_player_pos_rot) => {

        },
        InboundPlay::SetPlayerPosition(set_player_pos) => {

        },
        InboundPlay::KeepAlive(_) => todo!(),
    }
}

fn main() {
    let (mut main_boundary, _connection_handler) = ConnectionHandler::bootstrap();
    let mut server_state = ServerState::new();
    //let mut task_manager = TaskManager::new();

    loop {
        let start = Instant::now();

        //task_manager.poll_and_execute_all(&resources, false);

        for message in main_boundary.recieve_all_messages() {

            println!("{message:?}");

            match message.message {
                boundary::message::InboundMessage::InitConnection => {
                    handle_init(message.uuid, &mut server_state, &mut main_boundary)
                }
                boundary::message::InboundMessage::Play(packet) => {
                    handle_play(message.uuid, packet, &mut server_state, &mut main_boundary);
                },
                boundary::message::InboundMessage::TermConnection => {

                },
            }
        }

        let semi_tick_duration = Duration::from_micros(625);
        let elapsed = start.elapsed();
        
        if semi_tick_duration > elapsed {
            sleep(semi_tick_duration - elapsed); // 32 per tick!
        }
    }
}

