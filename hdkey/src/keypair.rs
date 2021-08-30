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

use super::version::{self, KeyType, Version};
use crate::errors::*;

use core::{convert::TryInto, fmt};

use ed25519_dalek_bip32_black::{
    ed25519_dalek::{ed25519::Signature, Digest},
    ChildIndex, DalekKeypair, ExtendedSecretKey, PublicKey, SecretKey, EXTENDED_KEY_LENGTH,
};
use sha3::Sha3_512;

const SEED_LENGTH: usize = 64;

#[cfg(feature = "security")]
use rand::Rng;

#[cfg(feature = "security")]
pub struct WrappedKeypair {
    keypair: Vec<Vec<u8>>,
    version: Version,
}

#[cfg(feature = "security")]
impl WrappedKeypair {
    pub fn new(keypair: Keypair) -> Self {
        let version = keypair.version;
        let bytes = keypair.into_bytes();
        let parts = rand::thread_rng().gen_range(1..=4) * 2;
        let part_size = bytes.len() / parts;
        let mut buf = Vec::with_capacity(8);
        for chunk in bytes.chunks(part_size) {
            buf.push(chunk.to_vec());
        }
        Self {
            keypair: buf,
            version,
        }
    }
    pub fn into_keypair(self) -> Result<Keypair> {
        Keypair::from_bytes(self.keypair.concat().as_slice(), self.version)
    }
}

pub struct Keypair {
    xprv: ExtendedSecretKey,
    pair: DalekKeypair,
    pub version: Version,
}

impl Keypair {
    pub fn drive_random_child(&self) -> Result<Keypair> {
        let xprv = self
            .xprv
            .derive_child(ChildIndex::Hardened(rand::random::<u32>()))?;
        Ok(Keypair::new_from_xprv(xprv, self.version))
    }
    pub fn new_from_xprv(xprv: ExtendedSecretKey, version: Version) -> Self {
        Self {
            pair: xprv.keypair(),
            xprv,
            version,
        }
    }
    pub fn new(seed: &[u8], version: Version) -> Result<Self> {
        if seed.len() != SEED_LENGTH {
            return errbang!(
                err::ValidationFailed,
                "seed size must be {}, you are {}",
                SEED_LENGTH,
                seed.len()
            );
        }
        let xprv = ExtendedSecretKey::from_seed(seed)?;
        Ok(Self::new_from_xprv(xprv, version))
    }
    pub fn sign<T: AsRef<[u8]>>(&self, msg: T, memo: Option<&[u8]>) -> Result<Signature> {
        self.pair
            .sign_prehashed(prehash512(msg.as_ref()), memo)
            .map_err(Error::msg)
    }
    pub fn public(&self) -> WrappedKey {
        WrappedKey::Public(self.pair.public, self.version)
    }
    pub fn into_bytes(self) -> [u8; EXTENDED_KEY_LENGTH] {
        self.xprv.into_bytes()
    }
    pub fn from_bytes(bytes: &[u8], version: Version) -> Result<Self> {
        Ok(Self::new_from_xprv(
            ExtendedSecretKey::from_bytes(bytes.try_into()?)?,
            version,
        ))
    }
}

fn prehash512(msg: &[u8]) -> Sha3_512 {
    let mut sha = Sha3_512::new();
    sha.update(msg);
    sha
}

pub enum WrappedKey {
    Public(PublicKey, Version),
    Secret(SecretKey, Version),
}

impl WrappedKey {
    pub fn verify(&self, msg: &[u8], memo: Option<&[u8]>, sig: &Signature) -> Result<()> {
        match self {
            Self::Public(public, _) => Ok(public
                .verify_prehashed(prehash512(msg), memo, sig)
                .map_err(Error::msg)?),
            _ => errbang!(err::ValidationFailed, "this is not a public key."),
        }
    }
    pub fn as_hex(&self) -> String {
        match self {
            Self::Secret(key, _) => hex::encode(key),
            Self::Public(key, _) => hex::encode(key),
        }
    }
    pub fn as_base58check(&self) -> String {
        match self {
            Self::Secret(key, version) => {
                bs58::encode(version::encode(key, *version, KeyType::Secret).as_slice())
                    .into_string()
            }
            Self::Public(key, version) => {
                bs58::encode(version::encode(key, *version, KeyType::Public).as_slice())
                    .into_string()
            }
        }
    }
    pub fn from_base58check(
        base58check: &str,
        version: Version,
        key_type: KeyType,
    ) -> Result<Self> {
        let bytes = version::decode(bs58::decode(base58check).into_vec()?, version, key_type)?;
        match key_type {
            KeyType::Secret => Ok(WrappedKey::Secret(
                errcast!(
                    SecretKey::from_bytes(bytes.as_slice()),
                    err::ValidationFailed,
                    "cannot parse the secret bytes."
                ),
                version,
            )),
            KeyType::Public => Ok(WrappedKey::Public(
                errcast!(
                    PublicKey::from_bytes(bytes.as_slice()),
                    err::ValidationFailed,
                    "cannot parse the public bytes."
                ),
                version,
            )),
        }
    }
}
impl fmt::Debug for Keypair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        // *** Test Secret Key ***
        // let sk = Key::Secret(
        //     SecretKey::from_bytes(self.pair.secret.as_bytes()).unwrap(),
        //     self.version,
        // );
        // let sk_hex = sk.as_hex();
        // let sk_bs58 = sk.as_base58check();
        // write!(
        //     f,
        //     "hex (\nsecret: {}\npublic: {}\n)\nbase58check[{}] (\nsecret: {}\npublic: {}\n)\n",
        //     sk_hex,
        //     pk_hex,
        //     self.version.to_string(),
        //     sk_bs58,
        //     pk_bs58
        // )
        let pk_hex = self.public().as_hex();
        let pk_bs58 = self.public().as_base58check();
        write!(
            f,
            "\n- hex -\npublic: {}\n\n- base58check [{}] -\npublic: {}\n",
            pk_hex,
            self.version.to_string(),
            pk_bs58,
        )
    }
}
impl PartialEq for Keypair {
    fn eq(&self, other: &Self) -> bool {
        (self.version == other.version)
            && (self.xprv == other.xprv)
            && (self.pair.public == other.pair.public)
            && (self.pair.secret.as_bytes() == other.pair.secret.as_bytes())
    }
}
impl Eq for Keypair {}
