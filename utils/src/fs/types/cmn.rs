/*
    .. + cmn.rs + ..

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

//! common

pub use crate::types::{Lim, VLim};
pub use crate::{system::*, types::CHUNK_SIZE};

pub use std::fs::File;
pub use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Stdin, Stdout, Write};

/// *** warning ***
#[allow(non_camel_case_types)]
pub type uPS = u64; // pos size
#[allow(non_camel_case_types)]
pub type iPS = i64;
pub type LS = usize; // len size
/// **************************

pub type Packet = Vec<(Vec<u8>, uPS)>;

pub trait OrderedFile {}
