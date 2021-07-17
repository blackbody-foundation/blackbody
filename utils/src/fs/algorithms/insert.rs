/*
    .. + insert.rs + ..

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

use crate::fs::types::{HeaderTrait, Packet, FM};
use crate::system::*;

use super::bst::BST;

pub fn insert<T: HeaderTrait>(fm: &mut FM<T>, mut packet: Packet) -> Result<()> {
    packet.sort_by_key(|k| k.1); // sort by position in the file

    let reader = fm.try_to_create_reader()?;
    let writer = fm.try_to_create_writer()?;

    let bst = BST::default();

    Ok(())
}
