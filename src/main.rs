mod connector;
mod boundary;
mod data;
mod simulation;
mod utils;

use std::{thread::sleep, time::Duration};

use boundary::{main_boundary::MainBoundary, message::OutboundMessage};
use data::base::*;
use data::packets::play::PluginMessage;
use data::{packets::{status::{InboundStatus, OutboundStatus, StatusResponse, PingResponse}, login::{InboundLogin, OutboundLogin, LoginSuccess}, play::{InboundPlay, OutboundPlay, Login}}};
use connector::connection_handler::ConnectionHandler;
use serde_json::json;
use simulation::server_state::{ServerState};

fn handle_login(uuid: Uuid, main_boundary: &mut MainBoundary, server_state: &mut ServerState) {
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
}

fn handle_play(uuid: Uuid, packet: InboundPlay, main_boundary: &mut MainBoundary, server_state: &mut ServerState) {
    match packet {
        InboundPlay::ClientInformation(client_information) => {

        },
        InboundPlay::PluginMessage(plugin_message) => {
            
        },
        InboundPlay::SetPlayerPositionAndRotation(set_player_pos_rot) => {

        },
        InboundPlay::SetPlayerPosition(set_player_pos) => {
            
        },
    }
}

fn main() {
    let (mut main_boundary, connection_handler) = ConnectionHandler::bootstrap();

    let mut server_state = ServerState::new();

    loop {
        while let Some(message) = main_boundary.recieve_message() {

            println!("{message:?}");

            match message.message {
                boundary::message::InboundMessage::InitConnection => {
                    handle_login(message.uuid, &mut main_boundary, &mut server_state)
                }
                boundary::message::InboundMessage::Play(packet) => {
                    handle_play(message.uuid, packet, &mut main_boundary, &mut server_state);
                },
                boundary::message::InboundMessage::TermConnection => {

                },
            }
        }

        sleep(Duration::from_micros(625)) // 32 per tick!
    }
}

