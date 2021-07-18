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
    max_bytes,
    system::*,
    types::{bytes::U512, Lim, MBox, VLim},
};

#[derive(Debug)]
pub struct BST {
    file_lim: Lim<uPS>,
    elem_lim: VLim,
    buf: Vec<u8>,
    width: uPS,
}

impl BST {
    pub fn new(file_lim: Lim<uPS>, elem_lim: VLim) -> Result<Self> {
        if Self::check_lens(&file_lim, &elem_lim) {
            let buf = elem_lim.create::<u8>();
            let width = file_lim.end / elem_lim.end as uPS;
            Ok(Self {
                file_lim,
                elem_lim,
                buf,
                width,
            })
        } else {
            errbang!(err::InvalidLenSize)
        }
    }
    fn check_lens(file_lim: &Lim<uPS>, elem_lim: &VLim) -> bool {
        let (file_len, elem_len) = (file_lim.end, elem_lim.end as uPS);
        file_len % elem_len == 0
    }
    pub fn file_lim(&self) -> &Lim<uPS> {
        &self.file_lim
    }
    pub fn elem_lim(&self) -> &VLim {
        &self.elem_lim
    }
    pub fn buf_mut(&mut self) -> &mut [u8] {
        &mut self.buf[..]
    }
    pub fn buf_left_limed(&mut self) -> &[u8] {
        &self.buf[..self.elem_lim.mid]
    }
    pub fn buf_right_limed(&mut self) -> &[u8] {
        &self.buf[self.elem_lim.mid..]
    }
    pub fn buf_reversed_left_limed(&mut self) -> &[u8] {
        &self.buf[..(self.elem_lim.end - self.elem_lim.mid)]
    }
    pub fn buf_reversed_right_limed(&mut self) -> &[u8] {
        &self.buf[(self.elem_lim.end - self.elem_lim.mid)..]
    }
    pub fn width(&self) -> uPS {
        self.width
    }
    pub fn change_file_lim(&mut self, file_lim: Lim<uPS>) -> Result<()> {
        match Self::check_lens(&file_lim, &self.elem_lim) {
            true => {
                self.file_lim = file_lim;
                self.change_width();
                Ok(())
            }
            false => errbang!(err::InvalidLenSize),
        }
    }
    pub fn change_elem_lim(&mut self, elem_lim: VLim) -> Result<()> {
        match Self::check_lens(&self.file_lim, &elem_lim) {
            true => {
                self.elem_lim = elem_lim;
                self.change_width();
                Ok(())
            }
            false => errbang!(err::InvalidLenSize),
        }
    }
    pub fn change_both(&mut self, file_lim: Lim<uPS>, elem_lim: VLim) -> Result<()> {
        if Self::check_lens(&file_lim, &elem_lim) {
            self.file_lim = file_lim;
            self.elem_lim = elem_lim;
            self.change_width();
            Ok(())
        } else {
            errbang!(err::InvalidLenSize)
        }
    }
    fn change_width(&mut self) {
        self.width = (self.file_lim.end - self.file_lim.start)
            / (self.elem_lim.end - self.elem_lim.start) as uPS;
    }
    pub fn validate(&self) -> Result<()> {
        todo!()
    }
    pub fn search<T>(&mut self, fm: &mut FM<T>, target: &[u8]) -> Result<(bool, uPS)>
    where
        T: HeaderTrait + OrderedFile,
    {
        let elem = &mut self.elem_lim;
        let m = MBox::new(&elem.right);

        elem.right = elem.is_right_side(target)?;

        let buf = elem.mut_lim(&mut self.buf)?;

        let start = self.file_lim.start; // header size excluded
        let elem_total_len = elem.end as uPS;

        let mut distance = self.width;

        let (mut mid, mut pos) = (0, 0);

        let mut forward = true;

        let mut check_zero = false;
        loop {
            distance /= 2;

            if forward {
                mid += distance + if distance == 0 && check_zero { 1 } else { 0 };
            } else if mid > (distance + 1) {
                mid -= distance - if distance == 0 && check_zero { 1 } else { 0 };
            } else {
                mid = 0;
            }

            pos = start + mid * elem_total_len;
            
            fm.read_cursoring(buf, pos)?;

            if target == buf {
                m.to(&mut elem.right);
                return Ok((true, pos));
            }

            forward = target == max_bytes![target, buf]?;

            if distance == 0 {
                if check_zero || self.width == 1 {
                    m.to(&mut elem.right);
                    pos += if forward { elem_total_len } else { 0 };
                    return Ok((false, pos));
                } else {
                    check_zero = true;
                }
            }
        }
    }
}

impl Default for BST {
    fn default() -> Self {
        let file_lim = Lim::new(0, 1);
        let elem_lim = VLim::new(0, 0, 1);
        let buf = elem_lim.create::<u8>();
        let width = file_lim.end / elem_lim.end as uPS;
        Self {
            file_lim,
            elem_lim,
            buf,
            width,
        }
    }
}
