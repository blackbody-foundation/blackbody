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

pub use crate::types::bytes::U512;

/// Bytes length match macro
///
/// ```rust
/// let len = is_bytes_len![&[0,0,0], 2, 3, 6];
/// assert_eq!(len, 3);
/// ```
///
/// - if the match is invalid, return Err(err::InvalidLenSize)
///
#[macro_export]
macro_rules! is_bytes_len {
    (
        $bytes:ident, $len0:expr $(, $len:expr)*
    ) => {
            if $bytes.len() == $len0 {
                Result::Ok($len0)
            } $(else if $bytes.len() == $len {
                Result::Ok($len)
            })* else {
                errbang!(err::InvalidLenSize)
            }
    };
}

/// Picking maximum bytes (by U512 'little endian')
///
/// ```rust
/// let max = max_bytes![&[0,0,0], &[1,2,3], &[3,2,1]];
/// assert_eq!(max, &[1,2,3]);
/// ```
///
/// - If several elements are equally maximum, the last element is returned.
/// - If the arguments place is empty, Err(err::EmptyArgument) is returned.
///
#[macro_export]
macro_rules! max_bytes {
    (
        $bytes0:expr$(, $bytes:expr)*
    ) => {
        {
            match vec![
                ($bytes0, U512::from_little_endian($bytes0)),
                $(
                    ($bytes, U512::from_little_endian($bytes))
                ),*
            ].into_iter().max_by_key(|x| x.1) {
                Some(x) => Result::Ok(x.0),
                None => errbang!(err::EmptyArgument)
            }
        }
    };
}

pub use is_bytes_len;
pub use max_bytes;
