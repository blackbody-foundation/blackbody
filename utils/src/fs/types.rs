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
use std::io::{BufReader, BufWriter, Read, Write};

pub type Reader = Box<BufReader<File>>;
pub type Writer = Box<BufWriter<File>>;

pub type Header = Box<dyn HeaderTrait>;

pub trait HeaderTrait: std::fmt::Debug {
    fn read(&mut self, fm: &mut FM) -> Result<()>;
    fn write(&mut self, fm: &mut FM) -> Result<()>;
}

pub struct FM {
    pub reader: Reader,
    pub writer: Writer,
}
impl FM {
    pub fn new(file: File, buffer_size: usize) -> Result<Self> {
        let reader = file.try_clone()?.into_reader(buffer_size);
        let writer = file.into_writer(buffer_size);
        Ok(Self { reader, writer })
    }
}

pub trait Convert
where
    Self: Read + Write,
{
    fn into_writer(self, capacity: usize) -> Writer;
    fn into_reader(self, capacity: usize) -> Reader;
}
impl Convert for File {
    fn into_writer(self, capacity: usize) -> Writer {
        Box::new(BufWriter::with_capacity(capacity, self))
    }
    fn into_reader(self, capacity: usize) -> Reader {
        Box::new(BufReader::with_capacity(capacity, self))
    }
}
