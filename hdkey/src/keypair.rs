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
use ed25519_dalek::{
    ed25519, Digest, Keypair as dalekKeypair, PublicKey, SecretKey, SignatureError, KEYPAIR_LENGTH,
    SECRET_KEY_LENGTH,
};
use ed25519_dalek_bip32::ExtendedSecretKey;
use sha3::Sha3_512;
use std::fmt;

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
        Keypair::from_bytes(self.keypair.concat(), self.version)
    }
}

pub struct Keypair {
    pair: dalekKeypair,
    pub version: Version,
}

impl Keypair {
    pub fn new(seed: &[u8], version: Version) -> Result<Self> {
        if seed.len() != KEYPAIR_LENGTH {
            /* = 32 * 2,  [8 * (32 * 2) = 512 bits] */
            return errbang!(
                err::ValidationFailed,
                "seed size must be {}, you are {}",
                KEYPAIR_LENGTH,
                seed.len()
            );
        }
        // let secret = SecretKey::from_bytes(&seed[..SECRET_KEY_LENGTH])?; // L 256 bits
        let xprv = ExtendedSecretKey::from_seed(&seed[..SECRET_KEY_LENGTH]).map_err(Error::msg)?;
        let public = xprv.public_key();
        let secret = xprv.secret_key;

        Ok(Self {
            pair: dalekKeypair { secret, public },
            version,
        })
    }
    pub fn sign<T: AsRef<[u8]>>(
        &self,
        msg: T,
        memo: Option<&[u8]>,
    ) -> std::result::Result<ed25519::Signature, SignatureError> {
        self.pair.sign_prehashed(prehash512(msg.as_ref()), memo)
    }
    pub fn public(&self) -> WrappedKey {
        WrappedKey::Public(self.pair.public, self.version)
    }
    pub fn from_secret_key(wrapped_secret: WrappedKey) -> Result<Self> {
        if let WrappedKey::Secret(secret, version) = wrapped_secret {
            let public = PublicKey::from(&secret);
            Ok(Self {
                pair: dalekKeypair { secret, public },
                version,
            })
        } else {
            errbang!(err::ValidationFailed, "this is not a secret key")
        }
    }
    pub fn into_bytes(self) -> [u8; 64] {
        self.pair.to_bytes()
    }
    pub fn from_bytes(bytes: Vec<u8>, version: Version) -> Result<Self> {
        Ok(Self {
            pair: dalekKeypair::from_bytes(bytes.as_slice())?,
            version,
        })
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
    pub fn verify(
        &self,
        msg: &[u8],
        memo: Option<&[u8]>,
        sig: &ed25519_dalek::Signature,
    ) -> Result<()> {
        match self {
            Self::Public(public, _) => Ok(public.verify_prehashed(prehash512(msg), memo, sig)?),
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
                SecretKey::from_bytes(bytes.as_slice())?,
                version,
            )),
            KeyType::Public => Ok(WrappedKey::Public(
                PublicKey::from_bytes(bytes.as_slice())?,
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
        (self.pair.public == other.pair.public)
            && (self.pair.secret.as_bytes() == other.pair.secret.as_bytes())
    }
}
impl Eq for Keypair {}
