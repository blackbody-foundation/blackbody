/*
    .. + fs.rs + ..

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

pub use super::derives::serde::*;
pub use crate::fs::types::*;
pub use crate::system::*;
pub use crate::types::bytes::*;
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
        @default_or (), $t:ty
    ) => {
        <$t>::default()
    };
    (
        @default_or ($val:expr), $t:ty
    ) => {
        $val
    };
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {

            $($free_mark:vis $var:ident: $t:ty$( => $val:expr)?),*$(,)?

        }

    ) => {


        serialize! {

            $(#[$meta])*
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
            $vis fn default() -> Box<$name> {
                Box::new(Self {
                    $(
                        $var: fheader!(@default_or ($($val)?), $t)
                    ),*
                })
            }

        }


        impl HeaderTrait for $name {
            fn read<R: Read + Seek>(&mut self, ptr: &mut R) -> Result<LS> {

                let src = self.to_bytes()?;

                let dst = Self::read_header_bytes(ptr, &src)?;

                Self::check_header_protocol(&src, &dst)?;

                let res_size = dst.len();

                *self = dst.into_something()?;

                Ok(res_size)

            }
            fn write<P: Read + Write + Seek>(&mut self, ptr: &mut P) -> Result<LS> {

                let mut src = self.to_bytes()?;

                let dst = Self::read_header_bytes(ptr, &src)?;

                Self::check_header_protocol(&src, &dst)?;

                ptr.seek(SeekFrom::Start(0))?;
                ptr.write_all(&src)?;

                Ok(src.len())

            }
            fn overwrite<W: Write + Seek>(&mut self, ptr: &mut W) -> Result<LS> {

                let mut src = self.to_bytes()?;

                ptr.seek(SeekFrom::Start(0))?;
                ptr.write_all(&src)?;

                Ok(src.len())

            }
        }

        impl $name {
            fn read_header_bytes<R: Read + Seek>(ptr: &mut R, src: &[u8]) -> Result<Vec<u8>> {

                let mut buf = vec![0; src.len()];

                ptr.seek(SeekFrom::Start(0))?;
                match ptr.read_exact(&mut buf[..]) {

                    Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof =>
                        errbang!(err::UnexpectedEof),

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
