/*
    .. + process.rs + ..

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

use super::cmn::*;

pub struct OtooDB(pub DB);

impl OtooDB {
    /// (a_set_bytes, b_set_bytes): (LS, LS)
    pub fn get_info(&self) -> (LS, LS) {
        let (_, src_bytes_size, dst_bytes_size) = self.0.get_info();
        (src_bytes_size, dst_bytes_size)
    }

    /// *** if any error occurs then panic! ***
    pub fn transform(&mut self, src_bytes: &[u8]) -> Option<Vec<u8>> {
        self.0
            .get(src_bytes)
            .expect("cannot transform some of src_bytes. &[u8]")
    }
}

pub struct File(pub std::fs::File);
