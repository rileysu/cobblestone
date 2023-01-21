use codec_derive::Codec;
use codec::{Codec, Result, Error, ErrorKind};
use crate::data::base::*;
use std::io::{Seek, Read, Write};

#[derive(Debug)]
pub enum InboundPlay {
}

impl Codec for InboundPlay {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        todo!()
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        todo!()
    }
}

#[derive(Debug)]
pub enum OutboundPlay {
    Login(Login),
}

impl Codec for OutboundPlay {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        unimplemented!()
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        match self {
            OutboundPlay::Login(login) => {
                VarInt::encode(&VarInt(0x24), buf)?;
                login.encode(buf)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Codec)]
pub struct Login {
    entity_id: PacketInt,
    is_hardcore: PacketBool,
    gamemode: PacketUByte,
    previous_gamemode: PacketUByte,
    dimension_names: LengthPrefixArray<PacketString>,
    registry_codec: NBTValue,
    dimension_type: Identifier,
    dimension_name: Identifier,
    hashed_seed: PacketLong,
    max_players: VarInt,
    view_distance: VarInt,
    simulation_distance: VarInt,
    reduced_debug_info: PacketBool,
    enabled_respawn_screen: PacketBool,
    is_debug: PacketBool,
    is_flat: PacketBool,
    has_death_location: PacketBool,
    death_dimension_name: Identifier,
    death_location: Position,
}