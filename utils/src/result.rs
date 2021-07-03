/*
    .. + result.rs + ..

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

use std::io;

pub type Result<T> = io::Result<T>;
pub type Error = std::io::Error;

use std::io::ErrorKind;

fn new_error(e_kind: ErrorKind, error: &str) -> Error {
    Error::new(e_kind, error)
}

pub enum ErrKind {
    BrokenHeader,
    FileNotFound,
}
use ErrKind::*;

pub trait Err {
    fn bang(kind: ErrKind) -> Error {
        match kind {
            BrokenHeader => new_error(ErrorKind::InvalidData, "broken header."),
            FileNotFound => new_error(ErrorKind::NotFound, "file not found."),
        }
    }
}
impl Err for Error {}
