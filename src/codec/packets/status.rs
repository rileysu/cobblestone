use codec_derive::Codec;
use codec::{Codec, Result, Error, ErrorKind};
use crate::codec::base::*;
use std::io::{Seek, Read, Write};

#[derive(Debug)]
pub enum InboundStatus {
    StatusRequest(StatusRequest),
    PingRequest(PingRequest),
}

impl Codec for InboundStatus {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        let VarInt(id) = VarInt::decode(buf)?;

        match id {
            0 => Ok(InboundStatus::StatusRequest(
                StatusRequest::decode(buf)?
            )),
            1 => Ok(InboundStatus::PingRequest(
                PingRequest::decode(buf)?
            )),
            _ => Err(Error::from(ErrorKind::InvalidData)),
        }
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum OutboundStatus {
    StatusResponse(StatusResponse),
    PingResponse(PingResponse),
}

impl Codec for OutboundStatus {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        unimplemented!()
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        match self {
            OutboundStatus::StatusResponse(packet) => {
                VarInt::encode(&VarInt(0), buf)?;
                StatusResponse::encode(packet, buf)?;
            },
            OutboundStatus::PingResponse(packet) => {
                VarInt::encode(&VarInt(1), buf)?;
                PingResponse::encode(packet, buf)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Codec)]
pub struct StatusRequest;

#[derive(Debug, Codec)]
pub struct PingRequest {
    pub payload: PacketLong,
}

#[derive(Debug, Codec)]
pub struct StatusResponse {
    pub json_response: PacketString,
}

#[derive(Debug, Codec)]
pub struct PingResponse {
    pub payload: PacketLong,
}