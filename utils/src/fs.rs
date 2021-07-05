/*
    .. + fs.rs + ..

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

pub mod header;
pub mod types;

use super::result::*;
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

        let fm = FM::new(ptr)?;

        let fm = Self::write(fm, &header)?;
        let fm = Self::read(fm, &mut header)?;

        Ok(Self { path, header, fm })
    }
    fn read(fm: FM, header: &mut Header) -> Result<FM> {
        header.read(fm)
    }
    fn write(fm: FM, header: &Header) -> Result<FM> {
        header.write(fm)
    }
    pub fn close(self) {}
}
