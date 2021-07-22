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
    types::{
        bytes::{max_bytes, U512},
        Lim, MBox, VLim,
    },
};

#[derive(Debug)]
pub struct BST {
    file_lim: Lim<uPS>, // searchable range
    elem_lim: VLim,     // a pair data element had (start, mid, end)
    buf: Vec<u8>,       // temporary buf vector
    width: uPS,
}

impl BST {
    pub fn new(file_lim: Lim<uPS>, elem_lim: VLim) -> Result<Self> {
        if Self::check_lens(&file_lim, &elem_lim) {
            let buf = elem_lim.create::<u8>();
            let width = (file_lim.end - file_lim.start) / elem_lim.width() as uPS;
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
        let (file_len, elem_len) = (file_lim.end - file_lim.start, elem_lim.width() as uPS);
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
                self.flush_width();
                Ok(())
            }
            false => errbang!(err::InvalidLenSize),
        }
    }
    pub fn change_elem_lim(&mut self, elem_lim: VLim) -> Result<()> {
        match Self::check_lens(&self.file_lim, &elem_lim) {
            true => {
                self.elem_lim = elem_lim;
                self.buf = self.elem_lim.create::<u8>();
                self.flush_width();
                Ok(())
            }
            false => errbang!(err::InvalidLenSize),
        }
    }
    pub fn change_both(&mut self, file_lim: Lim<uPS>, elem_lim: VLim) -> Result<()> {
        if Self::check_lens(&file_lim, &elem_lim) {
            self.file_lim = file_lim;
            self.elem_lim = elem_lim;
            self.buf = self.elem_lim.create::<u8>();
            self.flush_width();
            Ok(())
        } else {
            errbang!(err::InvalidLenSize)
        }
    }
    fn flush_width(&mut self) {
        self.width = (self.file_lim.end - self.file_lim.start) / self.elem_lim.width() as uPS;
    }
    pub fn validate(&self) -> Result<()> {
        todo!()
    }
    pub fn search<T>(&mut self, fm: &mut FM<T>, target: &[u8]) -> Result<(bool, uPS)>
    where
        T: HeaderTrait + OrderedFile,
    {
        let elem = &mut self.elem_lim;
        let m = MBox::new(&elem.right); // memorizing

        elem.right = elem.is_right_side(target)?;

        let buf = elem.mut_lim(&mut self.buf)?; // get a chunk

        let elem_total_len = elem.width() as uPS;

        let start = self.file_lim.start; // starting point

        let mut forward;

        // init
        match self.width {
            0 => {
                return Ok((false, start)); // had no element in the whole range
            }
            1 => {
                fm.read_cursoring(buf, start)?;

                forward = target == max_bytes![target, buf]?;

                m.to(&mut elem.right); // returning previous value (elem.right)
                if target == buf {
                    return Ok((true, start));
                } else {
                    return Ok((false, start + if forward { elem_total_len } else { 0 }));
                }
            }
            _ => {}
        }

        let (mut low, mut high) = (0, self.width - 1);

        let (mut mid, mut pos);

        loop {
            mid = low + ((high - low) / 2);

            pos = start + mid * elem_total_len;

            fm.read_cursoring(buf, pos)?;

            if target == buf {
                m.to(&mut elem.right);
                return Ok((true, pos));
            }

            forward = target == max_bytes![target, buf]?;

            if low >= high {
                break;
            }
            if forward {
                low = mid + 1;
            } else {
                high = mid - if mid == 0 { 0 } else { 1 };
            }
        }

        m.to(&mut elem.right);
        Ok((false, pos + if forward { elem_total_len } else { 0 }))
    }
}

impl Default for BST {
    fn default() -> Self {
        let file_lim = Lim::new(0, 1);
        let elem_lim = VLim::new(0, 0, 1);
        let buf = elem_lim.create::<u8>();
        let width = (file_lim.end - file_lim.start) / elem_lim.width() as uPS;
        Self {
            file_lim,
            elem_lim,
            buf,
            width,
        }
    }
}
