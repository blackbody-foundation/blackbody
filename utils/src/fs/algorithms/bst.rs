/*
    .. + bst.rs + ..

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

use crate::{
    fs::types::*,
    system::*,
    types::{bytes::*, Lim, VLim},
};

/// (Option\<ByteVec\>, uPS)
pub type SearchResult = (Option<Vec<u8>>, uPS);

/// default byte order = `little endian` (whenever can change)
#[derive(Debug, Clone)]
pub struct BST {
    file_lim: Lim<uPS>, // searchable range
    elem_lim: VLim<u8>, // a pair data element had (start, mid, end)
    buf: Vec<u8>,       // temporary buf vector
    width: uPS,
    pub byte_order: ByteOrder,
}

impl BST {
    pub fn new(file_lim: Lim<uPS>, elem_lim: VLim<u8>) -> Result<Self> {
        if Self::check_lens(&file_lim, &elem_lim) {
            let buf = elem_lim.create::<u8>();
            let width = (file_lim.end - file_lim.start) / elem_lim.width() as uPS;
            let byte_order = ByteOrder::LittleEndian; // *
            Ok(Self {
                file_lim,
                elem_lim,
                buf,
                width,
                byte_order,
            })
        } else {
            errbang!(err::InvalidLenSize)
        }
    }
    fn check_lens(file_lim: &Lim<uPS>, elem_lim: &VLim<u8>) -> bool {
        let (file_len, elem_len) = (file_lim.end - file_lim.start, elem_lim.width() as uPS);
        file_len % elem_len == 0
    }
    pub fn file_lim(&self) -> &Lim<uPS> {
        &self.file_lim
    }
    pub fn elem_lim(&self) -> &VLim<u8> {
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
    pub fn get_buf_by_right(&mut self) -> &[u8] {
        if self.elem_lim.right {
            self.buf_reversed_right_limed()
        } else {
            self.buf_right_limed()
        }
    }
    pub fn width(&self) -> uPS {
        self.width
    }
    pub fn change_file_lim(&mut self, file_lim: Lim<uPS>) -> Result<()> {
        match Self::check_lens(&file_lim, &self.elem_lim) {
            true => {
                self.file_lim = file_lim;
                self.flash_width();
                Ok(())
            }
            false => errbang!(err::InvalidLenSize),
        }
    }
    pub fn change_elem_lim(&mut self, elem_lim: VLim<u8>) -> Result<()> {
        match Self::check_lens(&self.file_lim, &elem_lim) {
            true => {
                self.elem_lim = elem_lim;
                self.buf = self.elem_lim.create::<u8>();
                self.flash_width();
                Ok(())
            }
            false => errbang!(err::InvalidLenSize),
        }
    }
    pub fn change_both(&mut self, file_lim: Lim<uPS>, elem_lim: VLim<u8>) -> Result<()> {
        if Self::check_lens(&file_lim, &elem_lim) {
            self.file_lim = file_lim;
            self.elem_lim = elem_lim;
            self.buf = self.elem_lim.create::<u8>();
            self.flash_width();
            Ok(())
        } else {
            errbang!(err::InvalidLenSize)
        }
    }
    pub fn get_right(&mut self) -> bool {
        self.elem_lim.right
    }
    pub fn set_right(&mut self, right: bool) {
        self.elem_lim.right = right;
    }
    /// ## Return
    /// `has changed` : bool
    pub fn fit_the_right(&mut self, target: &[u8]) -> Result<bool> {
        let new_right = self.elem_lim.is_right_side(target)?;
        if self.elem_lim.right == new_right {
            return Ok(false);
        }
        self.elem_lim.right = new_right;
        Ok(true)
    }
    fn flash_width(&mut self) {
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

        let buf = elem.lim_mut(&mut self.buf)?; // get a chunk

        let elem_total_len = elem.width() as uPS;

        let start = self.file_lim.start; // starting point

        let mut forward;

        let max_bytes = max_bytes_closure!(self.byte_order, a, b);

        // init
        match self.width {
            0 => {
                return Ok((false, start)); // had no element in the whole range
            }
            1 => {
                fm.read_cursoring(buf, start)?;

                forward = target == max_bytes(target, buf);

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
                return Ok((true, pos));
            }

            forward = target == max_bytes(target, buf);

            if low >= high {
                break;
            }
            if forward {
                low = mid + 1;
            } else {
                high = mid - if mid == low { 0 } else { 1 };
            }
        }

        Ok((false, pos + if forward { elem_total_len } else { 0 }))
    }
}

impl Default for BST {
    fn default() -> Self {
        let file_lim = Lim::default();
        let elem_lim = VLim::new(0, 0, 1);
        let buf = elem_lim.create::<u8>();
        let width = (file_lim.end - file_lim.start) / elem_lim.width() as uPS;
        let byte_order = ByteOrder::LittleEndian;
        Self {
            file_lim,
            elem_lim,
            buf,
            width,
            byte_order,
        }
    }
}
