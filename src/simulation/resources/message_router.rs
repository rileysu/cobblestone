use std::collections::VecDeque;
use bevy::prelude::*;
use crate::boundary::main_boundary::MainBoundary;
use crate::boundary::message::{InboundMessage, OutboundMessage};
use crate::codec_data::base::Uuid;
use crate::codec_data::packets::play::{InboundPlay, PluginMessage, ClientInformation, KeepAlive, SetPlayerPositionAndRotation, SetPlayerRotation, SetPlayerOnGround, SetPlayerPosition};

pub enum MovementMessage {
    SetPlayerPosition(SetPlayerPosition),
    SetPlayerPositionAndRotation(SetPlayerPositionAndRotation),
    SetPlayerRotation(SetPlayerRotation),
    SetPlayerOnGround(SetPlayerOnGround),
}

const MESSAGE_RESERVE: usize = 128;

#[derive(Resource)]
pub struct MessageRouter {
    main_boundary: MainBoundary,

    init_messages: VecDeque<Uuid>,
    client_information_messages: VecDeque<(Uuid, ClientInformation)>,
    plugin_messages: VecDeque<(Uuid, PluginMessage)>,
    keep_alive_messages: VecDeque<(Uuid, KeepAlive)>,
    movement_messages: VecDeque<(Uuid, MovementMessage)>,
    term_messages: VecDeque<Uuid>,
}

impl MessageRouter {
    pub fn new(main_boundary: MainBoundary) -> Self {
        let new = Self {
            main_boundary,

            init_messages: VecDeque::with_capacity(MESSAGE_RESERVE),
            client_information_messages: VecDeque::with_capacity(MESSAGE_RESERVE),
            plugin_messages: VecDeque::with_capacity(MESSAGE_RESERVE),
            keep_alive_messages: VecDeque::with_capacity(MESSAGE_RESERVE),
            movement_messages: VecDeque::with_capacity(MESSAGE_RESERVE),
            term_messages: VecDeque::with_capacity(MESSAGE_RESERVE),
        };

        new
    }

    pub fn load_messages(&mut self) {
        for (uuid, message) in self.main_boundary.recieve_all_messages() {
            match message {
                InboundMessage::InitConnection => self.init_messages.push_back(uuid),
                InboundMessage::Play(packet) => match packet {
                    InboundPlay::ClientInformation(packet) => self.client_information_messages.push_back((uuid, packet)),
                    InboundPlay::PluginMessage(packet) => self.plugin_messages.push_back((uuid, packet)),
                    InboundPlay::KeepAlive(packet) => self.keep_alive_messages.push_back((uuid, packet)),
                    InboundPlay::SetPlayerPosition(packet) => self.movement_messages.push_back((uuid, MovementMessage::SetPlayerPosition(packet))),
                    InboundPlay::SetPlayerPositionAndRotation(packet) => self.movement_messages.push_back((uuid, MovementMessage::SetPlayerPositionAndRotation(packet))),
                    InboundPlay::SetPlayerRotation(packet) => self.movement_messages.push_back((uuid, MovementMessage::SetPlayerRotation(packet))),
                    InboundPlay::SetPlayerOnGround(packet) => self.movement_messages.push_back((uuid, MovementMessage::SetPlayerOnGround(packet))),
                },
                InboundMessage::TermConnection => self.term_messages.push_back(uuid),
            }
        }
    }

    pub fn send_message(&self, uuid: Uuid, message: OutboundMessage) {
        self.main_boundary.send_message(uuid, message)
    }
    
    //The clear at the end allows for messages to be shared concurrently
    pub fn clear_all(&mut self) {
        self.init_messages.clear();
        self.client_information_messages.clear();
        self.plugin_messages.clear();
        self.keep_alive_messages.clear();
        self.movement_messages.clear();
        self.term_messages.clear();
    }

    pub fn get_all_init_messages(&self) -> impl Iterator<Item = &Uuid> + '_ {
        self.init_messages.iter()
    }

    pub fn get_all_client_information_messages(&self) -> impl Iterator<Item = &(Uuid, ClientInformation)> + '_ {
        self.client_information_messages.iter()
    }

    pub fn get_all_plugin_messages(&self) -> impl Iterator<Item = &(Uuid, PluginMessage)> + '_ {
        self.plugin_messages.iter()
    }

    pub fn get_all_keep_alive_messages(&self) -> impl Iterator<Item = &(Uuid, KeepAlive)> + '_ {
        self.keep_alive_messages.iter()
    }

    pub fn get_all_movement_messages(&self) -> impl Iterator<Item = &(Uuid, MovementMessage)> + '_ {
        self.movement_messages.iter()
    }

    pub fn get_all_term_messages(&self) -> impl Iterator<Item = &Uuid> + '_ {
        self.term_messages.iter()
    }
}