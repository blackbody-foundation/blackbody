/*
    .. + lib.rs + ..

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

mod cmn;
use cmn::*;

mod process;
mod read;
mod write;

pub mod cccs;
pub mod target;

/// this has only Path and Sizes(two of usize) so it's cheap to clone or new
pub struct Wormhole {
    pub db_path: PathBuf,
    pub src_bytes_size: LS,
    pub dst_bytes_size: LS,
}

impl Wormhole {
    pub fn new(db_path: &str, src_bytes_size: LS, dst_bytes_size: LS) -> Self {
        let db_path = pathy!(db_path);
        Self {
            db_path,
            src_bytes_size,
            dst_bytes_size,
        }
    }
    pub fn transform<'a>(&self, file_path: &'a str) -> Result<&'a str> {
        let otoodb = target::OtooDB(self.load_otoodb()?);

        let (read_tx, read_rx) = channel::bounded(BOUNDED_CAP);
        let (write_tx, write_rx) = channel::bounded(BOUNDED_CAP);

        let read_handle = thread::spawn(move || read::read_loop(read_tx));

        let process_handle =
            thread::spawn(move || process::process_loop(read_rx, otoodb, write_tx));

        let write_handle = thread::spawn(move || write::write_loop(write_rx));

        read_handle.join().unwrap()?;
        process_handle.join().unwrap()?;
        write_handle.join().unwrap()?;

        Ok(file_path)
    }
    fn load_otoodb(&self) -> Result<DB> {
        DB::open(
            valid_path!(self.db_path)?,
            self.src_bytes_size,
            self.dst_bytes_size,
        )
    }
}
