use codec::{Codec, Result};
use std::io::{Read, Seek, Write};
use super::PacketBool;

#[derive(Debug, PartialEq)]
pub enum PacketOption<T: Codec> {
    Some(T),
    None,
}

impl<T: Codec> Codec for PacketOption<T> {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        Ok(match PacketBool::decode(buf)?.into() {
            true => PacketOption::Some(T::decode(buf)?),
            false => PacketOption::None,
        })
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        match self {
            PacketOption::Some(value) => {
                PacketBool::encode(&false.into(), buf)?;
                value.encode(buf)?;
            }
            PacketOption::None => PacketBool::encode(&false.into(), buf)?,
        }

        Ok(())
    }
}