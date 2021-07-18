/*
    .. + main.rs + ..

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

// blackhole (file) or (file.cccs)
// cccs = collapsed and compressed core star

use utils::system::*;
use wormhole::DB;
fn main() -> Result<()> {
    let console = Console::new();
    let mut db = DB::open("test", 4, 32)?;
    db.concentric(Some(console));
    // db.define(&[2, 2, 3, 5], &[3; 32])?;
    db.define(&[6, 6, 4, 3], &[0; 32])?;
    // db.define(&[1, 2, 3, 4], &[1; 32])?;
    // db.define(&[5, 6, 7, 8], &[2; 32])?;
    // let mut a = db.get(&[1, 2, 3, 4])?;
    // let mut c = db.get(&[1; 32])?;
    // eprintln!("[1;32]: {:?}\n\n [1,2,3,4]: {:?}", c, a);
    // a = db.get(&[5, 6, 7, 8])?;
    // c = db.get(&[2; 32])?;
    // eprintln!("[2;32]: {:?}\n\n [5,6,7,8]: {:?}", c, a);
    // a = db.get(&[2, 2, 3, 5])?;
    // c = db.get(&[3; 32])?;
    // eprintln!("[3;32]: {:?}\n\n [2,2,3,5]: {:?}", c, a);
    // a = db.get(&[6, 6, 4, 3])?;
    // c = db.get(&[4; 32])?;
    // eprintln!("[4;32]: {:?}\n\n [6,6,4,3]: {:?}", c, a);
    db.debug();
    db.close();
    Ok(())
}
