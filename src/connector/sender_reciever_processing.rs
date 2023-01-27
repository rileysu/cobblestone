use std::io::Cursor;
use crate::data::codec::Codec;
use crate::data::packets::play::{InboundPlay};
use crate::boundary::message::{OutboundMessage, InboundMessage};


pub fn recieve_process(data: &[u8]) -> InboundMessage {
    let mut buf = Cursor::new(data);

    println!("{:?}", buf);

    InboundMessage::Play(InboundPlay::decode(&mut buf).unwrap())
}


pub enum NextAction {
    Send(Vec<u8>),
    Kill,
}


pub fn send_process(message: &OutboundMessage) -> NextAction {
    let mut buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    match message {
        OutboundMessage::Play(play_message) => {
            play_message.encode(&mut buf).unwrap();

            NextAction::Send(buf.into_inner())
        },
    }
}
