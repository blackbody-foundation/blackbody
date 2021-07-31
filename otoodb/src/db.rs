/*
    .. + db.rs + ..

    Copyright 2021 Hwakyeom Kim(=just-do-halee)

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

use crate::{cmn::*, head::*};

use utils::{
    fs::{
        algorithms::{bst::BST, insert},
        types::*,
        File,
    },
    types::{Lim, VLim},
};

#[derive(Debug)]
pub struct DB {
    file: File<OtooHeader>,
    bst: BST,
}

impl DB {
    pub fn open(file_path: &str, a_set_bytes: LS, b_set_bytes: LS) -> Result<Self> {
        let (mid, end) = (a_set_bytes, b_set_bytes);

        let header = OtooHeader::new(0, mid as HUSize, end as HUSize);
        let file = File::open(file_path, header)?;

        let file_lim = Lim::<uPS>::new(0, (mid + end) as uPS); // initial
        let elem_lim = VLim::new(0, mid, mid + end);
        let bst = BST::new(file_lim, elem_lim)?;

        let db = Self { file, bst };
        eprintln!("file successfully opened.");
        Self::validate(db)
    }
    pub fn debug(&mut self) {
        self.file.fm.debug().unwrap();
    }
    pub fn get(&mut self, bytes_a_or_b: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self.binary_search(bytes_a_or_b, 0)?.0)
    }
    /// version == specific height limited
    pub fn get_by_version(
        &mut self,
        bytes_a_or_b: &[u8],
        limited_height: HHSize,
    ) -> Result<Option<Vec<u8>>> {
        Ok(self.binary_search(bytes_a_or_b, limited_height)?.0)
    }
    pub fn define(&mut self, bytes_a: &[u8], bytes_b: &[u8]) -> Result<()> {
        let mut packet = Packet::new();

        for bytes in [[bytes_a, bytes_b], [bytes_b, bytes_a]].iter() {
            match self.binary_search(bytes[0], 0)? {
                (None, pos) => {
                    packet.push((bytes.concat(), pos));
                }
                (Some(_), _) => {
                    return errbang!(err::Interrupted, "item already exists");
                }
            }
        }

        let fm = self.file_manager();

        insert::cross_insert::insert(fm, packet)?;

        self.file.fm.header.current_height += 1;
        self.file.fm.flush_header()
    }
    pub fn get_file_path(&self) -> &Path {
        &self.file.fm.path
    }
    /// header's
    /// (height, a_set_bytes, b_set_bytes): (LS, LS, LS)
    pub fn get_info(&self) -> (LS, LS, LS) {
        let header = self.file.fm.header.as_ref();
        (
            header.current_height as LS,
            header.a_set_bytes as LS,
            header.b_set_bytes as LS,
        )
    }
    pub fn close(self) {}
    pub fn validate(mut db: DB) -> Result<DB> {
        let (height, a_bytes, b_bytes) = db.get_info();
        eprintln!(
            "validating..\nheight: {}\na set bytes: {}\nb set bytes: {}",
            height, a_bytes, b_bytes
        );
        if height < 2 {
            eprintln!("complete.");
            return Ok(db);
        }

        let fm = db.file_manager();

        fm.set_cursor(0)?;

        for (a_bl, b_bl) in [(a_bytes, b_bytes), (b_bytes, a_bytes)] {
            let mut buf = vec![0_u8; a_bl];
            let mut prev_buf = vec![0_u8; a_bl];

            fm.read(&mut prev_buf)?;
            fm.set_cursor_relative(b_bl as iPS)?; // *** warning b_bl is (usize)LS ***

            for _ in 1..height {
                fm.read(&mut buf)?;

                if buf != max_bytes![buf.as_slice(), prev_buf.as_slice()]? {
                    return errbang!(err::ValidationFailed);
                }
                fm.set_cursor_relative(b_bl as iPS)?;

                std::mem::swap(&mut buf, &mut prev_buf);
            }
        }

        eprintln!("complete.");
        Ok(db)
    }

    /// if limit_height == 0 then limit_height = fm.header.current_height
    fn binary_search(
        &mut self,
        target: &[u8],
        limited_height: HHSize,
    ) -> Result<(Option<Vec<u8>>, uPS)> {
        let fm = &mut self.file.fm;
        let height = fm.header.current_height;
        let limited_height = if limited_height > 0 {
            limited_height
        } else {
            height
        };
        if height < limited_height {
            return errbang!(
                err::HigherVersion,
                "limit height({}) must be less than original height({}).",
                limited_height,
                height
            );
        }
        let elem = self.bst.elem_lim();

        let right = elem.is_right_side(target)?;

        let (start_pos, end_pos) = match right {
            // careful mul overflow
            false => (0, limited_height * elem.width() as uPS),
            true => (
                height * elem.width() as uPS,
                (height + limited_height) * elem.width() as uPS,
            ),
        };

        self.bst.change_file_lim(Lim::new(start_pos, end_pos))?;

        let (found, pos) = self.bst.search(fm, target)?;

        if found {
            fm.read_cursoring(self.bst.buf_mut(), pos)?;
            let buf = if right {
                self.bst.buf_reversed_right_limed()
            } else {
                self.bst.buf_right_limed()
            };
            Ok((Some(buf.to_vec()), pos))
        } else {
            Ok((None, pos))
        }
    }
    fn file_manager(&mut self) -> &mut FM<OtooHeader> {
        self.file.fm.borrow_mut()
    }
}
