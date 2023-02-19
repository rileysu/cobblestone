use crate::codec_data::codec::{Codec, Result, Error, ErrorKind};
use std::{io::{Read, Seek, Write}, collections::HashMap};
use byteorder::{ReadBytesExt, WriteBytesExt, NetworkEndian};

pub const NBT_END_ID: u8 = 0;
pub const NBT_BYTE_ID: u8 = 1;
pub const NBT_SHORT_ID: u8 = 2;
pub const NBT_INT_ID: u8 = 3;
pub const NBT_LONG_ID: u8 = 4;
pub const NBT_FLOAT_ID: u8 = 5;
pub const NBT_DOUBLE_ID: u8 = 6;
pub const NBT_BYTE_ARRAY_ID: u8 = 7;
pub const NBT_STRING_ID: u8 = 8;
pub const NBT_LIST_ID: u8 = 9;
pub const NBT_COMPOUND_ID: u8 = 10;
pub const NBT_INT_ARRAY_ID: u8 = 11;
pub const NBT_LONG_ARRAY_ID: u8 = 12;
const NBT_ROOT_COMPOUND_ID: u8 = u8::MAX;

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
    List(u8, Vec<NBTValue>),
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

            Ok(NBTValue::List(elements_id, list))
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

                if id == NBT_ROOT_COMPOUND_ID {
                    return Ok(NBTValue::Compound(compound));
                }
            }
        },
        NBT_ROOT_COMPOUND_ID => {
            let element_id = buf.read_u8()?;

            let key = match decode_with_id(buf, NBT_STRING_ID)? {
                NBTValue::String(key) => key,
                _ => return Err(Error::from(ErrorKind::InvalidData)),
            };

            let value = decode_with_id(buf, element_id)?;

            return Ok(NBTValue::Compound(HashMap::from([(key, value)])));
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

fn encode_with_root_handler(value: &NBTValue, buf: &mut impl Write, is_root: bool) -> Result<()> {
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
            NBTValue::List(_, _) => NBT_LIST_ID,
            NBTValue::Compound(_) => NBT_COMPOUND_ID,
            NBTValue::IntArray(_) => NBT_INT_ARRAY_ID,
            NBTValue::LongArray(_) => NBT_LONG_ARRAY_ID,
        }
    }

    match value {
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
        NBTValue::List(id, value) => {
            buf.write_u8(*id)?;

            buf.write_i32::<NetworkEndian>(value.len() as i32)?;

            for element in value {
                encode_with_root_handler(element, buf, false)?;
            }
        },
        NBTValue::Compound(value) => {
            //Keys should be sorted in order to return consistent NBT values for unit tests
            //Alternatively one could just use a BTreeMap but this will impact interpreting NBT values
            //This can be changed later if it causes too many issues
            let mut values: Vec<(&String, &NBTValue)> = value.iter().collect();
            values.sort_by_key(|(key, _)| *key);

            for (key, value) in values {
                buf.write_u8(get_id(value))?;

                buf.write_u16::<NetworkEndian>(key.len() as u16)?;
                buf.write_all(key.as_bytes())?;

                encode_with_root_handler(value, buf, false)?;
            }
            if !is_root {
                buf.write_u8(NBT_END_ID)?;
            }
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

impl Codec for NBTValue {
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self> {
        //Implicit compound at the start
        Ok(decode_with_id(buf, NBT_ROOT_COMPOUND_ID)?)
    }

    fn encode(&self, buf: &mut impl Write) -> Result<()> {
        Ok(encode_with_root_handler(self, buf, true)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use std::collections::HashMap;
    use std::io::{Cursor};
    use std::fs::File;

    fn compare_file_to_mat(example_path: &str, mat: &NBTValue) {
        let mut file = File::open(example_path).unwrap();

        let bytes_decoded = NBTValue::decode(&mut file).unwrap();

        assert_eq!(bytes_decoded, *mat);

        let mut mat_buf_curs: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        mat.encode(&mut mat_buf_curs).unwrap();
        let mat_buf = mat_buf_curs.into_inner();

        file.rewind().unwrap();
        let mut file_buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut file_buf).unwrap();

        assert_eq!(mat_buf, file_buf);
    }

    #[test]
    fn byte_example() {
        let mat = NBTValue::Compound(HashMap::from([
            ("byte".into(), NBTValue::Byte(i8::MIN))
        ]));

        compare_file_to_mat("resources/nbt_examples/byte.nbt", &mat);
    }

    #[test]
    fn test_short() {
        let mat = NBTValue::Compound(HashMap::from([
            ("short".into(), NBTValue::Short(i16::MIN))
        ]));

        compare_file_to_mat("resources/nbt_examples/short.nbt", &mat);
    }

    #[test]
    fn test_int() {
        let mat = NBTValue::Compound(HashMap::from([
            ("int".into(), NBTValue::Int(i32::MIN))
        ]));

        compare_file_to_mat("resources/nbt_examples/int.nbt", &mat);
    }

    #[test]
    fn test_long() {
        let mat = NBTValue::Compound(HashMap::from([
            ("long".into(), NBTValue::Long(i64::MIN))
        ]));

        compare_file_to_mat("resources/nbt_examples/long.nbt", &mat);
    }

    #[test]
    fn test_float() {
        let mat = NBTValue::Compound(HashMap::from([
            ("float".into(), NBTValue::Float(0.2))
        ]));

        compare_file_to_mat("resources/nbt_examples/float.nbt", &mat);
    }

    #[test]
    fn test_double() {
        let mat = NBTValue::Compound(HashMap::from([
            ("double".into(), NBTValue::Double(0.2))
        ]));

        compare_file_to_mat("resources/nbt_examples/double.nbt", &mat);
    }

    #[test]
    fn test_byte_array() {
        let mat = NBTValue::Compound(HashMap::from([
            ("bytearray".into(), NBTValue::ByteArray(vec![0, i8::MIN, -1]))
        ]));

        compare_file_to_mat("resources/nbt_examples/bytearray.nbt", &mat);
    }

    #[test]
    fn test_string() {
        let mat = NBTValue::Compound(HashMap::from([
            ("string".into(), NBTValue::String("value! 😁🥺😬".into()))
        ]));

        compare_file_to_mat("resources/nbt_examples/string.nbt", &mat);
    }

    #[test]
    fn test_list() {
        let mat = NBTValue::Compound(HashMap::from([
            ("list".into(), NBTValue::List(NBT_DOUBLE_ID, Vec::from([0.0, 0.1, 0.2, 0.3, 0.4, 0.5].map(|x| NBTValue::Double(x)))))
        ]));

        compare_file_to_mat("resources/nbt_examples/list.nbt", &mat);
    }

    #[test]
    fn test_compound() {
        let mat = NBTValue::Compound(HashMap::from([
            ("compound".into(), NBTValue::Compound(HashMap::from([
                ("aaa".into(), NBTValue::Int(-100)),
                ("bbb".into(), NBTValue::Long(100)),
                ("ccc".into(), NBTValue::Float(0.2)),
                ("ddd".into(), NBTValue::List(NBT_STRING_ID, Vec::from(["aaa", "bbb", "ccc", "ddd"].map(|x| NBTValue::String(x.into()))))),
            ])))
        ]));

        compare_file_to_mat("resources/nbt_examples/compound.nbt", &mat);
    }

    #[test]
    fn test_int_array() {
        let mat = NBTValue::Compound(HashMap::from([
            ("intarray".into(), NBTValue::IntArray(vec![-2147483648, -1, 1, 2147483647]))
        ]));

        compare_file_to_mat("resources/nbt_examples/intarray.nbt", &mat);
    }

    #[test]
    fn test_long_array() {
        let mat = NBTValue::Compound(HashMap::from([
            ("longarray".into(), NBTValue::LongArray(vec![-9223372036854775808, -1, 1, 9223372036854775807]))
        ]));

        compare_file_to_mat("resources/nbt_examples/longarray.nbt", &mat);
    }

    #[test]
    fn test_hello_world() {
        let mat = NBTValue::Compound(HashMap::from([
            ("hello world".into(), NBTValue::Compound(HashMap::from([
                ("name".into(), NBTValue::String("Bananrama".into())),
            ]))),
        ]));

        compare_file_to_mat("resources/nbt_examples/hello_world.nbt", &mat);
    }

    // #[test]
    // fn test_bigtest() {
    //     let byte_array_val = (0u64..1000u64).map(|x| ((x * x * 255 + x * 7) % 100) as i8).collect();

    //     let mat = NBTValue::Compound(HashMap::from([
    //         ("Level".into(), NBTValue::Compound(HashMap::from([
    //             ("shortTest".into(), NBTValue::Short(32767)),
    //             ("longTest".into(), NBTValue::Long(9223372036854775807)),
    //             ("floatTest".into(), NBTValue::Float(0.49823147)),
    //             ("stringTest".into(), NBTValue::String("HELLO WORLD THIS IS A TEST STRING ÅÄÖ!".into())),
    //             ("intTest".into(), NBTValue::Int(2147483647)),
    //             ("nested compound test".into(), NBTValue::Compound(HashMap::from([
    //                 ("ham".into(), NBTValue::Compound(HashMap::from([
    //                     ("name".into(), NBTValue::String("Hampus".into())),
    //                     ("value".into(), NBTValue::Float(0.75))
    //                 ]))),
    //                 ("egg".into(), NBTValue::Compound(HashMap::from([
    //                     ("name".into(), NBTValue::String("Eggbert".into())),
    //                     ("value".into(), NBTValue::Float(0.5))
    //                 ])))
    //             ]))),
    //             ("listTest (long)".into(), NBTValue::List(vec![
    //                 NBTValue::Long(11), 
    //                 NBTValue::Long(12), 
    //                 NBTValue::Long(13), 
    //                 NBTValue::Long(14), 
    //                 NBTValue::Long(15)
    //             ])),
    //             ("byteTest".into(), NBTValue::Byte(127)),
    //             ("listTest (compound)".into(), NBTValue::List(vec![
    //                 NBTValue::Compound(HashMap::from([
    //                     ("name".into(), NBTValue::String("Compound tag #0".into())),
    //                     ("created-on".into(), NBTValue::Long(1264099775885))
    //                 ])),
    //                 NBTValue::Compound(HashMap::from([
    //                     ("name".into(), NBTValue::String("Compound tag #1".into())),
    //                     ("created-on".into(), NBTValue::Long(1264099775885))
    //                 ]))
    //             ])),
    //             ("byteArrayTest (the first 1000 values of (n*n*255+n*7)%100, starting with n=0 (0, 62, 34, 16, 8, ...))".into(), NBTValue::ByteArray(byte_array_val)),
    //             ("doubleTest".into(), NBTValue::Double(0.4931287132182315))
    //         ])))
    //     ]));

    //     compare_file_to_mat("resources/nbt_examples/bigtest.nbt", &mat);
    // }
}