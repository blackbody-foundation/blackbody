/*
    .. + cheque128.rs + ..

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

#[derive(PartialEq, Debug)]
pub struct Cheque128 {
    cheque: Vec<u128>,
}
impl Cheque128 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn len(&self) -> usize {
        self.cheque.len()
    }
    pub fn decimal_point(&mut self) -> &mut u128 {
        let len = self.len();
        &mut self.cheque[len - 1]
    }
    pub fn ceil(&self) -> usize {
        self.len()
    }
    pub fn floor(&self) -> usize {
        self.len() - 1
    }
    pub fn round(&mut self) -> usize {
        if *self.decimal_point() >= u128::MAX / 2 {
            self.ceil()
        } else {
            self.floor()
        }
    }
    pub fn fulfill(&mut self) {
        *self.decimal_point() = u128::MAX;
        self.cheque.push(0);
    }
    pub fn fulfill_and_fill(&mut self, number: u128) {
        self.fulfill();
        self.add(number);
    }
    pub fn add(&mut self, decimal: u128) {
        let left_space = u128::MAX - *self.decimal_point();
        if left_space < decimal {
            self.fulfill_and_fill(decimal - left_space);
        } else {
            *self.decimal_point() += decimal;
        }
    }
    pub fn get(&self) -> (usize, u128) {
        let len = self.len();
        (len - 1, self.cheque[len])
    }
}

impl Default for Cheque128 {
    fn default() -> Self {
        Self {
            cheque: Vec::<u128>::new(),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<(usize, u128)> for Cheque128 {
    fn into(self) -> (usize, u128) {
        self.get()
    }
}
