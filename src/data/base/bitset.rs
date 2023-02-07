use crate::data::codec::{Codec, Result};
use std::io::{Read, Seek, Write};

use super::{VarInt};

#[derive(Debug, PartialEq)]
pub struct BitSet(pub Vec<i64>);

impl Codec for BitSet {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        let length = VarInt::decode(buf)?;

        let mut out = Vec::<i64>::new();
        for _ in 0..length.0 {
            out.push(i64::decode(buf)?);
        }

        Ok(Self(out))
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        VarInt(self.0.len() as i32).encode(buf)?;

        for elem in self.0 {
            elem.encode(buf)?;
        }

        Ok(())
    }
}

