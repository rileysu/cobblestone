use codec::{Codec, Result, Error, ErrorKind};
use std::io::{Read, Write, Seek};
use super::VarInt;

#[derive(Debug, PartialEq)]
pub struct PacketString(pub String);

#[derive(Debug, PartialEq)]
pub struct Identifier {
    namespace: String,
    value: String,
}

impl Codec for PacketString {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        let VarInt(length) = VarInt::decode(buf)?;

        let mut value = vec![0u8; length as usize];
        buf.read_exact(&mut value)?;

        Ok(Self(String::from_utf8(value).map_err(|_| Error::from(ErrorKind::InvalidData))?))
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        let PacketString(value) = self;

        VarInt::encode(&VarInt(value.len() as i32), buf)?;

        buf.write(value.as_bytes())?;

        Ok(())
    }
}

impl Codec for Identifier {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        let VarInt(length) = VarInt::decode(buf)?;

        let mut value = vec![0u8; length as usize];
        buf.read_exact(&mut value)?;

        let entire = String::from_utf8(value).map_err(|other_err| Error::new(ErrorKind::InvalidData, other_err.to_string()))?;

        let (namespace, value) = if entire.contains(":") {
            let mut split = entire.split(":");

            //If string contains ":" then it must be able to split
            (split.nth(0).unwrap().to_string(), split.nth(1).unwrap().to_string())
        } else {
            ("minecraft".to_string(), entire)
        };

        Ok(Self { namespace, value})
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        let value = match self {
            Identifier { namespace, value} if namespace == "minecraft" => value.clone(),
            Identifier { namespace, value } => [namespace, ":", value].concat(),
        };

        VarInt::encode(&VarInt(value.len() as i32), buf)?;

        buf.write(value.as_bytes())?;

        Ok(())
    }
}

impl From<String> for PacketString {
    fn from(value: String) -> Self {
        PacketString(value)
    }
}