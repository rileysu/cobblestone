use codec::{Codec, Result};
use std::io::{Read, Seek, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};

#[derive(Debug, PartialEq)]
pub struct Uuid(pub u128);

impl Codec for Uuid {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        Ok(Uuid(buf.read_u128::<NetworkEndian>()?))
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        let Uuid(value) = self;

        buf.write_u128::<NetworkEndian>(*value)?;

        Ok(())
    }
}