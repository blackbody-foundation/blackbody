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
    console: Option<Plugin<Console>>,
}

impl DB {
    pub fn open(file_path: &'static str, a_set_bytes: LS, b_set_bytes: LS) -> Result<Self> {
        let (mid, end) = (a_set_bytes, b_set_bytes);
        let header = OtooHeader::new(0, mid as HUSize, end as HUSize);
        let file = File::open(file_path, header)?;

        let file_lim = Lim::<uPS>::new(0, (mid + end) as uPS); // initial
        let elem_lim = VLim::new(0, mid, mid + end);
        let bst = BST::new(file_lim, elem_lim)?;

        let db = Self {
            file,
            bst,
            console: None,
        };
        eprintln!("file successfully opened.");
        // Self::validate(db)
        Ok(db)
    }
    pub fn debug(&mut self) {
        self.file.fm.debug().unwrap();
    }
    fn init(fm: &mut FM<OtooHeader>, bytes_a: &[u8], bytes_b: &[u8]) -> Result<()> {
        let a_set = [bytes_a, bytes_b].concat();
        let b_set = [bytes_b, bytes_a].concat();
        let total_len = (bytes_a.len() + bytes_b.len()) as uPS;
        dbg!(&a_set, &b_set, &total_len);
        dbg!(fm.write_cursoring(&a_set[..], 0)?);
        dbg!(fm.write_cursoring(&b_set[..], total_len)?);
        Ok(())
    }
    pub fn get(&mut self, bytes_a_or_b: &[u8]) -> Result<Option<Vec<u8>>> {
        match self.binary_search(bytes_a_or_b)?.0 {
            Some(value) => Ok(Some(value)),
            None => Ok(None),
        }
    }
    pub fn define(&mut self, bytes_a: &[u8], bytes_b: &[u8]) -> Result<()> {
        let db_height = self.file.fm.header.current_height;
        if db_height == 0 {
            Self::init(&mut self.file.fm, bytes_a, bytes_b)?;
        } else {
            let mut packet = Packet::new();

            for bytes in [[bytes_a, bytes_b], [bytes_b, bytes_a]] {
                match self.binary_search(bytes[0])? {
                    (None, pos) => packet.push((bytes.concat(), pos)),
                    (Some(_), _) => {
                        return errbang!(err::Interrupted, "item already exists");
                    }
                }
            }

            dbg!(&packet);
            let fm = self.file_manager();

            insert::cross_insert::insert(fm, packet)?;
        }
        self.file.fm.header.current_height += 1;
        self.file.fm.flush_header()
    }
    pub fn close(self) {}
    pub fn info(&self) -> (LS, LS, LS) {
        (
            self.file.fm.header.current_height as LS,
            self.file.fm.header.a_set_bytes as LS,
            self.file.fm.header.b_set_bytes as LS,
        )
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
            fm.set_cursor_relative(b_bl as iPS)?;

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
    // ------------------
    fn binary_search(&mut self, target: &[u8]) -> Result<(Option<Vec<u8>>, uPS)> {
        let fm = &mut self.file.fm;

        let elem = self.bst.elem_lim();

        let right = elem.is_right_side(target)?;
        let (start_pos, end_pos) = match right {
            false => (0, fm.header.current_height * elem.end as uPS),
            true => (
                fm.header.current_height * elem.end as uPS,
                fm.content_end_pos(false)?,
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
