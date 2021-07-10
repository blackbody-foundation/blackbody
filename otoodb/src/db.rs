/*
    .. + db.rs + ..

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

//! One to One Set Database.

pub use crate::head::*;
use utils::{fs::File, system::*, types::bytes};

pub struct DB<'a> {
    pub head: Box<OtooHeader>,
    pub file: File<'a>,
}

impl<'a> DB<'a> {
    pub fn open(file_path: &'a str, a_set_bytes: HUSize, b_set_bytes: HUSize) -> Result<Self> {
        let mut header = OtooHeader::from(0, a_set_bytes, b_set_bytes);
        Ok(Self {
            head: header,
            file: File::open(file_path, &mut header)?,
        })
    }
    pub fn a_set_bytes(&self) {}
    pub fn define(&self, bytes_a: &[u8], bytes_b: &[u8]) {}
    pub fn close(self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn start() -> Result<()> {
        let db = DB::open("test", 4, 32)?;
        println!("{:#?}", db.file.header);
        db.close();
        Ok(())
    }
}
