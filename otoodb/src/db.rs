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
        let (height, a_bytes, b_bytes) = db.get_info();
        let fm = db.file_manager();

        fm.set_cursor(0)?;
        if fm.is_eof()? {
            return Ok(db);
        }

        for i in 0..1 {
            let (a_bl, b_bl) = if i == 0 {
                (a_bytes, b_bytes)
            } else {
                (b_bytes, a_bytes)
            };

            let mut buf = vec![0_u8; a_bl];
            let mut prev_buf = vec![0_u8; a_bl];

            fm.read(&mut prev_buf)?;

            for _ in 1..height {
                fm.read(&mut buf)?;

                if buf != max_bytes![buf.as_slice(), prev_buf.as_slice()]? {
                    return errbang!(err::ValidationFailed);
                }
                fm.set_cursor_relative(b_bl as i64)?;

                std::mem::swap(&mut buf, &mut prev_buf);
            }
        }

        Ok(db)
    }

    /// target bytes -> Result<(found, index, pos, len)>
    pub fn binary_search(&mut self, target: &[u8]) -> Result<(bool, usize, u64, usize)> {
        let (height, a_bl, b_bl) = self.get_info();
        let fm = self.file_manager();

        let bytes_len = is_bytes_len![target, a_bl, b_bl]?;
        let total_len = a_bl + b_bl;

        if height == 0 {
            return Ok((false, 0, 0, bytes_len));
        }

        let mut start = if bytes_len == a_bl { 0 } else { height };

        let mut distance = height;
        let mut mid;

        let mut mid_buf = vec![0u8; bytes_len];
        let mut upwards = false;

        loop {
            distance /= 2;
            if upwards {
                mid = start - distance;
            } else {
                mid = start + distance;
            }

            fm.set_cursor((mid * total_len) as u64)?;
            fm.read(mid_buf.as_mut_slice())?;

            if mid_buf == target {
                let pos = (mid * total_len) as u64;
                return Ok((true, mid, pos, bytes_len)); // found
            }

            start = mid;

            upwards = target != max_bytes![target, mid_buf.as_slice()]?;

            if distance == 0 {
                // couldn't find
                let index = if upwards { mid - 1 } else { mid + 1 };
                let pos = (index * total_len) as u64;
                return Ok((false, index, pos, bytes_len));
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
    pub fn get_info(&self) -> (usize, usize, usize) {
        (
            self.file.fm.header.current_height as usize,
            self.file.fm.header.a_set_bytes as usize,
            self.file.fm.header.b_set_bytes as usize,
        )
    }
    fn file_manager(&mut self) -> &mut FM<OtooHeader> {
        self.file.fm.borrow_mut()
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
