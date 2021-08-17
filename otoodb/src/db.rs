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
        algorithms::{
            bst::{self, BST},
            insert,
        },
        types::*,
        File,
    },
    macros::flags::flags,
    types::{
        hash::{Hex, HexSlice},
        Lim, VLim,
    },
};

flags! {
    pub Flags
    verbose u8 => 1
}

/// default byte set order = `little endian`
#[derive(Debug, Clone)]
pub struct DB {
    file: File<OtooHeader>,
    bst: BST,
    flags: Flags,
    closed: bool,
}

impl DB {
    pub fn open<P: AsRef<Path>>(
        file_path: P,
        a_set_bytes: LS,
        b_set_bytes: LS,
        flags: Option<Flags>,
    ) -> Result<Self> {
        let (mid, end) = (a_set_bytes, b_set_bytes);

        let mut header = OtooHeader::default();
        header.a_set_bytes = mid as HUSize;
        header.b_set_bytes = end as HUSize;

        let file_name = file_path.as_ref().file_name().unwrap().to_owned();

        let file = File::open(file_path, header)?;
        let height = file.fm.header.current_height;

        let elem_lim = VLim::new(0, mid, mid + end);
        let file_lim = Lim::<uPS>::new(0, height * elem_lim.width());
        let bst = BST::new(file_lim, elem_lim)?;

        let flags = if let Some(f) = flags {
            f
        } else {
            Flags::default()
        };

        let verbose = flags.verbose;

        let mut db = Self {
            file,
            bst,
            flags,
            closed: false,
        };

        if verbose > 0 {
            eprintln!("\n{:?} successfully opened.", file_name);
        }
        Self::validate(&mut db, verbose)?;
        Ok(db)
    }
    pub fn debug(&mut self) {
        self.file.fm.debug().unwrap();
    }
    pub fn get(&mut self, bytes_a_or_b: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self.binary_search(bytes_a_or_b)?.0)
    }
    pub fn undefine(&mut self, bytes_a_or_b: &[u8]) -> Result<()> {
        let packet = match self.binary_search(bytes_a_or_b)? {
            (Some(b), pos_b) => match self.binary_search(&b)? {
                (Some(a), pos_a) => {
                    if bytes_a_or_b == a {
                        let empty_sized = b" ".repeat(a.len() + b.len());
                        vec![(empty_sized.clone(), pos_a), (empty_sized, pos_b)]
                    } else {
                        return errbang!(
                            err::ValidationFailed,
                            "not matched pair. {} != {}",
                            HexSlice(bytes_a_or_b),
                            HexSlice(&a)
                        );
                    }
                }
                _ => return errbang!(err::BrokenContent, "undefined item: {}", HexSlice(&b)),
            },
            _ => {
                return errbang!(
                    err::ItemNotFound,
                    "already undefined item: {}",
                    HexSlice(bytes_a_or_b)
                )
            }
        };
        insert::cross_insert::insert(&mut self.file.fm, packet, true)?;

        let fm = self.file_manager();

        fm.header.current_height -= 1;
        fm.header.hash = [0_u8; 32];
        fm.flash_header()?;

        self.update_file_lim()
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

        insert::cross_insert::insert(&mut self.file.fm, packet, false)?;

        let fm = self.file_manager();

        fm.header.current_height += 1;
        fm.header.hash = [0_u8; 32];
        fm.flash_header()?;

        self.update_file_lim()
    }
    pub fn get_file_path(&self) -> &Path {
        &self.file.fm.path
    }
    /// (`hash`, `height`) * Copied
    pub fn version(&self) -> Version {
        let header = self.file.get_header();
        (header.hash, header.current_height)
    }
    /// header's<br>
    /// (`version`, `a_set_bytes`, `b_set_bytes`): (Version, HUSize, HUSize)
    pub fn get_info(&self) -> (Version, HUSize, HUSize) {
        let header = self.file.get_header();
        (
            (header.hash, header.current_height),
            header.a_set_bytes,
            header.b_set_bytes,
        )
    }
    /// header's<br>
    /// (`version`, `a_set_bytes`, `b_set_bytes`): (((u8; 32), usize), usize, usize)
    pub fn get_info_as_usize(&self) -> (([u8; 32], usize), usize, usize) {
        let header = self.file.get_header();
        (
            (header.hash, header.current_height as LS),
            header.a_set_bytes as LS,
            header.b_set_bytes as LS,
        )
    }
    pub fn close(mut self) -> Result<()> {
        self.closed = true;
        if self.flags.verbose > 0 {
            eprintln!("\nclosing..");
        }
        self.calc_hash()
    }
    fn calc_hash(&mut self) -> Result<()> {
        // something changed
        if self.file.get_header().hash.eq(&[0_u8; 32]) {
            if self.flags.verbose > 0 {
                eprintln!("caculating hash..");
            }
            Self::validate(self, 0)?;
        }
        Ok(())
    }

    /// 1. ordering test
    /// 2. pairing test
    /// 3. rewrite hash - by ordering of B set only(chaining hashing)
    pub fn validate(db: &mut DB, verbose: u8) -> Result<()> {
        let ((hash, height), a_bl, b_bl) = db.get_info_as_usize();
        let total_len = a_bl + b_bl;

        eprint!("\x1b[?25l"); // hide cursor

        if verbose > 0 {
            {
                eprintln!(
                    "one to one set database\nhash: {}\nheight: {}\na set bytes: {}\nb set bytes: {}\nvalidating..",
                    if hash.eq(&[0_u8; 32]) {
                        String::from("broken * needed validating")
                    } else {
                        format!("{}", Hex(hash))
                    },
                    height,
                    a_bl,
                    b_bl
                );
            }
        }
        if height < 2 {
            if verbose > 0 {
                eprintln!("complete.");
            }
            return Ok(());
        }

        // closures
        let max_bytes = max_bytes_closure![db.bst.byte_order, a, b];

        db.file.fm.set_cursor(0)?;

        // let mut hashchain = HashChain::new(); // sha3-256
        let mut hashed = blake3::hash(&[0_u8; 32]); // blake3-256

        let mut timer = Timer::new();
        timer.period = Duration::from_millis(60);

        let start = Instant::now();
        for middle in [a_bl, b_bl] {
            let mut prev_buf = vec![0_u8; total_len];
            let mut buf = vec![0_u8; total_len];

            db.file.fm.read(&mut prev_buf)?;
            db.pairing_test(&prev_buf, middle)?;

            if middle == b_bl {
                hashed = blake3::hash(&[hashed.as_bytes(), &prev_buf[middle..]].concat());
                // hashchain.hash_chain(&prev_buf[middle..]);
            }

            for i in 1..height {
                timer.update();

                db.file.fm.read(&mut buf)?;

                if &buf[..middle] != max_bytes(&buf[..middle], &prev_buf[..middle]) {
                    // odering test
                    return errbang!(err::ValidationFailed, "opposite ordering.");
                }

                // pairing test
                db.pairing_test(&buf, middle)?;

                if middle == b_bl {
                    hashed = blake3::hash(&[hashed.as_bytes(), &buf[middle..]].concat());
                    // hashchain.hash_chain(&buf[middle..]);
                }

                std::mem::swap(&mut buf, &mut prev_buf);

                if (verbose > 0) && timer.ready {
                    timer.ready = false;
                    eprint!("\r{} bytes found: {}   ", middle, i);
                }
            }
            if verbose > 0 {
                eprintln!("\r{} bytes found: {}   ", middle, height);
            }
        }

        // db.file.fm.header.hash = hashchain.output();
        db.file.fm.header.hash = hashed.into();
        db.file.fm.flash_header()?;

        if verbose > 0 {
            eprintln!("[{}] complete.", start.elapsed().as_secs().as_time());
        }

        eprint!("\x1b[?25h"); // show cursor

        Ok(())
    }

    /// validation test: input\[`a`\] -> output\[`b`\]<br>
    pub fn pairing_test(&mut self, bytes_line: &[u8], middle: usize) -> Result<()> {
        let a = &bytes_line[..middle];
        let b = &bytes_line[middle..];
        let prev_pos = self.file.fm.stream_position()?;

        let v = self.get(a)?;
        match v {
            Some(v) if v.eq(b) => match self.get(b)? {
                Some(vv) if a.eq(&vv) => {
                    self.file.fm.set_cursor(prev_pos)?;
                    Ok(())
                }
                _ => return errbang!(err::BrokenContent, "Ok: a => b, but Err: b => a"),
            },
            Some(_) => {
                return errbang!(err::ValidationFailed, "not matched pair.");
            }
            None => {
                return errbang!(
                    err::ValidationFailed,
                    "\na. {:?} cannot find -> b. {:?}",
                    &a,
                    &b
                )
            }
        }
    }
    /// ## Return
    /// (Option<ByteVec>, uPS)
    fn binary_search(&mut self, target: &[u8]) -> Result<bst::SearchResult> {
        let (found, res) = self._search(target)?;
        if !found && self.bst_analyse(target)? {
            // re- analyse and search
            let (_, res) = self._search(target)?;
            return Ok(res);
        }
        Ok(res)
    }
    /// ## Return
    /// `has changed` : bool
    fn bst_analyse(&mut self, target: &[u8]) -> Result<bool> {
        let changed = self.bst.fit_the_right(target)?;
        if changed {
            self.update_file_lim()?;
        }
        Ok(changed)
    }
    fn update_file_lim(&mut self) -> Result<()> {
        let elem = self.bst.elem_lim();

        // *** careful mul overflow ***
        let middle_pos = self.file.fm.header.current_height * elem.width();

        let (start_pos, end_pos) = match elem.right {
            false => (0, middle_pos),
            true => (middle_pos, 2 * middle_pos),
        };

        self.bst.change_file_lim(Lim::new(start_pos, end_pos))
    }
    /// ## Return
    /// (`found`: bool, `ResultData`)
    fn _search(&mut self, target: &[u8]) -> Result<(bool, bst::SearchResult)> {
        let fm = &mut self.file.fm;

        let (found, pos) = self.bst.search(fm, target)?;

        if found {
            fm.read_cursoring(self.bst.buf_mut(), pos)?;

            let buf = self.bst.get_buf_by_right();

            Ok((true, (Some(buf.into()), pos)))
        } else {
            Ok((false, (None, pos)))
        }
    }
    fn file_manager(&mut self) -> &mut FM<OtooHeader> {
        self.file.fm.borrow_mut()
    }
}

impl Drop for DB {
    fn drop(&mut self) {
        if !self.closed {
            if self.flags.verbose > 0 {
                eprintln!("\nclosing..")
            }
            Self::calc_hash(self)
                .unwrap_or_else(|e| eprintln!("DB closing error. ValidationFailed: {}", e));
        }
        if self.flags.verbose > 0 {
            eprintln!("complete.")
        }
        eprint!("\x1b[?25h"); // show cursor
    }
}
