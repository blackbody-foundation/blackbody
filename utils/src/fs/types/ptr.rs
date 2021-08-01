/*
    .. + ptr.rs + ..

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

use super::*;

impl Ptr for File {}

pub trait Ptr
where
    Self: Read + Write + Seek + std::fmt::Debug,
{
}
pub trait ReadPtr
where
    Self: Read + Seek + std::fmt::Debug,
{
}
pub trait WritePtr
where
    Self: Write + Seek + std::fmt::Debug,
{
}

impl<R: Read + Seek + std::fmt::Debug> ReadPtr for BufReader<R> {}
impl<W: Write + Seek + std::fmt::Debug> WritePtr for BufWriter<W> {}
