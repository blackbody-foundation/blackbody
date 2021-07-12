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
    sets_len: Box<(usize, usize)>, // bytes len for each set
}

impl DB {
    pub fn open(file_path: &'static str, a_set_bytes: usize, b_set_bytes: usize) -> Result<Self> {
        let header = OtooHeader::from(0, a_set_bytes as HUSize, b_set_bytes as HUSize);
        let sets_len = Box::new((a_set_bytes, b_set_bytes));

        let db = Self {
            file: File::open(file_path, header)?,
            sets_len,
        };

        Self::validate(db)
    }
    pub fn validate(mut db: DB) -> Result<DB> {
        db.file.set_cursor(0)?;
        if db.file.is_eof()? {
            return Ok(db);
        }

        let (height, a_bl, b_bl) = db.get_info();

        fn valid_loop(mut db: DB, height: u64, a_bytes: usize, b_bytes: usize) -> Result<DB> {
            let mut buf = vec![0_u8; a_bytes];
            let mut prev_buf = vec![0_u8; a_bytes];
            db.file.read(&mut prev_buf)?;

            for _ in 1..height {
                db.file.read(&mut buf)?;

                if prev_buf != max_bytes![buf.as_slice(), prev_buf.as_slice()]? {
                    return errbang!(err::ValidationFailed);
                }
                db.file.set_cursor_relative(b_bytes as i64)?;
            }

            Ok(db)
        }

        db = valid_loop(db, height, a_bl, b_bl)?;
        db = valid_loop(db, height, b_bl, a_bl)?;

        Ok(db)
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
        let max = max_bytes![bytes_a, bytes_b]?;

        Ok(())
    }
    pub fn close(self) {}
    pub fn get_info(&self) -> (HHSize, usize, usize) {
        (
            self.file.header.current_height,
            self.sets_len.0,
            self.sets_len.1,
        )
    }
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
