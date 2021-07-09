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
use std::io::{BufReader, BufWriter};

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
    pub fn new(ptr: File) -> Result<Self> {
        let reader = ptr.try_clone()?.into_reader();
        let writer = ptr.into_writer();
        Ok(Self { reader, writer })
    }
}

pub trait Convert {
    fn into_writer(self) -> Writer;
    fn into_reader(self) -> Reader;
}
impl Convert for File {
    fn into_reader(self) -> Reader {
        Box::new(BufReader::new(self))
    }
    fn into_writer(self) -> Writer {
        Box::new(BufWriter::new(self))
    }
}
