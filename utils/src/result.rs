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

use std::fmt;
use std::{error, result};

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct Error(ErrKind);
impl Error {
    pub fn bang<T>(kind: ErrKind) -> Result<T> {
        Result::<T>::Err(Box::new(Self(kind)))
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
impl error::Error for Error {}

#[derive(Debug, Clone, Copy)]
pub enum ErrKind {
    BrokenHeader,
    FileNotFound,
}
impl ErrKind {
    pub fn to_string(self) -> &'static str {
        match self {
            Self::BrokenHeader => "entity not found",
            Self::FileNotFound => "file not found.",
        }
    }
}
