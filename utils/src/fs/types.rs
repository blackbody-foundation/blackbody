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

use super::header;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

pub type IOFile = File;
pub type Header = Box<dyn header::Header>;
pub type Reader = Box<BufReader<File>>;
pub type Writer = Box<BufWriter<File>>;

pub trait Convert {
    fn into_writer(self) -> Box<BufWriter<File>>;
    fn into_reader(self) -> Box<BufReader<File>>;
}
impl Convert for File {
    fn into_reader(self) -> Box<BufReader<File>> {
        Box::new(BufReader::new(self))
    }
    fn into_writer(self) -> Box<BufWriter<File>> {
        Box::new(BufWriter::new(self))
    }
}
