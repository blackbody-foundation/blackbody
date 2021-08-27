/*
    .. + lib.rs + ..

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

mod cmn;
mod db;
mod head;

pub use db::{Flags, DB};
pub use head::*;

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::cmn::Result;

    const FILE_PATH: &str = "test";
    #[test]
    fn otoodb() -> Result<()> {
        if std::path::Path::new(FILE_PATH).is_file() {
            fs::remove_file(FILE_PATH)?;
        }

        let mut packet = [([0u8; 4], [0u8; 32]); 250];
        let mut i = 1u8;
        for p in packet.iter_mut() {
            *p = ([i + 1, i + 2, i + 3, i + 4], [i + 5; 32]);
            i += 1;
        }

        // test: define
        {
            let mut db = DB::open(FILE_PATH, 4, 32, None)?;

            for p in packet.iter() {
                db.define(&p.0, &p.1)?;
            }

            let (mut a, mut b);
            for p in packet.iter() {
                a = db.get(&p.0)?.unwrap();
                b = db.get(&p.1)?.unwrap();
                assert_eq!(a, p.1.to_vec());
                assert_eq!(b, p.0.to_vec());
            }
            db.close()?;
        }

        // test: undefine
        {
            let mut db = DB::open(FILE_PATH, 4, 32, None)?;
            let (mut a, mut b);
            let (mut a_, mut b_);
            for (i, p) in packet.iter().enumerate() {
                if i >= 10 {
                    break;
                }
                a = db.get(&p.0)?.unwrap();
                b = db.get(&p.1)?.unwrap();
                assert_eq!(a, p.1.to_vec());
                assert_eq!(b, p.0.to_vec());
                db.undefine(&p.0)?;
                a_ = db.get(&p.0)?;
                b_ = db.get(&p.1)?;
                assert_eq!(a_, None);
                assert_eq!(b_, None);
            }
            db.close()?;
        }
        let _ = DB::open(FILE_PATH, 4, 32, None)?;
        // db.debug();
        fs::remove_file(FILE_PATH)?;
        Ok(())
    }
}
