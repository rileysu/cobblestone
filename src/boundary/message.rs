use tokio::sync::mpsc;

use crate::data::{packets::play::{InboundPlay, OutboundPlay}, base::Uuid};

#[derive(Debug)]
pub enum InboundMessage {
    InitConnection,
    Play(InboundPlay),
    TermConnection,
}

#[derive(Debug)]
pub struct IdentifiedInboundMessage {
    pub uuid: Uuid,
    pub message: InboundMessage,
}

#[derive(Debug)]
pub struct IdentifiedChannel {
    pub uuid : Uuid,
    pub channel: mpsc::UnboundedSender<OutboundMessage>,
}

#[derive(Debug)]
pub enum OutboundMessage {
    Play(OutboundPlay),
}