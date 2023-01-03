use tokio::sync::mpsc;
use super::message::{IdentifiedInboundMessage, InboundMessage, OutboundMessage};

pub struct SenderConnectionBoundary {
    //Personal rx channel from main to the connection
    messages_rx: mpsc::UnboundedReceiver<OutboundMessage>, 
}

impl SenderConnectionBoundary {
    pub fn new(messages_rx: mpsc::UnboundedReceiver<OutboundMessage>) -> Self {
        Self {
            messages_rx,
        }
    }

    pub async fn recieve_message(&mut self) -> Option<OutboundMessage> {
        self.messages_rx.recv().await
    }
}

pub struct RecieverConnectionBoundary {
    //Shared channel for messages to main
    id: String,
    messages_tx: mpsc::UnboundedSender<IdentifiedInboundMessage>,
}

impl RecieverConnectionBoundary {
    pub fn new(id: &str, messages_tx: mpsc::UnboundedSender<IdentifiedInboundMessage>) -> Self {
        Self {
            id: id.to_string(),
            messages_tx,
        }
    }

    pub fn send_message(&self, message: InboundMessage) -> Result<(), mpsc::error::SendError<IdentifiedInboundMessage>>{
        self.messages_tx.send(IdentifiedInboundMessage {
            id: self.id.clone(),
            message
        })
    }
}
