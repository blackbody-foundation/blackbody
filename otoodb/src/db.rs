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

use crate::std::*;

use crate::head::*;

use utils::fs::{types::FM, File};

pub struct DB {
    pub file: File<OtooHeader>,
}

impl DB {
    pub fn open(file_path: &'static str, a_set_bytes: usize, b_set_bytes: usize) -> Result<Self> {
        let db = Self {
            file: File::open(
                file_path,
                OtooHeader::from(0, a_set_bytes as HUSize, b_set_bytes as HUSize),
            )?,
        };
        Self::validate(db)
    }
    pub fn validate(mut db: DB) -> Result<DB> {
        let (fm, height, a_bl, b_bl) = db.get_info_and_fm();

        fm.set_cursor(0)?;
        if fm.is_eof()? {
            return Ok(db);
        }

        fn valid_loop(
            fm: &mut FM<OtooHeader>,
            height: usize,
            a_bytes: usize,
            b_bytes: usize,
        ) -> Result<()> {
            let mut buf = vec![0_u8; a_bytes];
            let mut prev_buf = vec![0_u8; a_bytes];
            fm.read(&mut prev_buf)?;

            for _ in 1..height {
                fm.read(&mut buf)?;

                if buf != max_bytes![buf.as_slice(), prev_buf.as_slice()]? {
                    return errbang!(err::ValidationFailed);
                }
                fm.set_cursor_relative(b_bytes as i64)?;

                prev_buf = buf.clone();
            }
            Ok(())
        }

        valid_loop(fm, height, a_bl, b_bl)?;
        valid_loop(fm, height, b_bl, a_bl)?;

        Ok(db)
    }
    pub fn binary_search(&mut self, bytes: &[u8]) -> Result<usize> {
        let (fm, height, a_bl, b_bl) = self.get_info_and_fm();
        if height == 0 {
            return Ok(0);
        }

        let len = is_bytes_len![bytes, a_bl, b_bl]?; // + check valid length
        let start = if len == a_bl { 0 } else { height };
        let mut distance = height;
        let (mut left, mut mid, mut right): (usize, usize, usize);

        let mut buf_mid = vec![0u8; a_bl];

        loop {
            distance /= 2;
            mid = start + distance;

            fm.read_cursoring(buf_mid.as_mut_slice(), mid as u64)?;

            todo!("binary search")
        }
    }
    pub fn get(&self, bytes_a_or_b: &[u8]) -> Result<&[u8]> {
        Ok(&[1, 2, 3])
    }
    pub fn define(&self, bytes_a: &[u8], bytes_b: &[u8]) -> Result<()> {
        let max = max_bytes![bytes_a, bytes_b]?;

        Ok(())
    }
    pub fn close(self) {}
    pub fn get_info(&self) -> Box<(usize, usize, usize)> {
        Box::new((
            self.file.fm.header.current_height as usize,
            self.file.fm.header.a_set_bytes as usize,
            self.file.fm.header.b_set_bytes as usize,
        ))
    }
    fn get_info_and_fm(&mut self) -> (&mut FM<OtooHeader>, usize, usize, usize) {
        let (h, a, b) = *self.get_info();
        (self.file.fm.borrow_mut(), h, a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn start() -> Result<()> {
        Ok(())
    }
}
