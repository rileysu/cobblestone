use std::io::Cursor;

use codec::Codec;
use serde_json::json;
use crate::codec::base::{PacketLong, Uuid, LengthPrefixArray, PacketOption};
use crate::codec::packets::handshaking::*;
use crate::codec::packets::login::{InboundLogin, OutboundLogin, LoginSuccess, LoginSuccessProperty};
use crate::codec::packets::play::Login;
use crate::codec::packets::status::*;
use crate::codec::base::{VarInt, PacketString};
use crate::boundary::message::{OutboundMessage, InboundMessage};

#[derive(Debug)]
enum State {
    Handshaking,
    Status,
    Login,
    Play,
}

#[derive(Debug)]
pub struct RecieverProcessor {
    state: State,
}

pub struct SenderProcessor;

impl RecieverProcessor {
    pub fn new() -> Self {
        Self {
            state: State::Handshaking,
        }
    }

    pub fn process(&mut self, data: &[u8]) -> Vec<InboundMessage> {
        let mut buf = Cursor::new(data);
        let mut out: Vec<InboundMessage> = Vec::new(); 

        match self.state {
            State::Handshaking => {
                let state_packet = InboundHandshaking::decode(&mut buf).unwrap();

                match &state_packet {
                    InboundHandshaking::Handshake(packet) => {
                        match packet.next_state {
                            VarInt(1) => self.state = State::Status,
                            VarInt(2) => self.state = State::Login,
                            _ => todo!(),
                        }
                    },
                };
            },
            State::Status => {
                let state_packet = InboundStatus::decode(&mut buf).unwrap();

                match &state_packet {
                    InboundStatus::StatusRequest(packet) => out.push(InboundMessage::ServerInformationRequest),
                    InboundStatus::PingRequest(packet) => out.push(InboundMessage::PingRequest { 
                            payload: packet.payload.0 
                        }
                    ),
                };
            },
            State::Login => {
                let state_packet = InboundLogin::decode(&mut buf).unwrap();

                match &state_packet {
                    InboundLogin::LoginStart(packet) => {
                        out.push(InboundMessage::LoginStart { username: packet.username.0.clone(), uuid: packet.player_uuid.0 })
                    },
                    InboundLogin::EncryptionResponse(packet) => {
                        
                    },
                    InboundLogin::LoginPluginResponse(packet) => {
                        
                    },
                }
            },
            State::Play => {
                todo!()
            }
        }

        out
    }
}

impl SenderProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn process(&mut self, message: &OutboundMessage) -> Vec<u8> {
        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        match message {
            OutboundMessage::ServerInformationResponse { 
                version_name, 
                version_protocol, 
                players_max, 
                players_online, 
                sample, 
                description_text, 
                favicon, 
                previews_chat, 
                enforce_secure_chat } => {
                    let mat_json = json!({
                        "version": {
                            "name": version_name,
                            "protocol": version_protocol
                        },
                        "players": {
                            "max": players_max,
                            "online": players_online,
                            "sample": []
                        },
                        "description": {
                            "text": description_text
                        },
                        "favicon": favicon,
                        "previewsChat": previews_chat,
                        "enforcesSecureChat": enforce_secure_chat,
                    });

                    let packet = OutboundStatus::StatusResponse(StatusResponse {
                        json_response: PacketString(mat_json.to_string()),
                    });

                    packet.encode(&mut buf).unwrap();
            },
            OutboundMessage::PingResponse { 
                payload 
            } => {
                let packet = OutboundStatus::PingResponse(PingResponse { 
                    payload: PacketLong(*payload) 
                });

                packet.encode(&mut buf).unwrap();
            }
            OutboundMessage::LoginSuccess { 
                username, 
                uuid, 
                properties 
            } => {
                let packet = OutboundLogin::LoginSuccess(LoginSuccess {
                    uuid: Uuid(*uuid),
                    username: PacketString(username.clone()),
                    properties: LengthPrefixArray(properties.iter().map({ |property|
                        LoginSuccessProperty {
                            name: PacketString(property.name.clone()),
                            value: PacketString(property.value.clone()),
                            signature: match &property.signature {
                                Some(value) => PacketOption::Some(PacketString(value.clone())),
                                None => PacketOption::None,
                            },
                        }
                    }).collect()),
                });

                packet.encode(&mut buf).unwrap();
            },
            OutboundMessage::Login { entity_id, is_hardcore, gamemode, previous_gamemode, dimension_names } => {},
        };

        buf.into_inner()
    }
}