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

pub struct File {
    pub path: &'static str,
    pub header: Header,
    fm: FM,
}

impl File {
    pub fn open(path: &'static str, mut header: Header) -> Result<Self> {
        let ptr = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(path)?;

        let mut fm = FM::new(ptr)?;

        header.read(&mut fm)?;

        Ok(Self { path, header, fm })
    }
    fn read(&self) -> Result<()> {
        Ok(())
    }
    fn write(&self) -> Result<()> {
        Ok(())
    }
    pub fn close(self) {}
}
