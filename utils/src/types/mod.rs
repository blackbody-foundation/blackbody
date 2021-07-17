/*
    .. + types + ..

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

pub mod bytes;
pub mod cheque128;
pub mod epool;
pub mod mbuf;
pub mod rmbox;

mod lim;

pub use lim::{Lim, VLim};
pub use rmbox::{MBox, RMBox};

pub const CHUNK_SIZE: usize = 4 * 1024;

pub fn type_of<T>(_: T) -> &'static str {
    std::any::type_name::<T>()
}
