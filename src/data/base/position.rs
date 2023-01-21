use codec::{Codec, Result};
use std::io::{Read, Seek, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};

#[derive(Debug, PartialEq)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Codec for Position {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        let value = buf.read_i64::<NetworkEndian>()?;

        Ok(Self {
            x: (value >> 38) as i32,
            y: (value << 52 >> 52) as i32,
            z: (value << 26 >> 38) as i32,
        })
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        buf.write_i64::<NetworkEndian>(self.x as i64 & 0x3FFFFFF << 38 | self.z as i64 & 0x3fFFFFFF << 12 | self.y as i64 & 0xFFF)?;

        Ok(())
    }
}