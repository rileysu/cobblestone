use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;
use std::{collections::HashMap};
use crate::codec_data::base::Uuid;

use super::message::{OutboundMessage, InboundMessage};

#[derive(Debug)]
pub struct MainBoundary {
    messages_txs: HashMap<Uuid, mpsc::UnboundedSender<OutboundMessage>>,
    messages_rx: mpsc::UnboundedReceiver<(Uuid, InboundMessage)>,
    channel_rx: mpsc::UnboundedReceiver<(Uuid, mpsc::UnboundedSender<OutboundMessage>)>,
}

impl MainBoundary {
    pub fn new(messages_rx: mpsc::UnboundedReceiver<(Uuid, InboundMessage)>, channel_rx: mpsc::UnboundedReceiver<(Uuid, mpsc::UnboundedSender<OutboundMessage>)>) -> Self {
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

    pub fn recieve_message(&mut self) -> Option<(Uuid, InboundMessage)> {
        match self.channel_rx.try_recv() {
            Ok((uuid, channel)) => {
                self.register_sender(uuid, channel);

                return Some((uuid, InboundMessage::InitConnection))
            },
            Err(TryRecvError::Empty) => {},
            Err(TryRecvError::Disconnected) => panic!("Main's reciever channel has been disconnected!"),
        }

        match self.messages_rx.try_recv() {
            Ok((uuid, message)) => {
                match message {
                    InboundMessage::InitConnection => {
                        Some((uuid, message))
                    },
                    InboundMessage::Play(_) => Some((uuid, message)),
                    InboundMessage::TermConnection => {
                        self.remove_sender(&uuid);

                        Some((uuid, message))
                    },
                }

                
            },
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => panic!("Main's reciever channel has been disconnected!"),
        }
    }

    pub fn recieve_all_messages(&mut self) -> Vec<(Uuid, InboundMessage)> {
        let mut out = Vec::new();

        while let Some(message) = self.recieve_message() {
            out.push(message);
        }

        out
    }

    pub fn send_message(&self, uuid: Uuid, message: OutboundMessage) {
        self.messages_txs.get(&uuid).unwrap().send(message).unwrap();
    }


}