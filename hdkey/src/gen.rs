/*
    .. + gen.rs + ..

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

pub use bip39::{Language, Mnemonic, Seed};

use blake3::{keyed_hash, Hash};
use rand::{thread_rng, Rng};
use sha3::{Digest, Sha3_256 as sha256};
use std::{error::Error, time::Instant};

const ENTROPY1_SIZE: usize = 32;
const ORIGINAL_ENTROPY_SIZE: usize = 32;

#[derive(Debug, PartialEq, Clone)]
pub struct Ident {
    entropy: Hash,
    password: String,
    language: Language,
}
impl Ident {
    pub fn new(words: &str, language: Language) -> Self {
        let password = get_entropy_password(words);
        Self {
            entropy: get_entropy256_from_computer(),
            password,
            language,
        }
    }
    pub fn from(words: &str, language: Language, phrase: &str) -> Result<Self, Box<dyn Error>> {
        let mut entropy = [0u8; ORIGINAL_ENTROPY_SIZE];
        let password = get_entropy_password(words);
        entropy.copy_from_slice(Mnemonic::from_phrase(phrase, language)?.entropy());
        Ok(Self {
            entropy: blake3::Hash::from(entropy),
            password,
            language,
        })
    }
    pub fn into_seed(self) -> Result<(Mnemonic, Seed), Box<dyn Error>> {
        let mnemonic = Mnemonic::from_entropy(self.entropy.as_bytes(), self.language)?;
        let seed = Seed::new(&mnemonic, &self.password);
        Ok((mnemonic, seed))
    }
}

fn get_entropy_password(words: &str) -> String {
    let mut s = sha256::default();
    let bytes = words.as_bytes();
    s.update(bytes.repeat(bytes[1].into()));
    format!("{:02x}", s.finalize())
}

fn get_entropy256_from_computer() -> Hash {
    let mut rng = thread_rng();
    let start = Instant::now();
    let input = rng.gen::<[u8; ENTROPY1_SIZE]>();
    let duration: f32 = rng.gen_range(0.42..1.142);
    let min_count: usize = rng.gen_range(0..14);
    let mut hash;
    let mut i = 0;
    loop {
        hash = keyed_hash(&rng.gen::<[u8; ENTROPY1_SIZE]>(), &input);
        if i > min_count && start.elapsed().as_secs_f32() > duration {
            break;
        }
        i += 1;
    }
    hash
}
