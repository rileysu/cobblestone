use crate::codec_data::packets::play::{InboundPlay, OutboundPlay};

#[derive(Debug)]
pub enum InboundMessage {
    InitConnection,
    Play(InboundPlay),
    TermConnection,
}

#[derive(Debug)]
pub enum OutboundMessage {
    Play(OutboundPlay),
}