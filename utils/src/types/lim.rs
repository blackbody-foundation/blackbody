/*
    .. + lim.rs + ..

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

use crate::system::*;

#[derive(Debug, Default, Clone)]
pub struct Lim<T> {
    pub start: T,
    pub end: T,
}
impl<T: PartialEq + Ord + Clone + Default> Lim<T> {
    pub fn new(start: T, end: T) -> Self {
        Self { start, end }
    }
    pub fn into_(self) -> (T, T) {
        (self.start, self.end)
    }
    pub fn lim(&self, target: T) -> T {
        match target {
            v if v <= self.start => self.start.clone(),
            v if v >= self.end => self.end.clone(),
            _ => target,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VLim<T> {
    pub start: usize,
    pub mid: usize,
    pub end: usize,
    /// *** warning ***
    pub right: bool,
    phantom: std::marker::PhantomData<T>,
}
impl<T> VLim<T> {
    pub fn new(start: usize, mid: usize, end: usize) -> Self {
        Self {
            start,
            mid,
            end,
            right: false,
            phantom: std::marker::PhantomData,
        }
    }
    /// return value is the right (boolean)
    pub fn is_right_side(&self, v: &[T]) -> Result<bool> {
        let v_len = v.len();
        if self.mid == v_len {
            Ok(false)
        } else if (self.end - self.mid) == v_len {
            Ok(true)
        } else {
            errbang!(err::InvalidLenSize)
        }
    }
    pub fn lim<'a>(&self, v: &'a [T]) -> Result<&'a [T]> {
        if v.len() < self.end {
            errbang!(err::OutOfBounds)
        } else {
            match v.split_at(self.mid) {
                s if self.right => Ok(s.1),
                s => Ok(s.0),
            }
        }
    }
    pub fn lim_mut<'a>(&self, v: &'a mut [T]) -> Result<&'a mut [T]> {
        if v.len() < self.end {
            errbang!(err::OutOfBounds)
        } else {
            match v.split_at_mut(self.mid) {
                s if self.right => Ok(s.1),
                s => Ok(s.0),
            }
        }
    }
    pub fn create<C: Default + Clone>(&self) -> Vec<C> {
        vec![C::default(); self.end]
    }
    pub fn width(&self) -> u64 {
        (self.end - self.start) as u64
    }
    pub fn into_(self) -> (usize, usize, usize) {
        (self.start, self.mid, self.end)
    }
}
