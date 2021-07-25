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

use utils::{fs::types::LS, system::*, types::tgroup::TGroup};

use otoodb::DB;
use std::path::PathBuf;

pub mod cccs;

mod tgroup; // thread group
use tgroup::TG;

/// this has only Path and Sizes(two of usize) so it's cheap to clone or new
pub struct Wormhole {
    pub db_path: PathBuf,
    pub src_bytes_size: LS,
    pub dst_bytes_size: LS,
}

impl Wormhole {
    pub fn new(db_path: &str, src_bytes_size: LS, dst_bytes_size: LS) -> Self {
        Self {
            db_path: pathy!(db_path),
            src_bytes_size,
            dst_bytes_size,
        }
    }
    pub fn transform<'a>(&self, file_path: &'a str) -> Result<&'a str> {
        let tg = TG::new(tgroup::Requirement::new(
            file_path.to_owned(),
            self.load_otoodb()?,
        ));
        tg.join();
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
