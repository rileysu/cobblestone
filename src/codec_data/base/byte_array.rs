use crate::codec_data::codec::{Codec, Result};
use std::io::{Read, Write, Seek};
use super::VarInt;

#[derive(Debug, PartialEq)]
pub struct LengthPrefixByteArray(pub Vec<u8>);

impl Codec for LengthPrefixByteArray {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        let VarInt(length) = VarInt::decode(buf)?;

        let mut bytes = vec![0u8; length as usize];
        buf.read_exact(&mut bytes)?;

        Ok(LengthPrefixByteArray(bytes))
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        let LengthPrefixByteArray(value) = self;

        VarInt(value.len() as i32).encode(buf)?;
        buf.write(value)?;

        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct ConsumingByteArray(pub Vec<u8>);

impl Codec for ConsumingByteArray {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        let mut bytes = Vec::new();
        buf.read_to_end(&mut bytes)?;

        Ok(ConsumingByteArray(bytes))
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        let ConsumingByteArray(value) = self;

        buf.write(value)?;

        Ok(())
    }
}