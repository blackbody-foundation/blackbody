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
    errbang,
    fs::types::*,
    system::*,
    types::{Lim, RMBox, VLim},
};

pub trait OrderedFile {}

pub struct BST<'a, T> {
    fm: &'a mut FM<T>,
    file_lim: Lim<uPS>,
    elem_lim: VLim,
    buf: Vec<u8>,
    width: uPS,
}

impl<'a, T> BST<'a, T>
where
    T: OrderedFile,
{
    pub fn new(fm: &'a mut FM<T>, file_lim: Lim<uPS>, elem_lim: VLim) -> Result<Self> {
        let (file_len, elem_len) = (file_lim.end, elem_lim.end as uPS);

        if file_len % elem_len == 0 {
            let buf = elem_lim.create::<u8>();
            Ok(Self {
                fm,
                file_lim,
                elem_lim,
                buf,
                width: file_len / elem_len,
            })
        } else {
            errbang!(
                err::InvalidLenSize,
                "invalid matched : (file_lim.end % elem_lim.end) != 0"
            )
        }
    }
    pub fn search(&mut self, target: &[u8]) -> Result<Option<uPS>> {
        let right = self.elem_lim.is_right_side(target)?;
        let mut rmb = RMBox::new(&mut self.elem_lim.right);
        *rmb = right;

        

        Ok(None)
    }
}
