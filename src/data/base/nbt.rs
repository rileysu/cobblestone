use codec::{Codec, Result, Error, ErrorKind};
use std::{io::{Read, Seek, Write}, collections::HashMap};
use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};

const NBT_END_ID: u8 = 0;
const NBT_BYTE_ID: u8 = 1;
const NBT_SHORT_ID: u8 = 2;
const NBT_INT_ID: u8 = 3;
const NBT_LONG_ID: u8 = 4;
const NBT_FLOAT_ID: u8 = 5;
const NBT_DOUBLE_ID: u8 = 6;
const NBT_BYTE_ARRAY_ID: u8 = 7;
const NBT_STRING_ID: u8 = 8;
const NBT_LIST_ID: u8 = 9;
const NBT_COMPOUND_ID: u8 = 10;
const NBT_INT_ARRAY_ID: u8 = 11;
const NBT_LONG_ARRAY_ID: u8 = 12;

#[derive(Debug, PartialEq)]
pub enum NBTValue {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<i8>),
    String(String),
    List(Vec<NBTValue>),
    Compound(HashMap<String, NBTValue>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

fn decode_with_id(buf: &mut (impl Read + Seek), id: u8) -> Result<NBTValue> {
    match id {
        NBT_BYTE_ID => Ok(NBTValue::Byte(buf.read_i8()?)),
        NBT_SHORT_ID => Ok(NBTValue::Short(buf.read_i16::<NetworkEndian>()?)),
        NBT_INT_ID => Ok(NBTValue::Int(buf.read_i32::<NetworkEndian>()?)),
        NBT_LONG_ID => Ok(NBTValue::Long(buf.read_i64::<NetworkEndian>()?)),
        NBT_FLOAT_ID => Ok(NBTValue::Float(buf.read_f32::<NetworkEndian>()?)),
        NBT_DOUBLE_ID => Ok(NBTValue::Double(buf.read_f64::<NetworkEndian>()?)),
        NBT_BYTE_ARRAY_ID => {
            let length = buf.read_i32::<NetworkEndian>()?;

            let mut data_buf: Vec<i8> = Vec::new();
            for _ in 0..length {
                data_buf.push(buf.read_i8()?);
            }

            Ok(NBTValue::ByteArray(data_buf))
        },
        NBT_STRING_ID => {
            let length = buf.read_u16::<NetworkEndian>()?;

            let mut string_buf = vec![0u8; length as usize];
            buf.read_exact(&mut string_buf)?;
        
            Ok(NBTValue::String(String::from_utf8(string_buf).map_err(|other_err| {
                Error::new(ErrorKind::InvalidData, other_err.to_string())
            })?))
        },
        NBT_LIST_ID => {
            let elements_id = buf.read_u8()?;
            let length = buf.read_i32::<NetworkEndian>()?;

            let mut list: Vec<NBTValue> = Vec::new();
            for _ in 0..length {
                list.push(decode_with_id(buf, elements_id)?);
            }

            Ok(NBTValue::List(list))
        },
        NBT_COMPOUND_ID => {
            let mut compound: HashMap<String, NBTValue> = HashMap::new();

            loop {
                let element_id = buf.read_u8()?;

                if element_id == 0 {
                    return Ok(NBTValue::Compound(compound));
                }

                let key = match decode_with_id(buf, NBT_STRING_ID)? {
                    NBTValue::String(key) => key,
                    _ => return Err(Error::from(ErrorKind::InvalidData)),
                };

                let value = decode_with_id(buf, element_id)?;

                compound.insert(key, value);
            }
        },
        NBT_INT_ARRAY_ID => {
            let length = buf.read_i32::<NetworkEndian>()?;

            let mut data_buf: Vec<i32> = Vec::new();
            for _ in 0..length {
                data_buf.push(buf.read_i32::<NetworkEndian>()?);
            }

            Ok(NBTValue::IntArray(data_buf))
        },
        NBT_LONG_ARRAY_ID => {
            let length = buf.read_i32::<NetworkEndian>()?;

            let mut data_buf: Vec<i64> = Vec::new();
            for _ in 0..length {
                data_buf.push(buf.read_i64::<NetworkEndian>()?);
            }

            Ok(NBTValue::LongArray(data_buf))
        }
        _ => Err(Error::from(ErrorKind::InvalidData)),
    }
}

impl Codec for NBTValue {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        //Implicit compound at the start
        Ok(decode_with_id(buf, NBT_COMPOUND_ID)?)
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        fn get_id(nbt: &NBTValue) -> u8 {
            match nbt {
                NBTValue::Byte(_) => NBT_BYTE_ID,
                NBTValue::Short(_) => NBT_SHORT_ID,
                NBTValue::Int(_) => NBT_INT_ID,
                NBTValue::Long(_) => NBT_LONG_ID,
                NBTValue::Float(_) => NBT_FLOAT_ID,
                NBTValue::Double(_) => NBT_DOUBLE_ID,
                NBTValue::ByteArray(_) => NBT_BYTE_ARRAY_ID,
                NBTValue::String(_) => NBT_STRING_ID,
                NBTValue::List(_) => NBT_LIST_ID,
                NBTValue::Compound(_) => NBT_COMPOUND_ID,
                NBTValue::IntArray(_) => NBT_INT_ARRAY_ID,
                NBTValue::LongArray(_) => NBT_LONG_ARRAY_ID,
            }
        }

        match self {
            NBTValue::Byte(value) => buf.write_i8(*value)?,
            NBTValue::Short(value) => buf.write_i16::<NetworkEndian>(*value)?,
            NBTValue::Int(value) => buf.write_i32::<NetworkEndian>(*value)?,
            NBTValue::Long(value) => buf.write_i64::<NetworkEndian>(*value)?,
            NBTValue::Float(value) => buf.write_f32::<NetworkEndian>(*value)?,
            NBTValue::Double(value) => buf.write_f64::<NetworkEndian>(*value)?,
            NBTValue::ByteArray(value) => {
                buf.write_i32::<NetworkEndian>(value.len() as i32)?;
                
                for byte in value {
                    buf.write_i8(*byte)?;
                }
            },
            NBTValue::String(value) => {
                buf.write_u16::<NetworkEndian>(value.len() as u16)?;

                buf.write_all(value.as_bytes())?;
            },
            NBTValue::List(value) => {
                buf.write_u8(get_id(&value[0]))?;

                buf.write_i32::<NetworkEndian>(value.len() as i32)?;

                for element in value {
                    element.encode(buf)?;
                }
            },
            NBTValue::Compound(value) => {
                for (key, value) in value {
                    buf.write_u8(get_id(value))?;

                    buf.write_u16::<NetworkEndian>(key.len() as u16)?;
                    buf.write_all(key.as_bytes())?;

                    value.encode(buf)?;
                }

                buf.write_u8(NBT_END_ID)?;
            },
            NBTValue::IntArray(value) => {
                buf.write_i32::<NetworkEndian>(value.len() as i32)?;
                
                for int in value {
                    buf.write_i32::<NetworkEndian>(*int)?;
                }
            },
            NBTValue::LongArray(value) => {
                buf.write_i32::<NetworkEndian>(value.len() as i32)?;
                
                for long in value {
                    buf.write_i64::<NetworkEndian>(*long)?;
                }
            },
        };

        Ok(())
    }
}