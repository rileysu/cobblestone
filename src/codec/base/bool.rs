use codec::{Codec, Result, Error, ErrorKind};
use std::io::{Read, Seek, Write};
use byteorder::{ReadBytesExt, WriteBytesExt};

#[derive(Debug, PartialEq)]
pub struct PacketBool(pub bool);

impl Codec for PacketBool {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        Ok(PacketBool(match buf.read_u8()? {
            0 => false,
            1 => true,
            _ => return Err(Error::from(ErrorKind::InvalidData)),
        }))
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        let PacketBool(value) = self;

        buf.write_u8(match value {
            false => 0,
            true => 1,
        })?;

        Ok(())
    }
}

impl From<bool> for PacketBool {
    fn from(value: bool) -> Self {
        PacketBool(value)
    }
}

impl Into<bool> for PacketBool {
    fn into(self) -> bool {
        self.0
    }
}