use std::collections::VecDeque;
use bevy::prelude::*;
use crate::boundary::main_boundary::MainBoundary;
use crate::boundary::message::{IdentifiedInboundMessage, InboundMessage, OutboundMessage};
use crate::data::base::Uuid;
use crate::data::packets::play::InboundPlay;

const MESSAGE_RESERVE: usize = 128;

#[derive(Resource)]
pub struct MessageRouter {
    main_boundary: MainBoundary,

    init_messages: VecDeque<IdentifiedInboundMessage>,
    client_information_messages: VecDeque<IdentifiedInboundMessage>,
    plugin_messages: VecDeque<IdentifiedInboundMessage>,
    keep_alive_messages: VecDeque<IdentifiedInboundMessage>,
    movement_messages: VecDeque<IdentifiedInboundMessage>,
    term_messages: VecDeque<IdentifiedInboundMessage>,
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
        for ident_message in self.main_boundary.recieve_all_messages() {
            match &ident_message.message {
                InboundMessage::InitConnection => self.init_messages.push_back(ident_message),
                InboundMessage::Play(packet) => match packet {
                    InboundPlay::ClientInformation(_) => self.client_information_messages.push_back(ident_message),
                    InboundPlay::PluginMessage(_) => self.plugin_messages.push_back(ident_message),
                    InboundPlay::KeepAlive(_) => self.keep_alive_messages.push_back(ident_message),
                    InboundPlay::SetPlayerPosition(_) => self.movement_messages.push_back(ident_message),
                    InboundPlay::SetPlayerPositionAndRotation(_) => self.movement_messages.push_back(ident_message),
                    InboundPlay::SetPlayerRotation(_) => self.movement_messages.push_back(ident_message),
                    InboundPlay::SetPlayerOnGround(_) => self.movement_messages.push_back(ident_message),
                },
                InboundMessage::TermConnection => self.term_messages.push_back(ident_message),
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

    pub fn get_all_init_messages(&self) -> impl Iterator<Item = &IdentifiedInboundMessage> + '_ {
        self.init_messages.iter()
    }

    pub fn get_all_client_information_messages(&self) -> impl Iterator<Item = &IdentifiedInboundMessage> + '_ {
        self.client_information_messages.iter()
    }

    pub fn get_all_plugin_messages(&self) -> impl Iterator<Item = &IdentifiedInboundMessage> + '_ {
        self.plugin_messages.iter()
    }

    pub fn get_all_keep_alive_messages(&self) -> impl Iterator<Item = &IdentifiedInboundMessage> + '_ {
        self.keep_alive_messages.iter()
    }

    pub fn get_all_movement_messages(&self) -> impl Iterator<Item = &IdentifiedInboundMessage> + '_ {
        self.movement_messages.iter()
    }

    pub fn get_all_term_messages(&self) -> impl Iterator<Item = &IdentifiedInboundMessage> + '_ {
        self.term_messages.iter()
    }
}