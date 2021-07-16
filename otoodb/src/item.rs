/*
    .. + item.rs + ..

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

use utils::fs::types::*;

#[derive(Debug)]
pub struct ItemPointer {
    pub mid: LS,
    pub upwards: bool,
    pub pos: uPS,
    pub a_len: LS,
    pub b_len: LS,
}
impl ItemPointer {
    pub fn new(mid: LS, upwards: bool, pos: uPS, a_len: LS, b_len: LS) -> Self {
        Self {
            mid,
            upwards,
            pos,
            a_len,
            b_len,
        }
    }
}
