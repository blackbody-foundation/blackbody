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
    db.define(&[1, 2, 3, 4], &[1; 32])?;
    db.close();
    Ok(())
}
