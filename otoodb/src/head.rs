/*
    .. + head.rs + ..

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

//! Header Configures for One to One Database

use utils::macros::fs::*;

// *** warning: both must be less than 'usize(=LS)' ***
/// header height size
pub type HHSize = u64;
/// header unit size
pub type HUSize = u32;
// ***********************************************
use utils::fs::types::HeaderTrait;

fheader! {
    /// if content has changed, `hash` would be an empty(=all zeros)
    pub struct OtooHeader {
        pub hash: [u8; 32] => [0_u8; 32],
        pub current_height: HHSize => 0,
        a_set_bytes: HUSize,
        b_set_bytes: HUSize,
    }
}

/// (hash, height)
pub type Version = ([u8; 32], HHSize);

impl OrderedFile for OtooHeader {} // for bst
