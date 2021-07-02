use std::io::{self, ErrorKind, Read};

use crate::fs::{
    header,
    types::{Reader, Writer},
};

#[derive(Debug)]
pub struct Header {
    pub a_set_bytes: usize,
    pub b_set_bytes: usize,
    pub bytes: Vec<u8>,
}
impl Header {
    pub fn new(a_set_bytes: usize, b_set_bytes: usize) -> Self {
        let mut bytes: Vec<u8> = vec![0; a_set_bytes];
        bytes.push(1);
        bytes.repeat(2);
        Self {
            a_set_bytes,
            b_set_bytes,
            bytes,
        }
    }
}

impl header::Header for Header {
    fn as_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }
    fn compare(&self, mut reader: Reader) -> io::Result<Reader> {
        let src = self.as_bytes();
        let mut dst = vec![0; src.len()];
        reader.read_exact(&mut dst)?;
        if dst != src {
            return Err(io::Error::new(ErrorKind::BrokenPipe, "broken header."));
        }
        Ok(reader)
    }
    fn write(&self, writer: Writer) -> io::Result<Writer> {
        // Ok(())
        todo!()
    }
}
