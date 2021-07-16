/*
    .. + errors.rs + ..

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

use super::Result;
use crate::macros::errbang::*;

errors! {
    ValidationFailed => "invalid ordering sus."
    BrokenHeader => "broken header."
    AnotherHeader => "not matched header."
    FileNotFound => "file not found."
    InvalidLenSize => "invalid target len."
    EmptyArgument => "empty argument."
    MysteriousError => "mysterious error occurs."
    UnexpectedEof => "unexpected eof."
    Interrupted => "interrupted."
    UnwrapingError => "unwrap failed."
    OutOfBounds => "index out of bounds."
    ThreadSending => "thread sending error."
}

pub fn handle_io_error<T>(io_error: std::io::Result<T>) -> Result<T> {
    match io_error {
        Err(e) => match e.kind() {
            std::io::ErrorKind::UnexpectedEof => errbang!(err::UnexpectedEof),
            std::io::ErrorKind::Interrupted => errbang!(err::Interrupted),
            _ => Err(Box::new(e)),
        },
        Ok(t) => Ok(t),
    }
}
