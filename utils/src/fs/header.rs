/*
    .. + header.rs + ..

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

use std::fmt::Debug;

use super::types::*;
use crate::result::*;

pub trait Header: Debug {
    fn as_bytes(&self) -> Vec<u8>;
    fn write(&self, fm: FM) -> Result<FM>;
    fn read(&mut self, fm: FM) -> Result<FM>;

    /// get first and last bytes for the proof.
    fn get_flu8(&self, buf: &[u8]) -> Option<(u8, u8)> {
        let len = buf.len();
        let f = buf.get(0)?.to_owned();
        let l = buf.get(len - 1)?.to_owned();
        Some((f, l))
    }
    fn check_flu8(&self, buf: &[u8], first: u8, last: u8) -> Result<()> {
        match self.get_flu8(buf) {
            Some((f, l)) if (f == first && l == last) => Ok(()),
            _ => Error::bang(ErrKind::BrokenHeader),
        }
    }
}
