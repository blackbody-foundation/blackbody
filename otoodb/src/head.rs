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

use std::io::{self, Read, Write};

use utils::{
    fs::{header::*, types::*},
    macros::*,
    system::*,
};

/// header unit size.
pub type HSize = u64;
pub type SSize = u32;

fheader! {
    struct Content {
        height: HSize => 0,
        a_set_bytes: SSize => 4,
        b_set_bytes: SSize => 32,
    }
}

impl<'de> TContent<'de> for Content {}

#[derive(Debug)]
pub struct Header {
    content: Content,
}
impl Header {
    pub fn new(a_set_bytes: SSize, b_set_bytes: SSize) -> Box<Self> {
        Box::new(Self {
            content: Content {
                height: 0,
                a_set_bytes,
                b_set_bytes,
            },
        })
    }
}
impl THeader for Header {
    fn write(&mut self, fm: &mut FM) -> Result<()> {
        let writer = &mut fm.writer;

        let buf = self.content.encode()?;

        writer.write_all(&buf)?;

        Ok(())
    }
    fn read(&mut self, fm: &mut FM) -> Result<()> {
        let reader = &mut fm.reader;

        let src = self.content.encode()?;
        let mut buf = vec![0; src.len()];

        match reader.read_exact(&mut buf[..]) {
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                return self.write(fm);
            }
            Err(_) => return errbang!(err::BrokenHeader),
            _ => {}
        }

        let eq = src.iter().zip(buf.iter()).all(|(&x, &y)| x == y);

        if !eq {
            return errbang!(err::AnotherHeader);
        }

        self.content = self.content.decode(&buf)?;

        Ok(())
    }
}
