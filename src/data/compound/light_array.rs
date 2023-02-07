use crate::data::codec::{Codec, Result};
use std::io::{Read, Seek, Write};
use byteorder::{WriteBytesExt, NetworkEndian};
use crate::data::base::VarInt;

#[derive(Debug, PartialEq)]
pub struct LightArray([i8; 2048]);

impl Codec for LightArray {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        buf.seek(std::io::SeekFrom::Current(4))?;

        let mut light_bytes = [0u8; 2048];
        buf.read_exact(light_bytes.as_mut_slice())?;
        
        unsafe {
            Ok(Self(std::mem::transmute::<[u8; 2048], [i8; 2048]>(light_bytes)))
        }
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        VarInt(2048).encode(buf);

        let light_bytes = unsafe {
            std::mem::transmute::<[i8; 2048], [u8; 2048]>(self.0)
        };

        buf.write_all(light_bytes.as_slice());

        Ok(())
    }
}