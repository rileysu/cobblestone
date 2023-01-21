use tokio::sync::mpsc;

use crate::data::{packets::{login::{InboundLogin, OutboundLogin}, status::{InboundStatus, OutboundStatus}, play::{InboundPlay, OutboundPlay}}};

#[derive(Debug)]
pub enum InboundMessage {
    InitConnection {
        outbound_tx: mpsc::UnboundedSender<OutboundMessage>,
    },
    Status(InboundStatus),
    Login(InboundLogin),
    Play(InboundPlay),
    TermConnection,
}

#[derive(Debug)]
pub struct IdentifiedInboundMessage {
    pub id: String,
    pub message: InboundMessage,
}

#[derive(Debug)]
pub enum OutboundMessage {
    Status(OutboundStatus),
    Login(OutboundLogin),
    Play(OutboundPlay),
}