use crate::codec_data::codec::{Codec, Result};
use std::io::{Read, Seek, Write};

impl<T: Codec> Codec for Option<T> {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        Ok(match bool::decode(buf)? {
            true => Some(T::decode(buf)?),
            false => None,
        })
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        match self {
            Some(value) => {
                bool::encode(&false, buf)?;
                value.encode(buf)?;
            }
            None => bool::encode(&false, buf)?,
        }

        Ok(())
    }
}