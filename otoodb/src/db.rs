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
use utils::{fs::File, system::*, types::bytes::*};

pub struct DB {
    pub file: File<OtooHeader>,
    pub sets_len: Box<(usize, usize)>, // bytes len for each set
}

impl DB {
    pub fn open(file_path: &'static str, a_set_bytes: usize, b_set_bytes: usize) -> Result<Self> {
        let sets_len = Box::new((a_set_bytes, b_set_bytes));
        Ok(Self {
            file: File::open(
                file_path,
                OtooHeader::from(0, a_set_bytes as HUSize, b_set_bytes as HUSize),
                sets_len.0.max(sets_len.1),
            )?,
            sets_len,
        })
    }
    fn binary_search(&self, bytes: &[u8]) -> Result<()> {
        let len = is_bytes_len![bytes, self.sets_len.0, self.sets_len.1]?; // + check valid length
        let height = self.file.header.current_height;
        // self.file.set_cursor()
        Ok(())
    }
    pub fn get(&self, bytes_a_or_b: &[u8]) -> Result<&[u8]> {
        Ok(&[1, 2, 3])
    }
    pub fn define(&self, bytes_a: &[u8], bytes_b: &[u8]) -> Result<()> {
        let a = max_bytes![bytes_a, bytes_b]?;
        Ok(())
    }
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
