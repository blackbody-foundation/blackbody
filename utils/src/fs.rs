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

pub mod header;
pub mod types;

use super::macros::errbang;
use super::result::*;
use super::types::epool::Pool;
use types::*;

pub struct File {
    pub path: &'static str,
    pub header: Header,
    fm: FM,
}

impl File {
    pub fn open(path: &'static str, mut header: Header) -> Result<Self> {
        let ptr = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(path)?;

        let mut fm = FM::new(ptr)?;

        let eq = Self::cmp_header(&mut fm, &mut header)?;

        if !eq {
            return errbang!(err::AnotherHeader);
        }

        Ok(Self { path, header, fm })
    }
    fn cmp_header(fm: &mut FM, header: &mut Header) -> Result<bool> {
        match Error::extract(header.read(fm)) {
            // reading
            (OkOk(_), ErrNone) => Ok(true),
            (OkNone, ErrErr(ep)) => {
                let (name, kind) = ep.discriminate();
                if name == Pool::My(()) {
                    if kind. ErrKind::AnotherHeader {
                        header.write(fm)?;
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                } else {
                    Err(kind)
                }
            }
            _ => {}
        }
    }
    fn read(&self) -> Result<()> {
        Ok(())
    }
    fn write(&self) -> Result<()> {
        Ok(())
    }
    pub fn close(self) {}
}
