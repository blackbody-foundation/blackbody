/*
    .. + errors.rs + ..

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

pub use utils_results::*;

err! {
    ValidationFailed => "validation failed."
    BrokenHeader => "broken header."
    AnotherHeader => "not matched header."
    BrokenContent => "broken content."
    FileNotFound => "file not found."
    InvalidLenSize => "invalid target len."
    EmptyArgument => "empty argument."
    MysteriousError => "mysterious error occurs."
    UnexpectedEof => "unexpected eof."
    Interrupted => "interrupted."
    UnwrapingError => "unwrap failed."
    OutOfBounds => "index out of bounds."
    ThreadSending => "thread sending error."
    ThreadReceiving => "thread receiving error."
    UnexpectedVersion => "version is not matched."
}

io_err! {
    UnexpectedEof => err::UnexpectedEof
    Interrupted => err::Interrupted
}
