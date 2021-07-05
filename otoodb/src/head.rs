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

use std::{convert::TryInto, io::Read};

use utils::{
    fs::{header, types::*},
    result::*,
};

/// header unit size.
pub type HUSize = u64;

/// first and last bytes in the header.
pub const HFB: u8 = 0b10101010u8;
pub const HLB: u8 = !HFB;

#[derive(Debug)]
pub struct Content(&'static str, HUSize);
type Context = Vec<Content>;

#[derive(Debug)]
pub struct Header {
    pub context: Context,
}
impl Header {
    pub fn new(a_set_bytes: HUSize, b_set_bytes: HUSize) -> Self {
        let context = vec![
            Content("height", 0),
            Content("a_set_bytes", a_set_bytes),
            Content("b_set_bytes", b_set_bytes),
        ];
        Self { context }
    }
}
impl header::Header for Header {
    fn as_bytes(&self) -> Vec<u8> {
        self.context
            .iter()
            .map(|x| x.1.to_ne_bytes())
            .flatten()
            .collect::<Vec<u8>>()
    }
    fn write(&self, fm: FM) -> Result<FM> {
        // Ok(())
        Ok(fm)
    }
    fn read(&mut self, mut fm: FM) -> Result<FM> {
        let reader = &mut fm.reader;

        let mut buf = self.as_bytes();
        reader.read_exact(buf.as_mut_slice())?; // read

        self.check_flu8(&buf, HFB, HLB)?;

        let size = std::mem::size_of::<HUSize>();
        let mut cursor;

        for (idx, mut elem) in self.context.iter_mut().enumerate() {
            cursor = idx * size;
            // get bytes
            let bytes = match buf[cursor..(cursor + size)].try_into() {
                Ok(x) => x,
                Err(_) => {
                    return Error::bang(ErrKind::BrokenHeader);
                }
            };
            elem.1 = HUSize::from_ne_bytes(bytes);
        }
        Ok(fm)
    }
}
