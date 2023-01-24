use crate::data::codec::{Codec, Result};
use std::io::{Read, Seek, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};

impl Codec for u8 {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        buf.read_u8()
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        buf.write_u8(*self)
    }
}

impl Codec for u16 {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        buf.read_u16::<NetworkEndian>()
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        buf.write_u16::<NetworkEndian>(*self)
    }
}

impl Codec for u32 {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        buf.read_u32::<NetworkEndian>()
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        buf.write_u32::<NetworkEndian>(*self)
    }
}

impl Codec for u64 {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        buf.read_u64::<NetworkEndian>()
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        buf.write_u64::<NetworkEndian>(*self)
    }
}

impl Codec for u128 {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        buf.read_u128::<NetworkEndian>()
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        buf.write_u128::<NetworkEndian>(*self)
    }
}