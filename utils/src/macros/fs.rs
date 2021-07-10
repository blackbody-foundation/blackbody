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

pub use crate::fs::types::*;
pub use std::io::{self, Read, Write};

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

            $($free_mark:vis $var:ident: $t:ty => $val:expr),*,

        }

    ) => {

        serialize! {

            $vis struct $name {

                $($vis $var: $t),*

            }

        }

        impl $name {

            $vis fn new() -> Box<Self> {
                Box::new(Self {
                    $($var: $val),*
                })
            }
            $vis fn from($($var: $t),*) -> Box<Self> {
                Box::new(Self {
                    $($var),*
                })
            }

        }


        impl HeaderTrait for $name {
            fn read(&mut self, fm: &mut FM) -> Result<()> {

                let reader = &mut fm.reader;

                let src = bincode::serialize(&self)?;
                let mut buf = vec![0; src.len()];

                match reader.read_exact(&mut buf[..]) {

                    Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                        return self.write(fm); // create
                    }
                    Err(_) => return errbang!(err::BrokenHeader),
                    _ => {}

                }

                let mut cursor = 0;
                $(
                    let width = std::mem::size_of::<$t>() - 1;
                    if stringify!(($free_mark)) != "(pub )" {
                        if &src[cursor..cursor+width] != &buf[cursor..cursor+width] {
                            return errbang!(err::AnotherHeader);
                        }
                    }
                    cursor += width;
                )*

                *self = bincode::deserialize(&buf)?;

                Ok(())

            }
            fn write(&mut self, fm: &mut FM) -> Result<()> {

                let writer = &mut fm.writer;

                let buf = bincode::serialize(&self)?;

                writer.write_all(&buf)?;

                Ok(())

            }
        }

    }

}

pub use fheader;
