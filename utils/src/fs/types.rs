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
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};

pub type Ptr = Box<File>;
pub type Header = Box<dyn HeaderTrait>;

pub trait HeaderTrait: std::fmt::Debug {
    /// return value is bytes length of successfully filled buffer.
    fn read(&mut self, ptr: &mut Ptr) -> Result<usize>;
    /// return value is bytes length of successfully filled buffer.
    fn write(&mut self, ptr: &mut Ptr) -> Result<usize>;
}

pub const CHUNK_SIZE: usize = 8 * 1024;

#[derive(Debug)]
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
    pub fn flush_header(&mut self) -> Result<()> {
        let ptr = &mut self.ptr;
        let header_size = self.header.write(ptr)?;
        self.header_size = header_size as u64;
        Ok(())
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
        Self::err_tunnel(
            self.ptr
                .seek(SeekFrom::Current(pos + self.header_size as i64)),
        )
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
    pub fn write_cursoring(&mut self, buf: &[u8], pos: u64) -> Result<()> {
        self.set_cursor(pos)?;
        Self::err_tunnel(self.ptr.write_all(buf))
    }
    pub fn insert_special(&mut self, buf: &[u8], pos: u64, stop_pos: u64) -> Result<()> {
        let mut reader = BufReader::new(self.ptr.try_clone()?);
        let mut writer = BufWriter::new(self.ptr.try_clone()?);
        let mut buffer_1 = [0_u8; CHUNK_SIZE];
        let mut buffer_2 = [0_u8; CHUNK_SIZE];

        let mut checked_pos_1 = pos;
        let mut checked_pos_2 = pos;
        let mut num_read_1 = 1;
        let mut num_read_2 = 1;

        self.set_cursor(checked_pos_1)?;

        num_read_1 = Self::err_tunnel(reader.read(&mut buffer_1))?;

        checked_pos_1 += num_read_1 as u64;

        self.set_cursor(checked_pos_2)?;

        Self::err_tunnel(writer.write_all(&buf))?;

        checked_pos_2 += buf.len() as u64;

        let mut rot = true;

        while (num_read_1 > 0 && num_read_2 > 0) && (checked_pos_1 <= stop_pos || stop_pos != 0) {
            self.set_cursor(checked_pos_1)?;
            num_read_2 = Self::err_tunnel(reader.read(&mut if rot { buffer_2 } else { buffer_1 }))?;

            if num_read_2 == 0 {
                Self::err_tunnel(writer.write_all(if rot {
                    &buffer_1[..num_read_1]
                } else {
                    &buffer_2[..num_read_2]
                }))?;
                break;
            }

            checked_pos_2 += num_read_2 as u64;

            self.set_cursor(checked_pos_1)?;

            let diff = |num_read: usize| (checked_pos_1 + num_read as u64) - stop_pos;

            Self::err_tunnel(writer.write_all(if rot {
                let dif = diff(num_read_1);
                if dif > 0 {
                    num_read_1.overflowing_sub(dif as usize);
                    checked_pos_1 = stop_pos;
                }
                &buffer_1[..num_read_1]
            } else {
                num_read_1 = num_read_2;

                let dif = diff(num_read_1);
                if dif > 0 {
                    num_read_1.overflowing_sub(dif as usize);
                    checked_pos_1 = stop_pos;
                }
                &buffer_2[..num_read_1]
            }))?;

            checked_pos_1 += num_read_1 as u64;
            rot = !rot;
        }
        Ok(())
    }
    fn err_tunnel<E>(io_e: std::io::Result<E>) -> Result<E> {
        errors::handle_io_error(io_e)
    }
}
