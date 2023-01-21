mod connector;
mod boundary;
mod data;

use std::{thread::sleep, time::Duration};

use boundary::{main_boundary::MainBoundary, message::OutboundMessage};
use data::{packets::{status::{InboundStatus, OutboundStatus, StatusResponse, PingResponse}, login::{InboundLogin, OutboundLogin, LoginSuccess}, play::{InboundPlay, OutboundPlay, Login}}, base::{LengthPrefixArray, PacketString}};
use connector::connection_handler::ConnectionHandler;
use serde_json::json;

fn handle_status(id: &String, packet: InboundStatus, main_boundary: &mut MainBoundary) {
    match packet {
        InboundStatus::StatusRequest(status_request) => {
            main_boundary.send_message(id, OutboundMessage::Status(OutboundStatus::StatusResponse(StatusResponse {
                json_response: PacketString(json!({
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
                }).to_string()),
            })));
        },
        InboundStatus::PingRequest(ping_request) => {
            main_boundary.send_message(id, OutboundMessage::Status(OutboundStatus::PingResponse(PingResponse {
                payload: ping_request.payload,
            })))
        },
    }
}

fn handle_login(id: &String, packet: InboundLogin, main_boundary: &mut MainBoundary) {
    match packet {
        InboundLogin::LoginStart(login_start) => {
            main_boundary.send_message(id, OutboundMessage::Login(
                OutboundLogin::LoginSuccess(LoginSuccess {
                    uuid: login_start.player_uuid,
                    username: login_start.username,
                    properties: LengthPrefixArray(vec![]),
                }
            )));
        },
        InboundLogin::EncryptionResponse(_) => todo!(),
        InboundLogin::LoginPluginResponse(_) => todo!(),
    }
}

fn handle_play(id: &String, packet: InboundPlay, main_boundary: &mut MainBoundary) {
    match packet {

    }
}

fn main() {
    let (mut main_boundary, connection_handler) = ConnectionHandler::bootstrap();

    loop {
        while let Some(message) = main_boundary.recieve_message() {

            println!("{message:?}");

            match message.message {
                boundary::message::InboundMessage::InitConnection { outbound_tx } => {
                    main_boundary.register_sender(&message.id, outbound_tx);
                }
                boundary::message::InboundMessage::Status(packet) => {
                    handle_status(&message.id, packet, &mut main_boundary);
                },
                boundary::message::InboundMessage::Login(packet) => {
                    handle_login(&message.id, packet, &mut main_boundary);
                },
                boundary::message::InboundMessage::Play(packet) => {
                    handle_play(&message.id, packet, &mut main_boundary);
                },
                boundary::message::InboundMessage::TermConnection => {
                    main_boundary.remove_sender(&message.id);
                },
            }
        }

        sleep(Duration::from_micros(625)) // 32 per tick!
    }
}

