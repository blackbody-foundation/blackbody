/*
    .. + macros + ..

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

pub use crate::system::*;
pub use bincode;
pub use serde::{Deserialize, Serialize};
pub use Box;

pub mod bytes;
pub mod derives;
pub mod epool;
pub mod errbang;
pub mod fs;

pub mod gost;

pub use derives::*;
pub use errbang::*;
