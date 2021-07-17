/*
    .. + ptr.rs + ..

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

use std::ops::{Deref, DerefMut};

use super::*;

#[derive(Debug)]
pub struct Ptr {
    file: Box<File>,
}

impl Ptr {
    pub fn new(file: File) -> Self {
        Self {
            file: Box::new(file),
        }
    }
    /// copying
    pub fn to_reader(&self) -> Result<Box<BufReader<File>>> {
        Ok(Box::new(BufReader::new(self.try_clone()?)))
    }
    /// copying
    pub fn to_writer(&self) -> Result<Box<BufWriter<File>>> {
        Ok(Box::new(BufWriter::new(self.try_clone()?)))
    }
}

impl Deref for Ptr {
    type Target = Box<File>;
    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

impl DerefMut for Ptr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.file
    }
}