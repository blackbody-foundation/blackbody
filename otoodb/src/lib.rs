/*
    .. + lib.rs + ..

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

// One to One Set Database.
use utils::{fs::File, system::*};
mod head;
use head::*;

pub struct DB {
    pub file: File,
}

impl DB {
    pub fn open(file_path: &'static str, a_set_bytes: HUSize, b_set_bytes: HUSize) -> Result<Self> {
        let mut h = Head::new();

        h.a_set_bytes = a_set_bytes;
        h.b_set_bytes = b_set_bytes;

        Ok(Self {
            file: File::open(file_path, h)?,
        })
    }
    pub fn close(self) {}
}
