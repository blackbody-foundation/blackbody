/*
    .. + types.rs + ..

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

use crate::system::*;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

pub type Ptr = Box<File>;
pub type Header = Box<dyn HeaderTrait>;

pub trait HeaderTrait: std::fmt::Debug {
    fn read(&mut self, ptr: &mut Ptr) -> Result<usize>;
    fn write(&mut self, ptr: &mut Ptr) -> Result<usize>;
}

pub struct FM<T> {
    ptr: Ptr,
    pub header: Box<T>,
    pub header_size: u64,
}
impl<T: HeaderTrait> FM<T> {
    pub fn new(path: &'static str, mut header: Box<T>) -> Result<Self> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(path)?;

        let mut ptr = Box::new(file);

        let header_size = header.read(&mut ptr)? as u64;

        Ok(Self {
            ptr,
            header,
            header_size,
        })
    }
    pub fn is_eof(&mut self) -> Result<bool> {
        if 0 == self.ptr.read(&mut [0u8; 1])? {
            Ok(true)
        } else {
            self.set_cursor_relative(-1)?;
            Ok(false)
        }
    }
    pub fn set_cursor_relative(&mut self, pos: i64) -> Result<u64> {
        Self::err_tunnel(self.ptr.seek(SeekFrom::Current(pos)))
    }
    pub fn set_cursor_general(&mut self, pos: u64) -> Result<u64> {
        Self::err_tunnel(self.ptr.seek(SeekFrom::Start(pos)))
    }
    pub fn set_cursor(&mut self, pos: u64) -> Result<u64> {
        Self::err_tunnel(self.ptr.seek(SeekFrom::Start(pos + self.header_size)))
    }
    pub fn read_general(&mut self, buf: &mut [u8]) -> Result<usize> {
        Self::err_tunnel(self.ptr.read(buf))
    }
    pub fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        Self::err_tunnel(self.ptr.read_exact(buf))
    }
    pub fn read_cursoring(&mut self, buf: &mut [u8], pos: u64) -> Result<()> {
        self.set_cursor(pos)?;
        Self::err_tunnel(self.ptr.read_exact(buf))
    }
    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        Self::err_tunnel(self.ptr.write_all(buf))
    }
    fn err_tunnel<E>(io_e: std::io::Result<E>) -> Result<E> {
        errors::handle_io_error(io_e)
    }
}
