use std::io::Cursor;

use crate::data::codec::Codec;
use crate::data::packets::handshaking::*;
use crate::data::packets::login::{InboundLogin};
use crate::data::packets::play::{InboundPlay};
use crate::data::packets::status::*;
use crate::data::base::{VarInt};
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

    pub fn process(&mut self, data: &[u8]) -> Option<InboundMessage> {
        let mut buf = Cursor::new(data);

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

                None
            },
            State::Status => {
                Some(InboundMessage::Status(InboundStatus::decode(&mut buf).unwrap()))
            },
            State::Login => {
                let state_packet = InboundLogin::decode(&mut buf).unwrap();
                
                //If online / compression are enabled this will cause major issues
                //TODO
                if let InboundLogin::LoginStart(_) = &state_packet {
                    self.state = State::Play;
                }

                Some(InboundMessage::Login(state_packet))
            },
            State::Play => {
                Some(InboundMessage::Play(InboundPlay::decode(&mut buf).unwrap()))
            },
        }
    }
}

impl SenderProcessor {
    pub fn new() -> Self {
        Self
    }

    pub fn process(&mut self, message: &OutboundMessage) -> Vec<u8> {
        let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());

        match message {
            OutboundMessage::Status(status_message) => status_message.encode(&mut buf).unwrap(),
            OutboundMessage::Login(login_message) => login_message.encode(&mut buf).unwrap(),
            OutboundMessage::Play(play_message) => play_message.encode(&mut buf).unwrap(),
        }

        buf.into_inner()
    }
}