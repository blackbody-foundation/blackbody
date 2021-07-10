/*
    .. + head.rs + ..

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

//! Header Configures for One to One Database
use utils::macros::*;

pub type HHSize = u64;
pub type HUSize = u32;

use utils::fs::types::HeaderTrait;

fheader! {
    pub struct OtooHeader {
        pub current_height: HHSize => 0, // free marked
        a_set_bytes: HUSize => 4,
        b_set_bytes: HUSize => 32,
    }
}
