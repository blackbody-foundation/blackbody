/*
    .. + bytes.rs + ..

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

pub use primitive_types::U512;

pub fn max_bytes<'a>(b1: &'a [u8], b2: &'a [u8]) -> &'a [u8] {
    let a = U512::from_little_endian(b1);
    let b = U512::from_little_endian(b2);
    if a > b {
        b1
    } else {
        b2
    }
}
