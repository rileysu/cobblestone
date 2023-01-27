use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;
use std::{collections::HashMap};
use crate::data::base::Uuid;

use super::message::{OutboundMessage, IdentifiedInboundMessage, InboundMessage, IdentifiedChannel};

#[derive(Debug)]
pub struct MainBoundary {
    messages_txs: HashMap<Uuid, mpsc::UnboundedSender<OutboundMessage>>,
    messages_rx: mpsc::UnboundedReceiver<IdentifiedInboundMessage>,
    channel_rx: mpsc::UnboundedReceiver<IdentifiedChannel>,
}

impl MainBoundary {
    pub fn new(messages_rx: mpsc::UnboundedReceiver<IdentifiedInboundMessage>, channel_rx: mpsc::UnboundedReceiver<IdentifiedChannel>) -> Self {
        Self {
            messages_txs: HashMap::new(),
            messages_rx,
            channel_rx,
        }
    }

    fn register_sender(&mut self, uuid: Uuid, channel: mpsc::UnboundedSender<OutboundMessage>) {
        self.messages_txs.insert(uuid, channel);
    }

    fn remove_sender(&mut self, uuid: &Uuid) {
        self.messages_txs.remove(uuid);
    }

    pub fn recieve_message(&mut self) -> Option<IdentifiedInboundMessage> {
        match self.channel_rx.try_recv() {
            Ok(ident_channel) => {
                self.register_sender(ident_channel.uuid, ident_channel.channel);

                return Some(IdentifiedInboundMessage { uuid: ident_channel.uuid, message: InboundMessage::InitConnection })
            },
            Err(TryRecvError::Empty) => {},
            Err(TryRecvError::Disconnected) => panic!("Main's reciever channel has been disconnected!"),
        }

        match self.messages_rx.try_recv() {
            Ok(ident_message) => {
                match ident_message.message {
                    InboundMessage::InitConnection => {
                        Some(ident_message)
                    },
                    InboundMessage::Play(_) => Some(ident_message),
                    InboundMessage::TermConnection => {
                        self.remove_sender(&ident_message.uuid);

                        Some(ident_message)
                    },
                }

                
            },
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => panic!("Main's reciever channel has been disconnected!"),
        }
    }

    pub fn send_message(&self, uuid: Uuid, message: OutboundMessage) {
        self.messages_txs.get(&uuid).unwrap().send(message).unwrap();
    }


}