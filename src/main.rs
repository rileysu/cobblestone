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

fn handle_status(id: &String, packet: InboundStatus, main_boundary: &mut MainBoundary, server_state: &mut ServerState) {
    match packet {
        InboundStatus::StatusRequest(status_request) => {
            main_boundary.send_message(id, OutboundMessage::Status(OutboundStatus::StatusResponse(StatusResponse {
                json_response: json!({
                    "version": {
                        "name": "1.19.3",
                        "protocol": 761,
                    },
                    "players": {
                        "max": 100,
                        "online": 0,
                        "sample": [],
                    },
                    "description": {
                        "text": "Cobblestone Server!",
                    },
                    "favicon": "data:image/png;base64,<data>".to_string(),
                    "previewsChat": true,
                    "enforcesSecureChat": true,
                }).to_string(),
            })));
        },
        InboundStatus::PingRequest(ping_request) => {
            main_boundary.send_message(id, OutboundMessage::Status(OutboundStatus::PingResponse(PingResponse {
                payload: ping_request.payload,
            })))
        },
    }
}

fn handle_login(id: &String, packet: InboundLogin, main_boundary: &mut MainBoundary, server_state: &mut ServerState) {
    match packet {
        InboundLogin::LoginStart(login_start) => {
            main_boundary.send_message(id, OutboundMessage::Login(
                OutboundLogin::LoginSuccess(LoginSuccess {
                    uuid: login_start.player_uuid,
                    username: login_start.username,
                    properties: LengthPrefixArray(vec![]),
                }
            )));
            
            main_boundary.send_message(id, OutboundMessage::Play(OutboundPlay::Login(Login {
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

            main_boundary.send_message(id, OutboundMessage::Play(OutboundPlay::PluginMessage(PluginMessage {
                channel: Identifier { namespace: "minecraft".into(), value: "brand".into() },
                data: ConsumingByteArray("cobblestone".bytes().collect()),
            })));
        },
        InboundLogin::EncryptionResponse(_) => todo!(),
        InboundLogin::LoginPluginResponse(_) => todo!(),
    }
}

fn handle_play(id: &String, packet: InboundPlay, main_boundary: &mut MainBoundary, server_state: &mut ServerState) {
    match packet {
        InboundPlay::ClientInformation(client_information) => {

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
                boundary::message::InboundMessage::InitConnection { outbound_tx } => {
                    main_boundary.register_sender(&message.id, outbound_tx);
                }
                boundary::message::InboundMessage::Status(packet) => {
                    handle_status(&message.id, packet, &mut main_boundary, &mut server_state);
                },
                boundary::message::InboundMessage::Login(packet) => {
                    handle_login(&message.id, packet, &mut main_boundary, &mut server_state);
                },
                boundary::message::InboundMessage::Play(packet) => {
                    handle_play(&message.id, packet, &mut main_boundary, &mut server_state);
                },
                boundary::message::InboundMessage::TermConnection => {
                    main_boundary.remove_sender(&message.id);
                },
            }
        }

        sleep(Duration::from_micros(625)) // 32 per tick!
    }
}

