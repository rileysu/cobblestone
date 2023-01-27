use codec_derive::Codec;
use crate::data::codec;
use crate::data::codec::{Codec, Result, Error, ErrorKind};
use crate::data::base::*;
use std::io::{Seek, Read, Write};

#[derive(Debug)]
pub enum InboundPlay {
    ClientInformation(ClientInformation),
    PluginMessage(PluginMessage),
    SetPlayerPosition(SetPlayerPosition),
    SetPlayerPositionAndRotation(SetPlayerPositionAndRotation),
}

impl Codec for InboundPlay {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        let VarInt(id) = VarInt::decode(buf)?;

        match id {
            0x07 => Ok(InboundPlay::ClientInformation(
                ClientInformation::decode(buf)?
            )),
            0x0C => Ok(InboundPlay::PluginMessage(
                PluginMessage::decode(buf)?
            )),
            0x13 => Ok(InboundPlay::SetPlayerPosition(
                SetPlayerPosition::decode(buf)?
            )),
            0x14 => Ok(InboundPlay::SetPlayerPositionAndRotation(
                SetPlayerPositionAndRotation::decode(buf)?
            )),
            _ => Err(Error::from(ErrorKind::InvalidData)),
        }
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        todo!()
    }
}

#[derive(Debug)]
pub enum OutboundPlay {
    PluginMessage(PluginMessage),
    Login(Login),
}

impl Codec for OutboundPlay {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        unimplemented!()
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        match self {
            OutboundPlay::PluginMessage(plugin_message) => {
                VarInt::encode(&VarInt(0x15), buf)?;
                plugin_message.encode(buf)?;
            }
            OutboundPlay::Login(login) => {
                VarInt::encode(&VarInt(0x24), buf)?;
                login.encode(buf)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Codec)]
pub struct PluginMessage {
    pub channel: Identifier,
    pub data: ConsumingByteArray,
}

#[derive(Debug, Codec)]
pub struct Login {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub gamemode: u8,
    pub previous_gamemode: i8,
    pub dimension_names: LengthPrefixArray<Identifier>,
    pub registry_codec: NBTValue,
    pub dimension_type: Identifier,
    pub dimension_name: Identifier,
    pub hashed_seed: i64,
    pub max_players: VarInt,
    pub view_distance: VarInt,
    pub simulation_distance: VarInt,
    pub reduced_debug_info: bool,
    pub enabled_respawn_screen: bool,
    pub is_debug: bool,
    pub is_flat: bool,
    pub has_death_location: bool,
    pub death_dimension_name: Identifier,
    pub death_location: Position,
}

#[derive(Debug, Codec)]
pub struct ClientInformation {
    pub locale: String,
    pub view_distance: i8,
    pub chat_mode: VarInt,
    pub chat_colours: bool,
    pub display_skin_parts: u8,
    pub main_hand: VarInt,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
}

#[derive(Debug, Codec)]
pub struct SetPlayerPosition {
    x: f64,
    feet_y: f64,
    z: f64,
    on_ground: bool,
}

#[derive(Debug, Codec)]
pub struct SetPlayerPositionAndRotation {
    x: f64,
    feet_y: f64,
    z: f64,
    yaw: f32,
    pitch: f32,
    on_ground: bool,
}