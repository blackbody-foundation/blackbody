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

mod fs;
use fs::File;
use std::io;

mod head;
use head::Header;

pub struct ABSetBytes(usize, usize);

pub struct DB {
    pub a_b_bytes: ABSetBytes,
    pub file: File,
}
impl DB {
    pub fn open(
        file_path: &'static str,
        a_set_bytes: usize,
        b_set_bytes: usize,
    ) -> io::Result<Self> {
        Ok(Self {
            a_b_bytes: ABSetBytes(a_set_bytes, b_set_bytes),
            file: File::open(file_path, Box::new(Header::new(a_set_bytes, b_set_bytes)))?,
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
        db.close();
    }
}

// #[allow(..)]

// enum EnFoo {
//     Foo1,
//     Foo2,
//     Foo3,
// }
// impl Default for EnFoo {
//     fn default() -> Self {
//         Self::Foo1
//     }
// }
// struct StFoo {
//     foo1: usize,
//     foo2: String,
//     foo3: u8,
// }
// impl Default for StFoo {
//     fn default() -> Self {
//         Self {
//             foo1: 0,
//             foo2: String::from("0"),
//             foo3: 0,
//         }
//     }
// }

// fn fooo(a1: EnFoo*, a2: StFoo*) {
//     // ...
// }
// fn fooo_b_mut(a1: &mut EnFoo*, a2: &mut StFoo*) {
//     // ...
// }
// fn main() {

//     // ****
//     // EnFoo* <--- if EnFoo == empty place then
//     // Create EnFoo::default()
//     // ****

//     fooo(); /* <--- ==

//     fn fooo(a1: EnFoo*, a2: StFoo*) {
//         if EnFoo == empty then ↴
//         ""let a1 = EnFoo::default();"" <-- Internal Execution

//         if StFoo == empty then ↴
//         ""let a2 = StFoo::default();"" <-- Internal Execution

//         Now can be access
//         a1, a2
//     }
//     */
// }
