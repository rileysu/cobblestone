use codec_derive::Codec;
use crate::codec_data::codec::{Codec, Result, Error, ErrorKind};
use std::io::{Read, Seek, Write};
use crate::codec_data::base::*;

#[derive(Debug)]
pub enum InboundHandshaking {
    Handshake(Handshake),
}

impl Codec for InboundHandshaking {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        let VarInt(id) = VarInt::decode(buf)?;

        match id {
            0 => Ok(InboundHandshaking::Handshake(
                Handshake::decode(buf)?
            )),
            _ => Err(Error::from(ErrorKind::InvalidData)),
        }
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Debug, Codec)]
pub struct OutboundHandshaking;

#[derive(Debug, Codec)]
pub struct Handshake {
    pub protocol_version: VarInt,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: VarInt,
}

