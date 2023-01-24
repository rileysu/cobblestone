use codec_derive::Codec;
use crate::data::codec;
use crate::data::codec::{Codec, Result, Error, ErrorKind};
use std::io::{Read, Seek, Write};
use crate::data::base::*;

#[derive(Debug)]
pub enum InboundLogin {
    LoginStart(LoginStart),
    EncryptionResponse(EncryptionResponse),
    LoginPluginResponse(LoginPluginResponse),
}

impl Codec for InboundLogin {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        let VarInt(id) = VarInt::decode(buf)?;

        match id {
            0 => Ok(InboundLogin::LoginStart(
                LoginStart::decode(buf)?
            )),
            1 => Ok(InboundLogin::EncryptionResponse(
                EncryptionResponse::decode(buf)?
            )),
            2 => Ok(InboundLogin::LoginPluginResponse(
                LoginPluginResponse::decode(buf)?
            )),
            _ => Err(Error::from(ErrorKind::InvalidData)),
        }
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub enum OutboundLogin {
    Disconnect(Disconnect),
    EncryptionRequest(EncryptionRequest),
    LoginSuccess(LoginSuccess),
    SetCompression(SetCompression),
    LoginPluginRequest(LoginPluginRequest),
}

impl Codec for OutboundLogin {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        unimplemented!()
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        match self {
            OutboundLogin::Disconnect(packet) => {
                VarInt::encode(&VarInt(0), buf)?;
                Disconnect::encode(packet, buf)?;
            },
            OutboundLogin::EncryptionRequest(packet) => {
                VarInt::encode(&VarInt(1), buf)?;
                EncryptionRequest::encode(packet, buf)?;
            },
            OutboundLogin::LoginSuccess(packet) => {
                VarInt::encode(&VarInt(2), buf)?;
                LoginSuccess::encode(packet, buf)?;
            },
            OutboundLogin::SetCompression(packet) => {
                VarInt::encode(&VarInt(3), buf)?;
                SetCompression::encode(packet, buf)?;
            },
            OutboundLogin::LoginPluginRequest(packet) => {
                VarInt::encode(&VarInt(4), buf)?;
                LoginPluginRequest::encode(packet, buf)?;
            },
        }

        Ok(())
    }
}

#[derive(Debug, Codec)]
pub struct LoginStart {
    pub username: String,
    pub has_player_uuid: bool,
    pub player_uuid: Uuid,
}

#[derive(Debug, Codec)]
pub struct EncryptionResponse {
    pub shared_secret: LengthPrefixByteArray,
    pub verify_token: LengthPrefixByteArray,
}

#[derive(Debug, Codec)] 
pub struct LoginPluginResponse {
    pub message_id: VarInt,
    pub successful: bool,
    pub data: ConsumingByteArray,
}

#[derive(Debug, Codec)]
pub struct Disconnect {
    pub reason: String
}

#[derive(Debug, Codec)]
pub struct EncryptionRequest {
    pub server_id: String,
    pub public_key: LengthPrefixByteArray,
    pub verify_token: LengthPrefixByteArray,
}

#[derive(Debug, Codec)]
pub struct LoginSuccessProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Debug, Codec)]
pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String,
    pub properties: LengthPrefixArray<LoginSuccessProperty>,
}

#[derive(Debug, Codec)]
pub struct SetCompression {
    pub threshold: VarInt,
}

#[derive(Debug, Codec)]
pub struct LoginPluginRequest {
    pub message_id: VarInt,
    pub channel: String,
    pub data: ConsumingByteArray,
}