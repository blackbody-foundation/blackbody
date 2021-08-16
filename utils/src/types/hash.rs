/*
    .. + hash.rs + ..

     Copyrigh, Sha3_512t 2021 Hwakyeom Kim(=just-do-halee)

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

use crate::macros::hash::hashchains;
use sha3::{Digest, Sha3_256, Sha3_512};

hashchains! {
    pub sha512
    algo Sha3_512,
    output [u8; 64]
}

hashchains! {
    pub sha256
    algo Sha3_256,
    output [u8; 32]
}

pub struct Hex<const LENGTH: usize>(pub [u8; LENGTH]);
pub struct HexSlice<'a>(pub &'a [u8]);

impl<'a> std::fmt::Display for HexSlice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

impl<const LENGTH: usize> std::fmt::Display for Hex<LENGTH> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for byte in self.0 {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}
