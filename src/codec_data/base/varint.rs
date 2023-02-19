use crate::codec_data::codec::{Codec, Result, Error, ErrorKind};
use std::io::{Read, Seek, Write};
use byteorder::{ReadBytesExt, WriteBytesExt};

const CONTINUE_BITS: u8 = 0x80;
const DATA_BITS: u8 = 0x7F;

#[derive(Debug, PartialEq)]
pub struct VarInt(pub i32);

impl Codec for VarInt {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        let mut out = 0u32; //Unsigned for logical bit operations
        
        for pos in (0..32).step_by(7) {
            let val = buf.read_u8()?;

            out = out | (((val & DATA_BITS) as u32) << pos);

            if val & CONTINUE_BITS == 0 {
                break;
            }
        }

        Ok(VarInt(out as i32))
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()>{
        let VarInt(val) = *self;
        let mut val = val as u32;
        
        for _ in (0..32).step_by(7) {
            if (val & !(DATA_BITS as u32)) == 0 {
                buf.write_u8(val as u8)?;
                return Ok(())
            }

            buf.write_u8(((val as u8) & DATA_BITS) | CONTINUE_BITS)?;

            val = val >> 7;
        }

        Err(Error::from(ErrorKind::InvalidData))
    }
}

#[cfg(test)]
mod test{
    use std::io::Cursor;

    use super::*;

    #[test]
    fn varint_examples() {
        let examples = [
            (VarInt(0), [0x00u8].as_slice()),
            (VarInt(1), [0x01u8].as_slice()),
            (VarInt(2), [0x02u8].as_slice()),
            (VarInt(127), [0x7fu8].as_slice()),
            (VarInt(128), [0x80u8, 0x01u8].as_slice()),
            (VarInt(255), [0xffu8, 0x01u8].as_slice()),
            (VarInt(-1), [0xffu8, 0xffu8, 0xffu8, 0xffu8, 0x0fu8].as_slice()),
            (VarInt(2147483647), [0xffu8, 0xffu8, 0xffu8, 0xffu8, 0x07u8].as_slice()),
            (VarInt(-2147483648), [0x80u8, 0x80u8, 0x80u8, 0x80u8, 0x08u8].as_slice()),
        ];

        examples.iter().for_each(|(real, bin)| {
            assert_eq!(*real, VarInt::decode(&mut Cursor::new(bin)).unwrap());
        });

        examples.iter().for_each(|(real, bin)| {
            let mut buf = Cursor::new(Vec::new());
            VarInt::encode(real, &mut buf).unwrap();

            assert_eq!(buf.get_ref().as_slice(), *bin);
        });
    }
}