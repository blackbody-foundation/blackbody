/*
    .. + mbuf.rs + ..

    Copyright (C) 2021 Hwakyeom Kim(=just-do-halee)

    BlackBody is free software: you can redistribute it and/or modify
    it under the terms of the GNU Lesser General Public License as
    published by the Free Software Foundation, either version 3 of the
    License, or (at your option) any later version.

    BlackBody is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU Lesser General Public License for more details.

    You should have received a copy of the GNU Lesser General Public License
    along with BlackBody. If not, see <http://www.gnu.org/licenses/>.

*/

//! smart memorized buf

use std::io;

use super::CHUNK_SIZE;

#[derive(Debug, Clone)]
pub struct MBuf {
    pub buf: [u8; CHUNK_SIZE],
    pos: u64,
    len: usize,
}
impl MBuf {
    pub fn new(pos: u64) -> Self {
        Self {
            buf: [0; CHUNK_SIZE],
            pos,
            len: 0,
        }
    }
    pub fn add_num_process(&mut self, num: usize) {
        self.len = num;
        self.pos += num as u64;
    }
    pub fn set_buf_from(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        let src_len = buf.len();
        if src_len > CHUNK_SIZE {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "input size overflow.",
            ))
        } else {
            for (i, byte) in buf.iter().enumerate() {
                self.buf[i] = *byte;
            }
            self.len = src_len;
            Ok(self.len)
        }
    }
    pub fn reset(&mut self, pos: u64) {
        self.pos = pos;
        self.len = 0;
    }
    pub fn pos(&self) -> u64 {
        self.pos
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn set_len(&mut self, len: usize) {
        self.len = len;
    }
    pub fn get_slice(&self) -> &[u8] {
        &self.buf[..self.len]
    }
    pub fn get_mut_slice(&mut self) -> &mut [u8] {
        &mut self.buf[..self.len]
    }
}

impl Default for MBuf {
    fn default() -> Self {
        Self {
            buf: [0; CHUNK_SIZE],
            pos: 0,
            len: 0,
        }
    }
}
