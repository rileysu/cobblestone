use crate::codec_data::codec::{Codec, Result};
use std::io::{Read, Seek, Write};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Uuid(pub u128);

impl Codec for Uuid {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        Ok(Uuid(u128::decode(buf)?))
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        self.0.encode(buf)
    }
}