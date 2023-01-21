use codec::{Codec, Result};
use std::io::{Read, Seek, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};

#[derive(Debug, PartialEq)]
pub struct PacketInt(pub i32);

#[derive(Debug, PartialEq)]
pub struct PacketLong(pub i64);

impl Codec for PacketInt {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        Ok(PacketInt(buf.read_i32::<NetworkEndian>()?))
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        let PacketInt(value) = self;

        buf.write_i32::<NetworkEndian>(*value)?;

        Ok(())
    }
}

impl Codec for PacketLong {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        Ok(PacketLong(buf.read_i64::<NetworkEndian>()?))
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        let PacketLong(value) = self;

        buf.write_i64::<NetworkEndian>(*value)?;

        Ok(())
    }
}

impl From<i32> for PacketInt {
    fn from(value: i32) -> Self {
        PacketInt(value)
    }
}

impl From<i64> for PacketLong {
    fn from(value: i64) -> Self {
        PacketLong(value)
    }
}