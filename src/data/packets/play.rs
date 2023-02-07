use codec_derive::Codec;
use crate::data::codec;
use crate::data::codec::{Codec, Result, Error, ErrorKind};
use crate::data::base::*;
use crate::data::compound::{discrete_position::DiscretePosition, nbt::NBTValue};
use std::io::{Seek, Read, Write};

#[derive(Debug)]
pub enum InboundPlay {
    ClientInformation(ClientInformation),
    PluginMessage(PluginMessage),
    KeepAlive(KeepAlive),
    SetPlayerPosition(SetPlayerPosition),
    SetPlayerPositionAndRotation(SetPlayerPositionAndRotation),
    SetPlayerRotation(SetPlayerRotation),
    SetPlayerOnGround(SetPlayerOnGround),
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
            0x11 => Ok(InboundPlay::KeepAlive(
                KeepAlive::decode(buf)?
            )),
            0x13 => Ok(InboundPlay::SetPlayerPosition(
                SetPlayerPosition::decode(buf)?
            )),
            0x14 => Ok(InboundPlay::SetPlayerPositionAndRotation(
                SetPlayerPositionAndRotation::decode(buf)?
            )),
            0x15 => Ok(InboundPlay::SetPlayerRotation(
                SetPlayerRotation::decode(buf)?
            )),
            0x16 => Ok(InboundPlay::SetPlayerOnGround(
                SetPlayerOnGround::decode(buf)?
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
    EntityEvent(EntityEvent),
    KeepAlive(KeepAlive),
    Login(Login),
    SetDefaultSpawnPosition(SetDefaultSpawnPosition),
    SetHeldItem(SetHeldItem),
    UpdateRecipes(UpdateRecipes),
    UpdateTags(UpdateTags),
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
            },
            OutboundPlay::EntityEvent(entity_event) => {
                VarInt::encode(&VarInt(0x19), buf)?;
                entity_event.encode(buf)?;
            },
            OutboundPlay::KeepAlive(keep_alive) => {
                VarInt::encode(&VarInt(0x1F), buf)?;
                keep_alive.encode(buf)?;
            },
            OutboundPlay::Login(login) => {
                VarInt::encode(&VarInt(0x24), buf)?;
                login.encode(buf)?;
            },
            OutboundPlay::SetDefaultSpawnPosition(spawn_pos) => {
                VarInt::encode(&VarInt(0x4C), buf)?;
                spawn_pos.encode(buf)?;
            },
            OutboundPlay::SetHeldItem(set_held_item) => {
                VarInt::encode(&VarInt(0x49), buf)?;
                set_held_item.encode(buf)?;
            },
            OutboundPlay::UpdateRecipes(update_recipes) => {
                VarInt::encode(&VarInt(0x69), buf)?;
                update_recipes.encode(buf)?;
            },
            OutboundPlay::UpdateTags(update_tags) => {
                VarInt::encode(&VarInt(0x6A), buf)?;
                update_tags.encode(buf)?;
            },
        }

        Ok(())
    }
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
pub struct PluginMessage {
    pub channel: Identifier,
    pub data: ConsumingByteArray,
}

#[derive(Debug, Codec)]
pub struct KeepAlive {
    pub keep_alive_id: i64,
}

#[derive(Debug, Codec)]
pub struct SetPlayerPosition {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub on_ground: bool,
}

#[derive(Debug, Codec)]
pub struct SetPlayerPositionAndRotation {
    pub x: f64,
    pub feet_y: f64,
    pub z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub on_ground: bool,
}

#[derive(Debug, Codec)]
pub struct SetPlayerRotation {
    pub yaw: f32,
    pub pitch: f32,
}

#[derive(Debug, Codec)]
pub struct SetPlayerOnGround {
    pub on_ground: bool,
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
    pub death_location: DiscretePosition,
}

#[derive(Debug, Codec)]
pub struct SetDefaultSpawnPosition {
    pub location: DiscretePosition,
    pub angle: f32,
}

#[derive(Debug, Codec)]
pub struct SetHeldItem {
    pub slot: i8,
}

//TODO: Fix this according to specs
#[derive(Debug, Codec)]
pub struct UpdateRecipesEntry {
    pub recipe_type: Identifier,
    pub recipe_id: Identifier,
}

#[derive(Debug, Codec)]
pub struct UpdateRecipes {
    pub recipes: LengthPrefixArray<UpdateRecipesEntry>,
}

#[derive(Debug, Codec)]
pub struct UpdateTagsTag {
    pub tag_name: Identifier,
    pub values: LengthPrefixArray<VarInt>,
}

#[derive(Debug, Codec)]
pub struct UpdateTagsGroup {
    pub tag_type: Identifier,
    pub values: LengthPrefixArray<UpdateTagsTag>,
}

#[derive(Debug, Codec)]
pub struct UpdateTags {
    pub groups: LengthPrefixArray<UpdateTagsGroup>,
}

#[derive(Debug, Codec)]
pub struct EntityEvent {
    pub entity_id: i32,
    pub entity_status: i8,
}