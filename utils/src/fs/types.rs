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

use super::header::THeader;
use crate::system::*;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

pub type Ptr = File;
pub type Header = Box<dyn THeader>;
pub type Reader = Box<BufReader<Ptr>>;
pub type Writer = Box<BufWriter<Ptr>>;

pub struct FM {
    pub reader: Reader,
    pub writer: Writer,
}
impl FM {
    pub fn new(ptr: Ptr) -> Result<Self> {
        let reader = ptr.try_clone()?.into_reader();
        let writer = ptr.into_writer();
        Ok(Self { reader, writer })
    }
}

pub trait Convert {
    fn into_writer(self) -> Box<BufWriter<Ptr>>;
    fn into_reader(self) -> Box<BufReader<Ptr>>;
}
impl Convert for File {
    fn into_reader(self) -> Box<BufReader<Ptr>> {
        Box::new(BufReader::new(self))
    }
    fn into_writer(self) -> Box<BufWriter<Ptr>> {
        Box::new(BufWriter::new(self))
    }
}
