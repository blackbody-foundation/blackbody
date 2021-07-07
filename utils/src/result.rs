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

use crate::macros::errors;
use crate::types::epool::Pool;
use std::{error, result};

/// public result
pub type Result<T> = result::Result<T, Box<dyn error::Error>>;
pub type ErrPool = Pool<Box<dyn error::Error>>;

errors! {
    pub enum Error {
        BrokenHeader => "broken header.",
        AnotherHeader => "not matched header.",
        FileNotFound => "file not found.",
    }
}

// impl Error {
//     pub fn as_string(&self) -> &'static str {
//         match self {
//             Self::BrokenHeader => "broken header.",
//             Self::AnotherHeader => "not matched header.",
//             Self::FileNotFound => "file not found.",
//         }
//     }
// }

// #[derive(Debug)]
// pub struct Error(ErrKind);
// impl Error {
//     /// make new Error.
//     pub fn bang<T>(kind: ErrKind) -> Result<T> {
//         Result::<T>::Err(Box::new(Self(kind)))
//     }
//     /// discriminate and extract between Ok(T) and Err(ErrPool).
//     pub fn extract<Ok>(r: Result<Ok>) -> (OptionOk<Ok>, OptionErr<ErrPool>) {
//         match r {
//             Ok(t) => (OkOk(t), ErrNone),
//             Err(e) => {
//                 if let Some(kind) = e.source() {
//                     if kind.is::<ErrKind>() {
//                         (OkNone, ErrErr(ErrPool::My(e)))
//                     } else {
//                         (OkNone, ErrErr(ErrPool::Others(e)))
//                     }
//                 } else {
//                     (OkNone, ErrNone)
//                 }
//             }
//         }
//     }
// }
// impl fmt::Display for Error {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "[{}:{}] {}", file!(), line!(), self.0)
//     }
// }
// impl error::Error for Error {
//     fn source(&self) -> Option<&(dyn error::Error + 'static)> {
//         Some(&self.0)
//     }
// }

pub enum OptionOk<T> {
    OkOk(T),
    OkNone,
}
pub enum OptionErr<T> {
    ErrErr(T),
    ErrNone,
}

pub use OptionErr::{ErrErr, ErrNone};
pub use OptionOk::{OkNone, OkOk};
