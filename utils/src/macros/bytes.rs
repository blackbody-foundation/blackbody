/*
    .. + bytes.rs + ..

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

/// Bytes length match macro
///
/// ```no_run
/// let len = is_bytes_len![&[0,0,0], 2, 3, 6];
/// assert_eq!(len, Some(3));
/// ```
#[macro_export]
macro_rules! is_bytes_len {
    (
        $bytes:ident, $len0:expr $(, $len:expr)*
    ) => {
        vec![$len0:expr $(, $len:expr)*].into_iter().find(|&x| x == $bytes.len())
    };
}

/// Picking maximum bytes ('little endian')
///
/// ```no_run
/// let max = max_le_bytes![&[0,1,2], &[1,2,3], &[3,2,1]];
/// assert_eq!(max, &[1,2,3]);
/// ```
#[macro_export]
macro_rules! max_le_bytes {
    (
        $bytes0:expr$(, $bytes:expr)+
    ) => {
        {
            {
                let list = vec![$bytes0, $($bytes),*];
                let (mut max_len, mut max_i) = (list[0].len(), 0);
                let mut len;

                for (mut i, bytes) in list.iter().skip(1).enumerate() {
                    i += 1;

                    len = bytes.len();

                    if max_len < len {

                        max_len = len;
                        max_i = i;

                    } else if max_len == len {
                        for j in (0..len).rev() {

                            if list[max_i][j] < list[i][j]  {
                                max_i = i;
                            } else if list[max_i][j] > list[i][j] {
                                break;
                            }

                        }
                    }

                }
                list[max_i]
            }
        }
    };
}

/// Picking maximum bytes ('big endian')
///
/// ```no_run
/// let max = max_be_bytes![&[0,1,2], &[1,2,3], &[3,2,1]];
/// assert_eq!(max, &[3,2,1]);
/// ```
#[macro_export]
macro_rules! max_be_bytes {
    (
        $bytes0:expr$(, $bytes:expr)+
    ) => {
        {
            {
                let list = vec![$bytes0, $($bytes),*];
                let (mut max_len, mut max_i) = (list[0].len(), 0);
                let mut len;

                for (mut i, bytes) in list.iter().skip(1).enumerate() {
                    i += 1;

                    len = bytes.len();

                    if max_len < len {

                        max_len = len;
                        max_i = i;

                    } else if max_len == len {
                        for j in 0..len {

                            if list[max_i][j] < list[i][j]  {
                                max_i = i;
                            } else if list[max_i][j] > list[i][j] {
                                break;
                            }

                        }
                    }

                }
                list[max_i]
            }
        }
    };
}

/// Get the closure
///
/// ```no_run
/// let max_bytes = max_bytes_closure!(ByteOrder::, a, b, ...);
/// ```
#[macro_export]
macro_rules! max_bytes_closure {
    (
        $byte_order: expr,
        $($var:ident),+
    ) => {
        {
            let __max_bytes__: for<'a> fn($($var:&'a [u8]),+) -> &'a [u8] = match $byte_order {
                ByteOrder::LittleEndian => |$($var),+| max_le_bytes![$($var),+],
                ByteOrder::BigEndian => |$($var),+| max_be_bytes![$($var),+]
            };
            __max_bytes__
        }
    };
}

/// ```no_run
/// bincode::serialize(t)
/// ```
#[macro_export]
macro_rules! serialize_t {
    (
        $t:expr
    ) => {
        bincode::serialize($t)
    };
}
/// ```no_run
/// bincode::deserialize::<T>(t)?
/// ```
#[macro_export]
macro_rules! deserialize_t {
    (
        $t:expr
    ) => {
        bincode::deserialize::<T>($t)
    };
}
/// ```no_run
/// match t {
///     Some(v) => Ok(Some(bincode::serialize(&v)?)),
///     _ => Ok(None),
/// }
/// ```
#[macro_export]
macro_rules! serialize_option_t {
    (
        $t:expr
    ) => {
        match $t {
            Some(v) => Ok(Some(bincode::serialize(&v)?)),
            _ => Ok(None),
        }
    };
}
/// ```no_run
/// match t {
///     Some(v) => Ok(Some(bincode::deserialize::<T>(&v)?)),
///     _ => Ok(None),
/// }
/// ```
#[macro_export]
macro_rules! deserialize_option_t {
    (
        $t:expr
    ) => {
        match $t {
            Some(v) => Ok(Some(bincode::deserialize::<T>(&v)?)),
            _ => Ok(None),
        }
    };
}

pub use is_bytes_len;
pub use max_be_bytes;
pub use max_bytes_closure;
pub use max_le_bytes;

pub use deserialize_option_t;
pub use deserialize_t;
pub use serialize_option_t;
pub use serialize_t;
