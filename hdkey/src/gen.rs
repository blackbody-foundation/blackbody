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

use blake3::{hash, keyed_hash, Hash};
use rand::{thread_rng, Rng};
use sha3::{Digest, Sha3_256 as sha256};
use std::{error::Error, path::Path, time::Instant};

const SYSTEM_ENTROPY_SIZE: usize = 32;
const OUTPUT_ENTROPY_SIZE: usize = 32;

use super::*;

pub fn new_master_key<T: AsRef<Path>>(
    version: Version,
    words: &str,
    salt: usize,
    lang: Language,
    login_password: &str,
    target_directories: &[T],
) -> Result<Keypair, Box<dyn Error>> {
    let (phrase, seed) = new_seed(words, lang)?;
    shield::thrust_mnemonic_phrase(&phrase, target_directories, login_password, salt)?;
    Keypair::new(&seed, version)
}

pub fn master_key_from_directories<T: AsRef<Path>>(
    version: Version,
    words: &str,
    salt: usize,
    lang: Language,
    login_password: &str,
    target_directories: &[T],
) -> Result<Keypair, Box<dyn Error>> {
    let phrase = shield::extract_mnemonic_phrase(target_directories, login_password, salt)?;
    let seed = seed_from_phrase(words, lang, &phrase)?;
    Keypair::new(&seed, version)
}

pub fn new_seed(words: &str, lang: Language) -> Result<(String, Vec<u8>), Box<dyn Error>> {
    validate_words(words)?;
    let password = get_entropy256_from_password(words);
    let entropy = get_entropy256_from_computer(hash(words.as_bytes()).as_bytes()[0]);
    let mnemonic = Mnemonic::from_entropy(entropy.as_bytes(), lang)?;
    let seed = Seed::new(&mnemonic, &password);
    Ok((mnemonic.into_phrase(), seed.as_bytes().to_vec()))
}

pub fn seed_from_phrase(
    words: &str,
    lang: Language,
    phrase: &str,
) -> Result<Vec<u8>, Box<dyn Error>> {
    validate_words(words)?;
    let password = get_entropy256_from_password(words);
    let mut buf = [0u8; OUTPUT_ENTROPY_SIZE];
    buf.copy_from_slice(Mnemonic::from_phrase(phrase, lang)?.entropy());
    let entropy = Hash::from(buf);
    let mnemonic = Mnemonic::from_entropy(entropy.as_bytes(), lang)?;
    let seed = Seed::new(&mnemonic, &password);
    Ok(seed.as_bytes().to_vec())
}

#[inline]
fn validate_words(words: &str) -> Result<(), Box<dyn Error>> {
    if words.len() < 8 {
        return Err(format!(
            "password must be more than 8 length bytes. you are {}",
            words.len()
        )
        .into());
    }
    Ok(())
}

fn get_entropy256_from_password(words: &str) -> String {
    if words.is_empty() {
        panic!("words are empty");
    }
    let bytes = words.as_bytes();
    let bytes_len = bytes.len();
    let bytes_avg = bytes
        .iter()
        .fold(0, move |acc, &x| (acc as usize + x as usize) / bytes_len) as u8;
    let bytes_hash = hash(bytes); // blake3
    let nonce = 255usize + bytes_avg as usize + bytes_hash.as_bytes()[0] as usize;
    let mut sha = sha256::new();
    sha.update(bytes); // sha3
    let mut hash = sha.finalize_reset();
    for _ in 0..nonce {
        sha.update(hash);
        sha.update(&[bytes_hash.as_bytes()[1]]); // salt by blake3
        hash = sha.finalize_reset();
    }
    format!("{:02x}", hash) // into hex
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
