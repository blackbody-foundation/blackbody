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

// One to One Set Database.

use std::io;
use utils::fs::File;

mod head;
use head::Header;

pub struct ABSetBytes(u64, u64);

pub struct DB {
    pub a_b_bytes: ABSetBytes,
    pub file: File,
}
impl DB {
    pub fn open(file_path: &'static str, a_set_bytes: u64, b_set_bytes: u64) -> io::Result<Self> {
        let header = Box::new(Header::new(a_set_bytes, b_set_bytes));
        Ok(Self {
            a_b_bytes: ABSetBytes(a_set_bytes, b_set_bytes),
            file: File::open(file_path, header)?,
        })
    }
    pub fn close(self) {}
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let db = DB::open("/Volumes/programs/code/blackchain/test", 4, 32).unwrap();
        println!("{:#?}", db.file.header);
        db.close();
    }
}
