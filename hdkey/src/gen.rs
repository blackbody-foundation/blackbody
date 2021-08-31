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

pub use bip39::Language;
use bip39::{Mnemonic, Seed};

use blake3::{hash, keyed_hash, Hash, Hasher};
use rand::{thread_rng, Rng};
use sha3::{Digest, Sha3_256};
use std::{path::Path, time::Instant};
use vep::Vep;
use zeroize::Zeroize;

const SYSTEM_ENTROPY_SIZE: usize = 32;
const OUTPUT_ENTROPY_SIZE: usize = 32;

use super::*;
use crate::{errors::*, Password};

#[inline(always)]
fn mix_passwords(words: &Password, salt: &usize, login_password: &Password) -> String {
    format!("{}{}{}", words, salt, login_password)
}

pub fn new_master_key<T: AsRef<Path>>(
    version: Version,
    words: Password,
    salt: usize,
    lang: Language,
    login_password: Password,
    target_directories: &[T],
) -> Result<(Keypair, String)> {
    let mut mixed_password = mix_passwords(&words, &salt, &login_password);
    let (phrase, seed) = new_seed(words, lang)?;
    shield::thrust_mnemonic_phrase(&phrase, target_directories, &mixed_password, salt)?;
    mixed_password.zeroize();
    Ok((Keypair::new(&seed, version)?, phrase))
}

pub fn master_key_from_directories<T: AsRef<Path>>(
    version: Version,
    words: Password,
    salt: usize,
    lang: Language,
    login_password: Password,
    target_directories: &[T],
) -> Result<(Keypair, String)> {
    let mut mixed_password = mix_passwords(&words, &salt, &login_password);
    let phrase = shield::extract_mnemonic_phrase(target_directories, &mixed_password, salt)?;
    let seed = seed_from_phrase(words, lang, phrase.as_str())
        .map_err(|_| Error::msg("maybe another key you have"))?;
    mixed_password.zeroize();
    Ok((Keypair::new(&seed, version)?, phrase))
}

pub fn remove_master_key<T: AsRef<Path>>(
    words: Password,
    salt: usize,
    login_password: Password,
    target_directories: &[T],
) -> Result<()> {
    let mut mixed_password = mix_passwords(&words, &salt, &login_password);
    let res = shield::delete_key_file(target_directories, &mixed_password, salt);
    mixed_password.zeroize();
    res
}

pub fn new_seed(words: Password, lang: Language) -> Result<(String, Vec<u8>)> {
    let entropy = get_entropy256_from_computer(hash(words.as_bytes()).as_bytes()[0]);
    let mut password = get_entropy256_from_password(words);
    let mnemonic = Mnemonic::from_entropy(entropy.as_bytes(), lang)?;
    let seed = Seed::new(&mnemonic, &password).as_bytes().to_vec();
    password.zeroize();
    Ok((mnemonic.into_phrase(), seed))
}

pub fn seed_from_phrase(words: Password, lang: Language, phrase: &str) -> Result<Vec<u8>> {
    let mut password = get_entropy256_from_password(words);
    let mut buf = [0u8; OUTPUT_ENTROPY_SIZE];
    buf.copy_from_slice(Mnemonic::from_phrase(phrase, lang)?.entropy());
    let entropy = Hash::from(buf);
    let mnemonic = Mnemonic::from_entropy(entropy.as_bytes(), lang)?;
    let seed = Seed::new(&mnemonic, &password).as_bytes().to_vec();
    password.zeroize();
    Ok(seed)
}

struct VepHasher(blake3::Hasher); // for expanding a password

impl vep::Digester for VepHasher {
    fn digest(&mut self, bytes: &[u8]) -> Vec<u8> {
        self.0.reset();
        self.0.update(bytes);
        self.0.finalize().as_bytes().to_vec()
    }
}

fn get_entropy256_from_password(words: Password) -> String {
    let mut bytes = Vep(VepHasher(Hasher::new())).expand(words.as_bytes()); // blake3
    let mut sha3 = Sha3_256::new(); // sha3
    sha3.update(&bytes);
    bytes.zeroize();
    format!("{:02x}", sha3.finalize_reset()) // into hex
}

fn get_entropy256_from_computer(salt: u8) -> Hash {
    let mut rng = thread_rng();
    let start = Instant::now();
    let mut input = rng.gen::<[u8; SYSTEM_ENTROPY_SIZE]>();
    let duration: f32 = rng.gen_range(0.42..1.142);
    let min_count: usize = rng.gen_range(0..14);
    let mut hash;
    let mut i = 0;
    let input_len = input.len();
    input.rotate_left(absolute_rem(salt as usize, input_len));
    loop {
        hash = keyed_hash(&rng.gen::<[u8; SYSTEM_ENTROPY_SIZE]>(), &input);
        if i > min_count && start.elapsed().as_secs_f32() > duration {
            break;
        }
        i += 1;
    }
    hash
}

#[inline]
fn absolute_rem(a: usize, b: usize) -> usize {
    if a > b {
        if b == 0 {
            return a;
        }
        a % b
    } else {
        if a == 0 {
            return b;
        }
        b % a
    }
}
