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
    pub fm: FM<T>,
}

impl<T: HeaderTrait> File<T> {
    pub fn open(path: &'static str, header: Box<T>) -> Result<Self> {
        let fm = FM::new(path, header)?;

        Ok(Self { path, fm })
    }
    pub fn close(self) {}
}
