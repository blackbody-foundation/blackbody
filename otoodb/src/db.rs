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

use crate::{head::*, item::*, std::*};

use utils::{
    fs::{types::FM, File},
    macros::bytes,
};

#[derive(Debug)]
pub struct DB {
    file: File<OtooHeader>,
    console: Option<Plugin<Console>>,
}

impl DB {
    pub fn open(file_path: &'static str, a_set_bytes: usize, b_set_bytes: usize) -> Result<Self> {
        let db = Self {
            file: File::open(
                file_path,
                OtooHeader::new(0, a_set_bytes as HUSize, b_set_bytes as HUSize),
            )?,
            console: None,
        };
        eprintln!("file successfully opened.");
        // Self::validate(db)
        Ok(db)
    }
    pub fn debug(&mut self) -> Result<()> {
        let mut buf = [0u8; 200];
        let mut num_read;
        loop {
            num_read = self.file.fm.read_general(&mut buf[..])?;
            if num_read < 1 {
                break;
            }
            eprintln!("{:?}", &buf);
        }
        Ok(())
    }
    pub fn validate(mut db: DB) -> Result<DB> {
        let (height, a_bytes, b_bytes) = db.info();
        eprintln!(
            "validating..\nheight: {}\na set bytes: {}\nb set bytes: {}",
            height, a_bytes, b_bytes
        );
        if height <= 1 {
            eprintln!("complete.");
            return Ok(db);
        }

        let fm = db.file_manager();
        fm.set_cursor(0)?;

        for (a_bl, b_bl) in [(a_bytes, b_bytes), (b_bytes, a_bytes)] {
            let mut buf = vec![0_u8; a_bl];
            let mut prev_buf = vec![0_u8; a_bl];

            fm.read(&mut prev_buf)?;
            fm.set_cursor_relative(b_bl as i64)?;

            for _ in 1..height {
                fm.read(&mut buf)?;

                if buf != max_bytes![buf.as_slice(), prev_buf.as_slice()]? {
                    return errbang!(err::ValidationFailed);
                }
                fm.set_cursor_relative(b_bl as i64)?;

                std::mem::swap(&mut buf, &mut prev_buf);
            }
        }

        eprintln!("complete.");
        Ok(db)
    }

    pub fn binary_search(&mut self, target: &[u8]) -> Result<(bool, ItemPointer)> {
        let (height, a_bl, b_bl) = self.info();

        let a_len = is_bytes_len![target, a_bl, b_bl]?;

        let (b_len, start) = if a_len == a_bl {
            (b_bl, 0)
        } else {
            (a_bl, height)
        };

        let total_len = a_len + b_len;

        if height == 0 {
            return Ok((
                false,
                ItemPointer::new(
                    0,
                    true,
                    if a_len == a_bl { 0 } else { total_len as u64 },
                    a_len,
                    b_len,
                ),
            ));
        }

        let fm = self.file_manager();

        let mut distance = height;
        let mut mid = 0;

        let mut mid_buf = vec![0u8; a_len];
        let mut upwards = false;

        let mut pos;

        loop {
            distance /= 2;
            if upwards {
                // if distance > mid {
                //     mid = 0;
                // } else {
                //     mid -= distance;
                // }
                mid -= distance;
            } else {
                mid += distance;
            }

            pos = ((start + mid) * total_len) as u64;

            fm.read_cursoring(mid_buf.as_mut_slice(), pos)?;

            if mid_buf == target {
                // found
                return Ok((true, ItemPointer::new(mid, upwards, pos, a_len, b_len)));
            }

            upwards = target != max_bytes![target, mid_buf.as_slice()]?;

            if distance == 0 {
                // couldn't find
                pos = if upwards { pos } else { pos + total_len as u64 };
                return Ok((false, ItemPointer::new(mid, upwards, pos, a_len, b_len)));
            }
        }
    }
    pub fn get(&mut self, bytes_a_or_b: &[u8]) -> Result<Option<Vec<u8>>> {
        let (found, item) = self.binary_search(bytes_a_or_b)?;
        if !found {
            Ok(None)
        } else {
            let mut buf = vec![0_u8; item.b_len];
            self.file
                .fm
                .read_cursoring(&mut buf, item.pos + item.a_len as u64)?;
            Ok(Some(buf))
        }
    }
    pub fn define(&mut self, bytes_a: &[u8], bytes_b: &[u8]) -> Result<()> {
        let mut item_bag = Vec::new();

        for bytes in [(bytes_a, bytes_b), (bytes_b, bytes_a)] {
            let (found, ptr) = self.binary_search(bytes.0)?;
            if found {
                return errbang!(err::Interrupted, "item already exists");
            }

            item_bag.push((ptr, [bytes.0, bytes.1].concat()));
        }

        item_bag.sort_by_key(|k| k.0.pos); // sort by position in the file

        dbg!(&item_bag);

        let fm = self.file_manager();

        let (ptr0, buf0) = &item_bag[0];
        let (ptr1, buf1) = &item_bag[1];

        let total_len = ptr0.a_len + ptr0.b_len;

        let reading_start = ptr0.pos; // + (!ptr0.upwards as usize * total_len) as u64;
        let reading_end = ptr1.pos; // + (!ptr1.upwards as usize * total_len) as u64;

        dbg!(reading_start, reading_end);

        fm.insert_special(&buf0, reading_start, reading_end)?;
        fm.insert_special(&buf1, reading_end, 0)?;

        dbg!(fm.header.current_height += 1);

        fm.flush_header()
    }
    pub fn close(self) {}
    pub fn info(&self) -> (usize, usize, usize) {
        (
            self.file.fm.header.current_height as usize,
            self.file.fm.header.a_set_bytes as usize,
            self.file.fm.header.b_set_bytes as usize,
        )
    }
    fn file_manager(&mut self) -> &mut FM<OtooHeader> {
        self.file.fm.borrow_mut()
    }
    fn cli(&mut self, context: String) -> Result<()> {
        employ!(self.console)?.cli(context);
        Ok(())
    }
}

impl Concentric<Console> for DB {
    fn concentric(&mut self, _some_plugin: Option<Plugin<Console>>) -> &mut Self {
        self.console = _some_plugin;
        self
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
