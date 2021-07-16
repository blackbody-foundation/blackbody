/*
    .. + lim.rs + ..

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

use crate::fs::types::LS;
use crate::{errbang, system::*};

pub struct Lim<T> {
    pub start: T,
    pub end: T,
}
impl<T> Lim<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
}
impl<T> Into<(T, T)> for Lim<T> {
    fn into(self) -> (T, T) {
        (self.start, self.end)
    }
}

pub struct VLim {
    pub start: LS,
    pub mid: LS,
    pub end: LS,
}
impl VLim {
    pub fn new(start: LS, mid: LS, end: LS) -> Self {
        Self { start, mid, end }
    }
    pub fn lim<'a, T>(&self, v: &'a [T]) -> Result<(&'a [T], &'a [T])> {
        if v.len() < self.end {
            errbang!(err::OutOfBounds)
        } else {
            Ok(v.split_at(self.mid))
        }
    }
    pub fn mut_lim<'a, T>(&self, v: &'a mut [T]) -> Result<(&'a mut [T], &'a mut [T])> {
        if v.len() < self.end {
            errbang!(err::OutOfBounds)
        } else {
            Ok(v.split_at_mut(self.mid))
        }
    }
    pub fn create<T: Default + Clone>(&self) -> Vec<T> {
        vec![T::default(); self.end]
    }
}
impl Into<(LS, LS, LS)> for VLim {
    fn into(self) -> (LS, LS, LS) {
        (self.start, self.mid, self.end)
    }
}
