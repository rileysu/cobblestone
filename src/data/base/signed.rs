use crate::data::codec::{Codec, Result};
use std::io::{Read, Seek, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};

impl Codec for i8 {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        buf.read_i8()
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        buf.write_i8(*self)
    }
}

impl Codec for i16 {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        buf.read_i16::<NetworkEndian>()
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        buf.write_i16::<NetworkEndian>(*self)
    }
}

impl Codec for i32 {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        buf.read_i32::<NetworkEndian>()
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        buf.write_i32::<NetworkEndian>(*self)
    }
}

impl Codec for i64 {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        buf.read_i64::<NetworkEndian>()
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        buf.write_i64::<NetworkEndian>(*self)
    }
}

impl Codec for i128 {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        buf.read_i128::<NetworkEndian>()
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        buf.write_i128::<NetworkEndian>(*self)
    }
}