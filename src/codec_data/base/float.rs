use crate::codec_data::codec::{Codec, Result};
use std::io::{Read, Seek, Write};
use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};

impl Codec for f32 {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        buf.read_f32::<NetworkEndian>()
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        buf.write_f32::<NetworkEndian>(*self)
    }
}

impl Codec for f64 {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        buf.read_f64::<NetworkEndian>()
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        buf.write_f64::<NetworkEndian>(*self)
    }
}
