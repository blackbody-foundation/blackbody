use std::io;

use super::types::{Reader, Writer};

pub trait Header {
    fn as_bytes(&self) -> &[u8];
    fn compare(&self, reader: Reader) -> io::Result<Reader>;
    fn write(&self, writer: Writer) -> io::Result<Writer>;
}
