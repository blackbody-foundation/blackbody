/*
    .. + hash.rs + ..

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

use sha3::{Digest, Sha3_256};

pub struct HashChain256 {
    hash: Sha3_256,
    latest_output: [u8; 32],
}

impl HashChain256 {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn reset(&mut self, initial: &[u8; 32]) {
        self.latest_output.copy_from_slice(initial);
    }
    pub fn hash_chain(&mut self, payload: &[u8]) {
        let mut mix = self.latest_output.to_vec();
        mix.extend_from_slice(payload);

        self.hash.update(mix);
        self.latest_output = self.hash.finalize_reset().into();
    }
    pub fn output(&self) -> [u8; 32] {
        self.latest_output
    }
}

impl Default for HashChain256 {
    fn default() -> Self {
        Self {
            hash: Sha3_256::new(),
            latest_output: [0_u8; 32],
        }
    }
}

pub struct Hex<const LENGTH: usize>(pub [u8; LENGTH]);

impl<const LENGTH: usize> std::fmt::Display for Hex<LENGTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}
