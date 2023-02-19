use std::io::Cursor;
use crate::codec_data::codec::{Codec, Error};
use crate::codec_data::packets::play::{InboundPlay};
use crate::boundary::message::{OutboundMessage, InboundMessage};


pub fn recieve_process(data: &[u8]) -> Result<InboundMessage, Error> {
    let mut buf = Cursor::new(data);

    Ok(InboundMessage::Play(InboundPlay::decode(&mut buf)?))
}


pub enum NextAction {
    Send(Vec<u8>),
    Kill,
}


pub fn send_process(message: &OutboundMessage) -> Result<NextAction, Error> {
    let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    match message {
        OutboundMessage::Play(play_message) => {
            play_message.encode(&mut buf)?;

            Ok(NextAction::Send(buf.into_inner()))
        },
    }
}
