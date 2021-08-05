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

/// default byte set order = `little endian`
#[derive(Debug, Clone)]
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
        Ok(self.binary_search(bytes_a_or_b)?.0)
    }
    pub fn define(&mut self, bytes_a: &[u8], bytes_b: &[u8]) -> Result<()> {
        let mut packet = Packet::new();

        for bytes in [[bytes_a, bytes_b], [bytes_b, bytes_a]].iter() {
            match self.binary_search(bytes[0])? {
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
    pub fn version(&self) -> HHSize {
        self.file.fm.header.current_height
    }
    /// header's
    /// (height, a_set_bytes, b_set_bytes): (HHSize, HUSize, HUSize)
    pub fn get_info(&self) -> (HHSize, HUSize, HUSize) {
        let header = self.file.fm.header.as_ref();
        (
            header.current_height,
            header.a_set_bytes,
            header.b_set_bytes,
        )
    }
    /// header's
    /// (height, a_set_bytes, b_set_bytes): (LS, LS, LS)
    pub fn get_info_as_usize(&self) -> (LS, LS, LS) {
        let header = self.file.fm.header.as_ref();
        (
            header.current_height as LS,
            header.a_set_bytes as LS,
            header.b_set_bytes as LS,
        )
    }
    pub fn close(self) {}
    pub fn validate(mut db: DB) -> Result<DB> {
        let (height, a_bl, b_bl) = db.get_info_as_usize();
        let total_len = a_bl + b_bl;
        eprintln!(
            "validating..\nheight: {}\na set bytes: {}\nb set bytes: {}",
            height, a_bl, b_bl
        );
        if height < 2 {
            eprintln!("complete.");
            return Ok(db);
        }

        // closures
        let max_bytes = max_bytes_closure![db.bst.byte_order, a, b];

        db.file.fm.set_cursor(0)?;

        for middle in [a_bl, b_bl] {
            let mut prev_buf = vec![0_u8; total_len];
            let mut buf = vec![0_u8; total_len];

            db.file.fm.read(&mut prev_buf)?;
            db.pairing_test(&prev_buf, middle)?;

            for _ in 1..height {
                db.file.fm.read(&mut buf)?;

                if &buf[..middle] != max_bytes(&buf[..middle], &prev_buf[..middle]) {
                    // odering test
                    return errbang!(err::ValidationFailed, "opposite ordering.");
                }

                // pairing test
                db.pairing_test(&buf, middle)?;

                std::mem::swap(&mut buf, &mut prev_buf);
            }
        }

        eprintln!("complete.");
        Ok(db)
    }

    /// validation test: input\[`a`\] -> output\[`b`\]
    pub fn pairing_test(&mut self, bytes_line: &[u8], middle: usize) -> Result<()> {
        let a = &bytes_line[..middle];
        let b = &bytes_line[middle..];
        let v = self.get(a)?;
        match v {
            Some(v) if v.eq(b) => Ok(()),
            Some(_) => {
                return errbang!(err::ValidationFailed, "not matched pair.");
            }
            None => {
                return errbang!(
                    err::ValidationFailed,
                    "a. {:?} cannot find -> b. {:?}",
                    &a,
                    &b
                )
            }
        }
    }
    /// if limit_height == 0 then limit_height = fm.header.current_height
    fn binary_search(&mut self, target: &[u8]) -> Result<(Option<Vec<u8>>, uPS)> {
        let fm = &mut self.file.fm;
        let height = fm.header.current_height;

        let elem = self.bst.elem_lim();

        let right = elem.is_right_side(target)?;

        let (start_pos, end_pos) = match right {
            // careful mul overflow
            false => (0, height * elem.width() as uPS),
            true => (
                height * elem.width() as uPS,
                (2 * height) * elem.width() as uPS,
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
