/*
    .. + keypair.rs + ..

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

use super::{ed25519_dalek::Signature, errors::*, ExtendedKeypair, PublicKey, Version};

use core::fmt;
use std::str::FromStr;

use sha3::{Digest, Sha3_512};

#[cfg(feature = "security")]
use rand::Rng;

#[cfg(feature = "security")]
pub struct WrappedKeypair {
    buf: Vec<Vec<u8>>,
}

#[cfg(feature = "security")]
impl WrappedKeypair {
    pub fn new(keypair: Keypair) -> Self {
        let bytes = keypair.into_bytes();
        let parts = rand::thread_rng().gen_range(1..=4) * 2;
        let part_size = bytes.len() / parts;
        let mut buf = Vec::with_capacity(8);
        for chunk in bytes.chunks(part_size) {
            buf.push(chunk.to_vec());
        }
        Self { buf }
    }
    pub fn into_keypair(self) -> Result<Keypair> {
        Keypair::from_bytes(self.buf.concat().as_slice())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Keypair(ExtendedKeypair);

impl Keypair {
    #[inline]
    pub fn drive_random_child(&self) -> Result<Keypair> {
        let pair = self.0.derive_child(rand::random::<u32>())?;
        Ok(Self(pair))
    }
    #[inline]
    pub fn new(seed: &[u8], prefix: Version) -> Result<Self> {
        let pair = ExtendedKeypair::from_seed(seed, prefix.into_prefix())?;
        Ok(Self(pair))
    }
    #[inline]
    pub fn sign<T: AsRef<[u8]>>(&self, msg: T, memo: Option<&[u8]>) -> Result<Signature> {
        self.0
            .pair()
            .sign_prehashed(prehash512(msg.as_ref()), memo)
            .map_err(Error::msg)
    }
    #[inline]
    pub fn verify(&self, msg: &[u8], memo: Option<&[u8]>, sig: &Signature) -> Result<()> {
        self.public()
            .verify_prehashed(prehash512(msg), memo, sig)
            .map_err(Error::msg)
    }
    #[inline]
    pub fn public(&self) -> PublicKey {
        self.0.pair().public
    }
    #[inline]
    pub fn into_bytes(self) -> [u8; ExtendedKeypair::LENGTH] {
        self.0.into_bytes()
    }
    #[inline]
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(Self(ExtendedKeypair::from_bytes(bytes)?))
    }
    #[inline]
    pub fn from_base58check(base58check: &str) -> Result<Self> {
        Ok(Self(ExtendedKeypair::from_str(base58check)?))
    }
}

#[inline]
fn prehash512(msg: &[u8]) -> Sha3_512 {
    let mut sha = Sha3_512::new();
    sha.update(msg);
    sha
}

impl fmt::Debug for Keypair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n{}\n", self.0)
    }
}

impl fmt::Display for Keypair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
