/*
    .. + header.rs + ..

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

pub type Header = Box<dyn HeaderTrait>;

pub trait HeaderTrait: std::fmt::Debug {
    /// return value is length of bytes for header
    fn read<R: Read + Seek>(&mut self, ptr: &mut R) -> Result<LS>;
    /// return value is length of bytes for header
    fn write<W: Read + Write + Seek>(&mut self, ptr: &mut W) -> Result<LS>;
    /// return value is length of bytes for header
    fn overwrite<W: Write + Seek>(&mut self, ptr: &mut W) -> Result<LS>;
}
