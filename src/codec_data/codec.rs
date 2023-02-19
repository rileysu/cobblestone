use std::io::{Read, Write, Seek};

pub use std::io::{Error, ErrorKind};

pub type Result<T> = std::result::Result<T, Error>;

pub trait Codec
where
    Self: Sized,
{
    fn decode(buf: &mut (impl Read + Seek)) -> Result<Self>;
    fn encode(&self, buf: &mut impl Write) -> Result<()>;
}