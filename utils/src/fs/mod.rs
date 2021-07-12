/*
    .. + fs + ..

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

pub mod types;

use crate::system::*;
use types::*;

pub struct File<T> {
    pub path: &'static str,
    pub header: T,
    fm: FM,
    header_size: u64,
}

impl<T: HeaderTrait> File<T> {
    pub fn open(path: &'static str, mut header: T) -> Result<Self> {
        let ptr = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(path)?;

        let mut fm = FM::new(ptr)?;

        let header_size = header.read(&mut fm)? as u64;

        Ok(Self {
            path,
            header,
            fm,
            header_size,
        })
    }
    pub fn is_eof(&mut self) -> Result<bool> {
        self.fm.is_eof()
    }
    pub fn set_cursor(&mut self, pos: u64) -> Result<()> {
        self.fm.set_cursor(pos + self.header_size)
    }
    pub fn set_cursor_relative(&mut self, pos: i64) -> Result<()> {
        self.fm.set_cursor_relative(pos)
    }
    pub fn set_cursor_general(&mut self, pos: u64) -> Result<()> {
        self.fm.set_cursor(pos)
    }
    pub fn read(&mut self, buf: &mut [u8]) -> Result<()> {
        self.fm.read(buf)
    }
    pub fn write(&mut self, buf: &mut [u8]) -> Result<()> {
        self.fm.write(buf)
    }
    pub fn close(self) {}
}
