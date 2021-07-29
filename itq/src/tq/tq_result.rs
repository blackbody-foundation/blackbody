/*
    .. + tq_result.rs + ..

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

use std::{
    error::Error,
    fmt::{self, Debug},
    result,
};

pub type TQResult = result::Result<(), Box<TQError>>;
pub type TQFn = Box<dyn FnOnce() + Send>;

// must be a 'Something(TQFn)' enum
pub enum TQError {
    Full(TQFn),
}
impl TQError {
    pub fn err(t: Self) -> TQResult {
        Err(Box::new(t))
    }
}
impl fmt::Display for TQError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TQ Error occurs.")
    }
}
impl Error for TQError {}
impl Debug for TQError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error occurs, so returned TQ function.")
    }
}
