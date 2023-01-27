use crate::data::codec::{Codec, Result, Error, ErrorKind};
use std::io::{Read, Write, Seek};
use super::VarInt;

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub namespace: String,
    pub value: String,
}

impl Codec for String {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        let VarInt(length) = VarInt::decode(buf)?;

        let mut value = vec![0u8; length as usize];
        buf.read_exact(&mut value)?;

        Ok(String::from_utf8(value).map_err(|_| Error::from(ErrorKind::InvalidData))?)
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        VarInt::encode(&VarInt(self.len() as i32), buf)?;

        buf.write(self.as_bytes())?;

        Ok(())
    }
}

impl Codec for Identifier {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        let entire = String::decode(buf)?;

        let (namespace, value) = if entire.contains(":") {
            let mut split = entire.split(":");

            //If string contains ":" then it must be able to split
            (split.next().unwrap().to_string(), split.next().unwrap().to_string())
        } else {
            ("minecraft".to_string(), entire)
        };

        Ok(Self { namespace, value })
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        let value = [&self.namespace, ":", &self.value].concat();

        VarInt::encode(&VarInt(value.len() as i32), buf)?;

        buf.write(value.as_bytes())?;

        Ok(())
    }
}