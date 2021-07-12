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

pub type Reader = Box<File>;
pub type Writer = Box<File>;

pub type Header = Box<dyn HeaderTrait>;

pub trait HeaderTrait: std::fmt::Debug {
    fn read(&mut self, fm: &mut FM) -> Result<usize>;
    fn write(&mut self, fm: &mut FM) -> Result<usize>;
}

pub struct FM {
    pub reader: Reader,
    pub writer: Writer,
}
impl FM {
    pub fn new(file: File) -> Result<Self> {
        let reader = file.try_clone()?.into_reader();
        let writer = file.into_writer();
        Ok(Self { reader, writer })
    }
    pub fn is_eof(&mut self) -> Result<bool> {
        if 0 == self.reader.read(&mut [0u8; 1])? {
            Ok(true)
        } else {
            self.set_cursor_relative(-1)?;
            Ok(false)
        }
    }
    pub fn set_cursor_relative(&mut self, pos: i64) -> Result<()> {
        self.reader.seek(SeekFrom::Current(pos))?;
        self.writer.seek(SeekFrom::Current(pos))?;
        Ok(())
    }
    pub fn set_cursor(&mut self, pos: u64) -> Result<()> {
        self.reader.seek(SeekFrom::Start(pos))?;
        self.writer.seek(SeekFrom::Start(pos))?;
        Ok(())
    }
    pub fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        self.reader.read_exact(buf)?;
        Ok(())
    }
    pub fn write(&mut self, buf: &[u8]) -> Result<()> {
        self.writer.write_all(buf)?;
        Ok(())
    }
}

pub trait Convert
where
    Self: Read + Write,
{
    fn into_writer(self) -> Writer;
    fn into_reader(self) -> Reader;
}
impl Convert for File {
    fn into_writer(self) -> Writer {
        // Box::new(BufWriter::with_capacity(capacity, self))
        Box::new(self)
    }
    fn into_reader(self) -> Reader {
        Box::new(self)
        // Box::new(BufReader::with_capacity(capacity, self))
    }
}
