use crate::codec_data::codec::{Codec, Result, Error, ErrorKind};
use std::io::{Read, Seek, Write};
use byteorder::{ReadBytesExt, WriteBytesExt};

impl Codec for bool {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        match buf.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::from(ErrorKind::InvalidData)),
        }
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        buf.write_u8(*self as u8)
    }
}