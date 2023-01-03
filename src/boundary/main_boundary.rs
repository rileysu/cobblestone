use tokio::sync::mpsc;
use tokio::sync::mpsc::error::TryRecvError;
use std::collections::HashMap;
use super::message::{OutboundMessage, IdentifiedInboundMessage};

#[derive(Debug)]
pub struct MainBoundary {
    messages_txs: HashMap<String, mpsc::UnboundedSender<OutboundMessage>>,
    messages_rx: mpsc::UnboundedReceiver<IdentifiedInboundMessage>,
}

impl MainBoundary {
    pub fn new(messages_rx: mpsc::UnboundedReceiver<IdentifiedInboundMessage>) -> Self {
        Self {
            messages_txs: HashMap::new(),
            messages_rx,
        }
    }

    pub fn recieve_message(&mut self) -> Option<IdentifiedInboundMessage> {
        match self.messages_rx.try_recv() {
            Ok(message) => Some(message),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => panic!("Main's reciever channel has been disconnected!"),
        }
    }

    pub fn send_message(&self, id: &String, message: OutboundMessage) {
        self.messages_txs.get(id).unwrap().send(message).unwrap();
    }

    pub fn register_sender(&mut self, id: &String, channel: mpsc::UnboundedSender<OutboundMessage>) {
        self.messages_txs.insert(id.clone(), channel);
    }

    pub fn remove_sender(&mut self, id: &String) {
        self.messages_txs.remove(id);
    }
}