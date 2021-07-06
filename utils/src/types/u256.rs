/*
    .. + u256.rs + ..

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

pub struct U256 {
    buffer: [u8; 32],
}
impl U256 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn as_u8(&self) -> &[u8; 32] {
        &self.buffer
    }
    pub fn as_mut_u8(&mut self) -> &mut [u8; 32] {
        &mut self.buffer
    }
    pub fn as_u8_slice(&self, len: usize) -> &[u8] {
        &self.buffer[..len]
    }
}
impl Default for U256 {
    fn default() -> Self {
        Self { buffer: [0; 32] }
    }
}
