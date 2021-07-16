/*
    .. + fs.rs + ..

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

pub use super::*;
pub use crate::fs::types::*;
pub use crate::system::*;
pub use std::io::{self, Read, Seek, SeekFrom, Write};
///```
/// fheader! {
///     pub struct Name {
///         pub h: u64 => 0, // "pub" is free mark
///         a: u32 => 4,
///         b: u32 => 32,
///     }
/// }
///```
#[macro_export]
macro_rules! fheader {
    (

        $vis:vis struct $name:ident {

            $($free_mark:vis $var:ident: $t:ty),*,

        }

    ) => {


        serialize! {

            $vis struct $name {

                $($vis $var: $t),*

            }

        }

        impl $name
        where Self: HeaderTrait {

            $vis fn new($($var: $t),*) -> Box<$name> {
                Box::new(Self {
                    $($var),*
                })
            }

        }

        impl HeaderTrait for $name {

            fn read(&mut self, ptr: &mut Ptr) -> Result<LS> {

                let src = bincode::serialize(&self)?;

                let dst = Self::read_header_bytes(ptr, &src)?;

                Self::check_header_protocol(&src, &dst)?;

                *self = bincode::deserialize(&dst)?;

                Ok(dst.len())

            }
            fn write(&mut self, ptr: &mut Ptr) -> Result<LS> {

                let mut src = bincode::serialize(&self)?;

                let dst = Self::read_header_bytes(ptr, &src)?;

                Self::check_header_protocol(&src, &dst)?;

                ptr.seek(SeekFrom::Start(0))?;
                ptr.write_all(&src)?;

                Ok(src.len())

            }
        }

        impl $name {
            fn is_eof(ptr: &mut Ptr) -> Result<bool> {
                if 0 == ptr.read(&mut [0u8; 1])? {
                    Ok(true)
                } else {
                    ptr.seek(SeekFrom::Current(-1))?;
                    Ok(false)
                }
            }
            fn read_header_bytes(ptr: &mut Ptr, src: &[u8]) -> Result<Vec<u8>> {

                let mut buf = vec![0; src.len()];

                ptr.seek(SeekFrom::Start(0))?;
                match ptr.read_exact(&mut buf[..]) {

                    Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                        if Self::is_eof(ptr)? {
                            ptr.seek(SeekFrom::Start(0))?;
                            ptr.write_all(&src)?; // create
                            Ok(src.clone().to_vec())
                        } else {
                            errbang!(err::BrokenHeader)
                        }
                    }
                    Err(_) => errbang!(err::BrokenHeader),
                    _ => Ok(buf)

                }

            }
            fn check_header_protocol(a_just_head_bytes: &[u8], b_just_head_bytes: &[u8]) -> Result<()> {

                if a_just_head_bytes.len() != b_just_head_bytes.len() { return errbang!(err::AnotherHeader); }

                let mut cursor = 0;

                $(
                    let width = std::mem::size_of::<$t>() - 1;
                    if stringify!(($free_mark)) != "(pub )" {
                        if &a_just_head_bytes[cursor..cursor+width] != &b_just_head_bytes[cursor..cursor+width] {
                            return errbang!(err::AnotherHeader);
                        }
                    }
                    cursor += width;
                )*

                Ok(())
            }
        }

    }

}

pub use fheader;
