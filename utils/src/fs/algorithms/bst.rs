/*
    .. + bst.rs + ..

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

use crate::{
    fs::types::*,
    system::*,
    types::{Lim, VLim},
};

pub trait OrderedFile {}

pub struct BST<'a, T> {
    fm: &'a mut FM<T>,
    file_lim: Lim<uPS>,
    buf: Vec<u8>,
    elem_lim: VLim,
}

impl<'a, T> BST<'a, T>
where
    T: OrderedFile,
{
    pub fn new(fm: &'a mut FM<T>, file_lim: Lim<uPS>, elem_lim: VLim) -> Self {
        let buf = elem_lim.create::<u8>();
        Self {
            fm,
            file_lim,
            buf,
            elem_lim,
        }
    }
    pub fn search(&self, target: &[u8]) -> Option<uPS> {}
}
