use std::io::{Cursor};

use serde_json::json;

use crate::data::{
    base::{VarInt, Uuid, LengthPrefixArray},
    codec::Codec,
    packets::{
        handshaking::InboundHandshaking,
        status::{InboundStatus, PingResponse, StatusResponse, OutboundStatus}, login::{InboundLogin, LoginSuccess, OutboundLogin},
    },
};

enum State {
    Handshaking,
    Status,
    Login,
}

pub struct NewConnectionProcessor {
    curr_uuid: Option<Uuid>,
    state: State,
}

impl NewConnectionProcessor {
    pub fn new() -> Self {
        Self {
            curr_uuid: None,
            state: State::Handshaking,
        }
    }

    pub fn process(&mut self, data: &[u8]) -> (Vec<Vec<u8>>, Option<Uuid>) {
        let mut reader = Cursor::new(data);
        

        match self.state {
            State::Handshaking => {
                let packet_group = InboundHandshaking::decode(&mut reader).unwrap();

                match &packet_group {
                    InboundHandshaking::Handshake(packet) => match packet.next_state {
                        VarInt(1) => self.state = State::Status,
                        VarInt(2) => self.state = State::Login,
                        _ => unimplemented!(),
                    },
                };

                (vec![], None)
            }
            State::Status => {
                let mut writer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

                let packet_group = InboundStatus::decode(&mut reader).unwrap();

                match &packet_group {
                    InboundStatus::StatusRequest(packet) => OutboundStatus::StatusResponse(StatusResponse {
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
                        })
                        .to_string(),
                    })
                    .encode(&mut writer)
                    .unwrap(),

                    InboundStatus::PingRequest(packet) => OutboundStatus::PingResponse(PingResponse {
                        payload: packet.payload,
                    })
                    .encode(&mut writer)
                    .unwrap(),
                }

                (vec![writer.into_inner()], None)
            }
            State::Login => {
                let mut writer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

                let packet_group = InboundLogin::decode(&mut reader).unwrap();

                match packet_group {
                    //Currently offline without compression so we can just log the client in after getting this
                    //UUID always available for offline
                    //TODO
                    InboundLogin::LoginStart(packet) => {
                        self.curr_uuid = Some(packet.player_uuid.unwrap());

                        OutboundLogin::LoginSuccess(LoginSuccess {
                            uuid: packet.player_uuid.unwrap(),
                            username: packet.username.clone(),
                            properties: LengthPrefixArray(vec![]),
                        }).encode(&mut writer).unwrap();
                    },
                    InboundLogin::EncryptionResponse(_) => todo!(),
                    InboundLogin::LoginPluginResponse(_) => todo!(),
                }

                (vec![writer.into_inner()], Some(self.curr_uuid.unwrap()))
            },
        }
    }
}
