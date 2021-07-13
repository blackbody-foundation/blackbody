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
    pub fn binary_search(&mut self, bytes: &[u8]) -> Result<(bool, usize, u64, usize)> {
        // found, index, pos, len
        let (fm, height, a_l, b_l) = self.get_info_and_fm();

        let (main_len, total_len, mut start) = if a_l == is_bytes_len![bytes, a_l, b_l]? {
            (a_l, a_l + b_l, 0)
        } else {
            (b_l, b_l + a_l, height)
        };

        if height == 0 {
            return Ok((false, 0, 0, main_len));
        }

        let mut distance = height;
        let mut mid;

        let mut buf_mid = vec![0u8; main_len];
        let mut top_gear = false;

        loop {
            distance /= 2;
            if top_gear {
                mid = start - distance;
            } else {
                mid = start + distance;
            }

            fm.read_cursoring(buf_mid.as_mut_slice(), (total_len * mid) as u64)?;
            if buf_mid == bytes {
                return Ok((true, mid, (mid * total_len) as u64, main_len)); // found
            }

            start = mid;

            top_gear = max_bytes![bytes, buf_mid.as_slice()]? != bytes;

            if distance == 0 {
                let index = if top_gear { mid - 1 } else { mid + 1 };
                return Ok((false, index, (index * total_len) as u64, main_len));
                // couldn't find
            }
        }
    }
    pub fn get(&mut self, bytes_a_or_b: &[u8]) -> Result<Option<Vec<u8>>> {
        let (found, _, pos, len) = self.binary_search(bytes_a_or_b)?;
        if !found {
            Ok(None)
        } else {
            let mut buf = vec![0_u8; len];
            self.file.fm.read_cursoring(&mut buf, pos)?;
            Ok(Some(buf))
        }
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
