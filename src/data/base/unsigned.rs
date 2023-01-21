use codec::{Codec, Result};
use std::io::{Read, Seek, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};

#[derive(Debug, PartialEq)]
pub struct PacketUByte(pub u8);

#[derive(Debug, PartialEq)]
pub struct PacketUShort(pub u16);

impl Codec for PacketUByte {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        Ok(PacketUByte(buf.read_u8()?))
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        let PacketUByte(value) = self;

        buf.write_u8(*value)?;

        Ok(())
    }
}

impl Codec for PacketUShort {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        Ok(PacketUShort(buf.read_u16::<NetworkEndian>()?))
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        let PacketUShort(value) = self;

        buf.write_u16::<NetworkEndian>(*value)?;

        Ok(())
    }
}

impl From<u8> for PacketUByte {
    fn from(value: u8) -> Self {
        PacketUByte(value)
    }
}

impl From<u16> for PacketUShort {
    fn from(value: u16) -> Self {
        PacketUShort(value)
    }
}