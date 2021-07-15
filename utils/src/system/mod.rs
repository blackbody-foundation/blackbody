/*
    .. + system + ..

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

use std::{error, result};

pub mod errors;
pub use errors::err;

pub mod timer;
pub use timer::Timer;

pub type ErrPool = super::types::epool::Pool<Box<dyn error::Error>>;
pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub type ResultSend<T> = result::Result<T, Box<dyn error::Error + Send + Sync>>;

pub mod console;
pub use console::Console;

#[macro_use]
pub mod concentric;
pub use concentric::*;
