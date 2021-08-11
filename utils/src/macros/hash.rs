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

/// simply create mod $name for `hashchains`
///```no_run
/// hashchains! {
///     pub $name
///     algo $Sha3_256
///     output [u8; $32]
/// }
///```
#[macro_export]
macro_rules! hashchains {
    (
        $vis:vis $name:ident$(,)?
        algo $algo:ty,
        output [u8; $output:expr]$(,)?
    ) => {
            $vis mod $name {
                use super::*;

                pub struct HashChain {
                    hash: $algo,
                    latest_output: [u8; $output],
                }

                impl HashChain {
                    pub fn new() -> Self {
                        Self::default()
                    }
                    pub fn reset(&mut self, initial: &[u8; $output]) {
                        self.latest_output.copy_from_slice(initial);
                    }
                    pub fn hash_chain(&mut self, payload: &[u8]) {
                        let mut mix = self.latest_output.to_vec();
                        mix.extend_from_slice(payload);

                        self.hash.update(mix);
                        self.latest_output.copy_from_slice(&self.hash.finalize_reset());
                    }
                    pub fn output(&self) -> [u8; $output] {
                        self.latest_output
                    }
                }

                impl Default for HashChain {
                    fn default() -> Self {
                        Self {
                            hash: <$algo>::new(),
                            latest_output: [0_u8; $output],
                        }
                    }
                }

                pub struct HashCoverIter {
                    algo: $algo,
                    buf: [u8; $output],
                }
                impl HashCoverIter {
                    pub fn new(original_src: &[u8]) -> Self {
                        let mut algo = <$algo>::new();
                        let buf = Self::squeeze(&mut algo, original_src);
                        Self { algo, buf }
                    }
                    fn squeeze(algo: &mut $algo, src: &[u8]) -> [u8; $output] {
                        algo.update(src);
                        let mut output = [0_u8; $output];
                        output.copy_from_slice(&algo.finalize_reset());
                        output
                    }
                }
                impl Iterator for HashCoverIter {
                    type Item = [u8; $output];
                    fn next(&mut self) -> Option<Self::Item> {
                        self.buf = Self::squeeze(&mut self.algo, &self.buf);
                        Some(self.buf)
                    }
                }

            }
    };
}

pub use hashchains;
