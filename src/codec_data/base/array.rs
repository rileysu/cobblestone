use crate::codec_data::codec::{Codec, Result};
use std::io::{Read, Seek, Write};
use super::VarInt;

#[derive(Debug, PartialEq)]
pub struct LengthPrefixArray<T: Codec>(pub Vec<T>);

impl<T: Codec> Codec for LengthPrefixArray<T> {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        let VarInt(length) = VarInt::decode(buf)?;

        let mut elements: Vec<T> = Vec::new();
        for _ in 0..length {
            elements.push(T::decode(buf)?);
        }

        Ok(LengthPrefixArray(elements))
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        let LengthPrefixArray(value) = self;

        VarInt(value.len() as i32).encode(buf)?;
        
        for element in self.0.iter() {
            element.encode(buf)?;
        }

        Ok(())
    }
}